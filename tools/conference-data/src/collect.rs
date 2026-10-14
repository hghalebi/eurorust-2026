use crate::{Error, Result, domain::*, invalid};
use reqwest::{blocking::Client, header};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path, thread, time::Duration};

pub const ORIGIN: &str = "https://eurorust.eu";
pub const SCHEDULE: &str = "https://eurorust.eu/schedule/";
pub fn digest(bytes: &[u8]) -> ContentHash {
    ContentHash::from_content(bytes)
}
pub fn selector(value: &str) -> Selector {
    Selector::parse(value).expect("static CSS selector")
}
pub fn text(element: scraper::ElementRef<'_>) -> String {
    element
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
pub fn slug(value: &str) -> String {
    value
        .to_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[derive(Serialize, Deserialize)]
struct CachedPage {
    body: String,
    etag: Option<String>,
    modified: Option<String>,
    source: Provenance,
}

struct SourceClient {
    client: Client,
    last_request: Option<std::time::Instant>,
}
impl SourceClient {
    fn new() -> Result<Self> {
        Ok(Self {
            client: Client::builder()
                .user_agent("EuroRustFieldNotes/0.1 (+https://github.com/hghalebi/eurorust-2026)")
                .https_only(true)
                .redirect(reqwest::redirect::Policy::none())
                .timeout(Duration::from_secs(30))
                .build()?,
            last_request: None,
        })
    }
    fn get(&mut self, url: &SourceUrl, cache: &Path) -> Result<CachedPage> {
        let parsed =
            url::Url::parse(url.as_str()).map_err(|e| invalid("source URL", e.to_string()))?;
        if parsed.origin().ascii_serialization() != ORIGIN {
            return Err(invalid(
                "source URL",
                "collector only fetches the official origin",
            ));
        }
        fs::create_dir_all(cache)?;
        let path = cache.join(format!("{}.json", digest(url.as_str().as_bytes())));
        let previous = if path.exists() {
            Some(serde_json::from_slice::<CachedPage>(&fs::read(&path)?)?)
        } else {
            None
        };
        for attempt in 0..3 {
            if let Some(last) = self.last_request {
                thread::sleep(Duration::from_secs(1).saturating_sub(last.elapsed()));
            }
            let mut request = self.client.get(url.as_str());
            if let Some(old) = &previous {
                if let Some(etag) = &old.etag {
                    request = request.header(header::IF_NONE_MATCH, etag);
                }
                if let Some(modified) = &old.modified {
                    request = request.header(header::IF_MODIFIED_SINCE, modified);
                }
            }
            self.last_request = Some(std::time::Instant::now());
            let response = match request.send() {
                Ok(response) => response,
                Err(error) if attempt < 2 && (error.is_timeout() || error.is_connect()) => {
                    thread::sleep(Duration::from_secs(1 << attempt));
                    continue;
                }
                Err(error) => return Err(error.into()),
            };
            if response.status().as_u16() == 304 {
                let mut old =
                    previous.ok_or_else(|| Error::Layout("304 without cached source".into()))?;
                old.source.retrieved_at = Timestamp::now();
                fs::write(&path, serde_json::to_vec(&old)?)?;
                return Ok(old);
            }
            if response.status().as_u16() == 429 || response.status().is_server_error() {
                let delay = response
                    .headers()
                    .get(header::RETRY_AFTER)
                    .and_then(|s| s.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(1 << attempt);
                if attempt < 2 && delay <= 30 {
                    thread::sleep(Duration::from_secs(delay));
                    continue;
                }
            }
            let response = response.error_for_status()?;
            if response.status().is_redirection() {
                return Err(Error::Layout("source redirect requires review".into()));
            }
            let etag = response
                .headers()
                .get(header::ETAG)
                .and_then(|s| s.to_str().ok())
                .map(str::to_owned);
            let modified = response
                .headers()
                .get(header::LAST_MODIFIED)
                .and_then(|s| s.to_str().ok())
                .map(str::to_owned);
            let source_updated_at = modified
                .as_deref()
                .and_then(|s| chrono::DateTime::parse_from_rfc2822(s).ok())
                .map(|t| Timestamp::try_from(t.to_rfc3339()))
                .transpose()?;
            let body = response.text()?;
            let source = Provenance {
                source_url: url.clone(),
                retrieved_at: Timestamp::now(),
                source_updated_at,
                content_sha256: digest(body.as_bytes()),
            };
            let page = CachedPage {
                body,
                etag,
                modified,
                source,
            };
            fs::write(&path, serde_json::to_vec(&page)?)?;
            return Ok(page);
        }
        Err(Error::Layout("request retry budget exhausted".into()))
    }
}

/// Select the most specific applicable robots user-agent group and longest path rule.
pub fn robots_allow(body: &str, path: &str) -> bool {
    type RobotsGroup = (Vec<String>, Vec<(bool, String)>);
    let mut groups: Vec<RobotsGroup> = Vec::new();
    let mut agents = Vec::new();
    let mut rules = Vec::new();
    let mut has_rules = false;
    for line in body.lines().chain(std::iter::once("User-agent: __end__")) {
        let clean = line.split('#').next().unwrap_or("").trim();
        let Some((key, value)) = clean.split_once(':') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();
        if key == "user-agent" {
            if has_rules {
                groups.push((std::mem::take(&mut agents), std::mem::take(&mut rules)));
                has_rules = false;
            }
            agents.push(value.to_ascii_lowercase());
        } else if (key == "allow" || key == "disallow") && !agents.is_empty() {
            has_rules = true;
            if !value.is_empty() {
                rules.push((key == "allow", value.to_owned()));
            }
        }
    }
    let specificity = |agents: &[String]| {
        agents
            .iter()
            .filter_map(|a| {
                if a == "*" {
                    Some(0)
                } else if "eurorustfieldnotes".contains(a) {
                    Some(a.len())
                } else {
                    None
                }
            })
            .max()
    };
    let best = groups.iter().filter_map(|(a, _)| specificity(a)).max();
    let mut matching = Vec::new();
    for (agents, rules) in &groups {
        if specificity(agents) != best || best.is_none() {
            continue;
        }
        for (allow, rule) in rules {
            let end = rule.ends_with('$');
            let pattern = rule
                .trim_end_matches('$')
                .split('*')
                .map(regex::escape)
                .collect::<Vec<_>>()
                .join(".*");
            if regex::Regex::new(&format!("^{pattern}{}", if end { "$" } else { "" }))
                .expect("escaped rule")
                .is_match(path)
            {
                matching.push((
                    rule.chars().filter(|c| *c != '*' && *c != '$').count(),
                    *allow,
                ));
            }
        }
    }
    matching.into_iter().max().is_none_or(|(_, allow)| allow)
}

pub fn parse_schedule(body: &str, source: Provenance) -> Result<DraftSnapshot> {
    let html = Html::parse_document(body);
    let metadata = html
        .select(&selector("script[type='application/ld+json']"))
        .next()
        .ok_or_else(|| Error::Layout("conference JSON-LD missing".into()))?;
    let metadata: serde_json::Value = serde_json::from_str(&metadata.inner_html())?;
    let event = metadata
        .get("@graph")
        .and_then(|v| v.as_array())
        .and_then(|nodes| {
            nodes
                .iter()
                .find(|node| node.get("@type").and_then(|v| v.as_str()) == Some("Event"))
        })
        .ok_or_else(|| Error::Layout("conference event metadata missing".into()))?;
    let required = |value: &serde_json::Value, key: &str| -> Result<String> {
        value
            .get(key)
            .and_then(|v| v.as_str())
            .map(str::to_owned)
            .ok_or_else(|| Error::Layout(format!("conference metadata missing {key}")))
    };
    let conference_name = required(event, "name")?;
    if conference_name != "EuroRust 2026" {
        return Err(Error::Layout(
            "source now describes a different conference edition; update the adapter explicitly"
                .into(),
        ));
    }
    let location = event
        .get("location")
        .ok_or_else(|| Error::Layout("conference venue missing".into()))?;
    let address = location
        .get("address")
        .ok_or_else(|| Error::Layout("conference address missing".into()))?;
    let mut sessions = Vec::new();
    let mut speakers = BTreeMap::new();
    for item in html.select(&selector("[data-schedule-entry]")) {
        let attr = |name: &str| {
            item.value()
                .attr(name)
                .ok_or_else(|| Error::Layout(format!("missing {name}")))
        };
        let start = Timestamp::try_from(attr("data-start")?.to_owned())?;
        let end = item
            .value()
            .attr("data-end")
            .map(|s| s.to_owned().try_into())
            .transpose()?;
        let kind = match attr("data-filter-type")? {
            "workshop" => SessionKind::Workshop,
            "main-stage-talk" | "side-stage-talk" => SessionKind::Talk,
            "side-activity" => SessionKind::SideActivity,
            "service" => SessionKind::Service,
            other => return Err(Error::Layout(format!("unknown session kind {other}"))),
        };
        let title = item
            .select(&selector(".activity-card__title"))
            .next()
            .map(text)
            .ok_or_else(|| Error::Layout("session title missing".into()))?;
        let href = item
            .select(&selector("a.activity-card__link"))
            .next()
            .and_then(|a| a.value().attr("href"));
        if href.is_none() && kind != SessionKind::Service {
            return Err(Error::Layout("session detail link missing".into()));
        }
        let (id, url) = match href {
            Some(href) if href.starts_with('/') && !href.starts_with("//") => {
                (slug(href), SourceUrl::try_from(format!("{ORIGIN}{href}"))?)
            }
            Some(href) if kind == SessionKind::SideActivity && href.starts_with("https://") => (
                format!(
                    "activities-external-{}",
                    &digest(href.as_bytes()).as_str()[..16]
                ),
                SourceUrl::try_from(href.to_owned())?,
            ),
            Some(_) => return Err(Error::Layout("unexpected external session link".into())),
            None => (
                format!("service-{}-{}", slug(&start.as_rfc3339()), slug(&title)),
                source.source_url.clone(),
            ),
        };
        let mut speaker_ids = Vec::new();
        for name in item.select(&selector(".activity-card__speaker")).map(text) {
            for name in name
                .trim_start_matches('»')
                .trim()
                .trim_start_matches("Led by")
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                let id: SpeakerId = format!(
                    "speaker-{}-{}",
                    slug(name),
                    &digest(name.as_bytes()).as_str()[..8]
                )
                .try_into()?;
                speakers.entry(id.clone()).or_insert(Speaker {
                    id: id.clone(),
                    name: name.to_owned().try_into()?,
                    source_url: url.clone(),
                });
                speaker_ids.push(id);
            }
        }
        let venue_id = match kind {
            SessionKind::Workshop => Some("workshops".to_owned().try_into()?),
            SessionKind::Talk | SessionKind::Service => Some("conference".to_owned().try_into()?),
            _ => None,
        };
        sessions.push(Session {
            id: id.try_into()?,
            title: title.try_into()?,
            official_url: url,
            kind,
            slot: Slot::new(start, end)?,
            stage: item
                .value()
                .attr("data-track-label")
                .map(|s| s.to_owned().try_into())
                .transpose()?,
            speaker_ids,
            venue_id,
            source: source.clone(),
            registration_url: None,
            ticket_requirement: None,
            remote_access: None,
            recording_url: None,
            detail_source: None,
            annotation: None,
        });
    }
    if sessions.is_empty() {
        return Err(Error::Layout("no schedule entries discovered".into()));
    }
    sessions.sort_by(|a, b| {
        a.slot
            .start()
            .cmp(b.slot.start())
            .then_with(|| a.id.cmp(&b.id))
    });
    let catalog = Catalog {
        schema_version: "1.0.0".to_owned().try_into()?,
        conference: Conference {
            name: conference_name.try_into()?,
            timezone: ConferenceTimezone::EuropeMadrid,
            source_url: format!("{ORIGIN}/").try_into()?,
            schedule_url: SCHEDULE.to_owned().try_into()?,
            starts_at: required(event, "startDate")?.try_into()?,
            ends_at: required(event, "endDate")?.try_into()?,
        },
        sessions,
        speakers: speakers.into_values().collect(),
        venues: vec![
            Venue {
                id: "conference".to_owned().try_into()?,
                name: required(location, "name")?.try_into()?,
                address: Some(
                    format!(
                        "{}, {}",
                        required(address, "streetAddress")?,
                        required(address, "addressLocality")?
                    )
                    .try_into()?,
                ),
                source_url: format!("{ORIGIN}/").try_into()?,
            },
            Venue {
                id: "workshops".to_owned().try_into()?,
                name: "WeWork Glories Barcelona".to_owned().try_into()?,
                address: Some("Carrer Tànger 86, Barcelona".to_owned().try_into()?),
                source_url: format!("{ORIGIN}/").try_into()?,
            },
        ],
    };
    Ok(DraftSnapshot::new(catalog))
}

pub fn refresh(root: &Path, cache: &Path, accept_removals: bool) -> Result<ValidatedSnapshot> {
    let mut client = SourceClient::new()?;
    let robots = client.get(&format!("{ORIGIN}/robots.txt").try_into()?, cache)?;
    if !robots_allow(&robots.body, "/schedule/") {
        return Err(Error::Validation(
            "robots rules disallow schedule fetch".into(),
        ));
    }
    let schedule = client.get(&SCHEDULE.to_owned().try_into()?, cache)?;
    let checked = parse_schedule(&schedule.body, schedule.source)?.validate()?;
    let mut catalog: Catalog = serde_json::from_value(serde_json::to_value(checked.catalog())?)?;
    let annotation_path = root.join("community/annotations/sessions.json");
    let annotations: Vec<Annotation> = if annotation_path.exists() {
        serde_json::from_slice(&fs::read(annotation_path)?)?
    } else {
        Vec::new()
    };
    let mut applied = std::collections::BTreeSet::new();
    for session in &mut catalog.sessions {
        if session.kind != SessionKind::Service
            && session
                .official_url
                .as_str()
                .starts_with(&format!("{ORIGIN}/"))
        {
            let url = &session.official_url;
            let parsed =
                url::Url::parse(url.as_str()).map_err(|e| invalid("source URL", e.to_string()))?;
            if !robots_allow(&robots.body, parsed.path()) {
                return Err(Error::Validation(format!(
                    "robots rules disallow {}",
                    parsed.path()
                )));
            }
            let detail = client.get(url, cache)?;
            if Html::parse_document(&detail.body)
                .select(&selector("h1"))
                .next()
                .is_none()
            {
                return Err(Error::Layout(format!(
                    "detail heading missing: {}",
                    url.as_str()
                )));
            }
            let html = Html::parse_document(&detail.body);
            session.registration_url = html
                .select(&selector("a[href]"))
                .filter_map(|a| a.value().attr("href"))
                .find(|href| href.starts_with("https://www.tickettailor.com/"))
                .map(|s| s.to_owned().try_into())
                .transpose()?;
            session.detail_source = Some(detail.source);
        }
        if let Some(annotation) = annotations.iter().find(|a| a.session_id == session.id) {
            if !applied.insert(annotation.session_id.clone()) {
                return Err(Error::Validation("duplicate annotation".into()));
            }
            let mut annotation = annotation.clone();
            if session
                .detail_source
                .as_ref()
                .is_some_and(|source| source.content_sha256 != annotation.source_content_sha256)
            {
                annotation.review_status = AnnotationReview::SourceChanged;
            }
            session.annotation = Some(annotation);
        }
    }
    if applied.len() != annotations.len() {
        return Err(Error::Validation(
            "annotation has a missing or duplicate session".into(),
        ));
    }
    let validated = DraftSnapshot::new(catalog).validate()?;
    let previous = root.join("data/catalog.json");
    if previous.exists() {
        let previous = load(&previous)?;
        let now: std::collections::BTreeSet<_> =
            validated.catalog().sessions.iter().map(|s| &s.id).collect();
        let removed: Vec<_> = previous
            .catalog()
            .sessions
            .iter()
            .filter(|s| !now.contains(&s.id))
            .map(|s| s.id.as_str())
            .collect();
        if !removed.is_empty() && !accept_removals {
            return Err(Error::Validation(format!(
                "sessions disappeared: {removed:?}; review source and use --accept-removals only for confirmed changes"
            )));
        }
    }
    Ok(validated)
}

pub fn load(path: &Path) -> Result<ValidatedSnapshot> {
    DraftSnapshot::new(serde_json::from_slice(&fs::read(path)?)?).validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn provenance() -> Provenance {
        Provenance {
            source_url: SCHEDULE.to_owned().try_into().unwrap(),
            retrieved_at: Timestamp::now(),
            source_updated_at: None,
            content_sha256: digest(b"fixture"),
        }
    }
    #[test]
    fn fixture_checks_layout_and_unknown_end() {
        let body = include_str!("../tests/fixtures/schedule.html");
        let validated = parse_schedule(body, provenance())
            .unwrap()
            .validate()
            .unwrap();
        assert_eq!(validated.catalog().sessions.len(), 2);
        assert!(
            validated
                .catalog()
                .sessions
                .iter()
                .any(|s| s.slot.end().is_none())
        );
        assert!(
            parse_schedule(&body.replace("data-start", "renamed-start"), provenance()).is_err()
        );
        assert!(parse_schedule("<h1>No schedule</h1>", provenance()).is_err());
        assert!(
            parse_schedule(
                &body.replace(
                    "/activities/fixture-dinner/",
                    "https://example.com/activity"
                ),
                provenance()
            )
            .unwrap()
            .validate()
            .is_ok()
        );
        assert!(
            parse_schedule(
                &body.replace("/talks/fixture-contract/", "https://example.com/talk"),
                provenance()
            )
            .is_err()
        );
        assert!(
            parse_schedule(&body.replace("main-stage-talk", "new-kind"), provenance()).is_err()
        );
        assert!(
            parse_schedule(&format!("{body}{body}"), provenance())
                .unwrap()
                .validate()
                .is_err()
        );
    }
    #[test]
    fn robots_respects_groups_and_longest_rules() {
        assert!(robots_allow(
            "User-agent: *\nDisallow:\nAllow: /",
            "/schedule/"
        ));
        assert!(!robots_allow(
            "User-agent: *\nDisallow: /\nAllow: /public",
            "/schedule/"
        ));
        assert!(robots_allow(
            "User-agent: *\nDisallow: /\nUser-agent: EuroRustFieldNotes\nAllow: /",
            "/schedule/"
        ));
        assert!(!robots_allow(
            "User-agent: *\nDisallow: /*secret$",
            "/a/secret"
        ));
    }
}
