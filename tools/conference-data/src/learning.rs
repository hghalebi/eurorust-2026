//! Offline validation and deterministic views of curated learning resources.
use crate::{Error, Result, domain::*, export, invalid};
use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

macro_rules! text_type {
    ($($name:ident),+ $(,)?) => {$(
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
        #[serde(try_from = "String", into = "String")]
        #[schemars(with = "String")]
        struct $name(String);
        impl TryFrom<String> for $name {
            type Error = Error;
            fn try_from(value: String) -> Result<Self> {
                if value.trim().is_empty() || value.chars().any(char::is_control) {
                    return Err(invalid(stringify!($name), "expected nonempty single-line text"));
                }
                Ok(Self(value))
            }
        }
        impl From<$name> for String { fn from(value: $name) -> Self { value.0 } }
    )+};
}
text_type!(EditionName);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "String", into = "String")]
#[schemars(with = "String")]
struct ResourceId(String);
impl TryFrom<String> for ResourceId {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty()
            || !value
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
        {
            return Err(invalid("ResourceId", "expected a lowercase identifier"));
        }
        Ok(Self(value))
    }
}
impl From<ResourceId> for String {
    fn from(value: ResourceId) -> Self {
        value.0
    }
}

macro_rules! date_type {
    ($($name:ident),+ $(,)?) => {$(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
        #[serde(transparent)]
        struct $name(NaiveDate);
    )+};
}
date_type!(CheckedOn, PublishedOn, EventDay);

macro_rules! labels {
    ($name:ident { $($variant:ident => $label:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
        #[serde(rename_all = "snake_case")]
        enum $name { $($variant),+ }
        impl $name { fn label(self) -> &'static str { match self { $(Self::$variant => $label),+ } } }
    };
}
labels!(ResourceKind { Book => "Book", Article => "Article", Paper => "Paper", Video => "Video", Documentation => "Documentation", Project => "Project", Course => "Course", ConferenceTalk => "Talk", Workshop => "Workshop" });
labels!(Access { Open => "open access", Purchase => "purchase", Registration => "registration", Unknown => "access unverified" });
labels!(Relation { Preparation => "Preparation", SameTopic => "Same topic", AlternativeApproach => "Alternative approach", NextStep => "Suggested next step", ProjectEvolution => "Project evolution", RepeatAppearance => "Repeat appearance" });
labels!(TemporalStatus { Published => "Published", Announced => "Announced", Unknown => "Delivery unverified" });
labels!(Verification { SourceContentChecked => "Source page checked", ListingChecked => "Listing checked" });

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EventContext {
    name: Title,
    edition: EditionName,
    starts_on: Option<EventDay>,
    source_url: SourceUrl,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Resource {
    id: ResourceId,
    title: Title,
    kind: ResourceKind,
    url: SourceUrl,
    authors: Vec<Author>,
    published_on: Option<PublishedOn>,
    access: Access,
    relation: Relation,
    why: Summary,
    evidence_urls: Vec<SourceUrl>,
    checked_on: CheckedOn,
    verification: Verification,
    temporal_status: TemporalStatus,
    event: Option<EventContext>,
    caveat: Option<Summary>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ResourceSet {
    session_id: SessionId,
    checked_on: CheckedOn,
    review_status: AnnotationReview,
    research_note: Summary,
    resources: Vec<Resource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LearningSourceManifest {
    format_version: SchemaVersion,
    catalog_snapshot_id: SnapshotId,
    catalog_content_sha256: Option<ContentHash>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(try_from = "String")]
struct ProfileSlug(String);
impl TryFrom<String> for ProfileSlug {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty()
            || !value
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
        {
            return Err(invalid(
                "ProfileSlug",
                "expected a lowercase profile identifier",
            ));
        }
        Ok(Self(value))
    }
}

// This is a navigation projection of the separate speaker-research format.
// Its biographies, works, and research dates remain owned by that collection.
#[derive(Deserialize)]
struct SpeakerRoutes {
    profiles: Vec<SpeakerRoute>,
}
#[derive(Deserialize)]
struct SpeakerRoute {
    speaker_id: Option<SpeakerId>,
    profile_id: ProfileSlug,
}

fn speaker_routes(root: &Path, catalog: &Catalog) -> Result<BTreeMap<SpeakerId, ProfileSlug>> {
    let source: SpeakerRoutes =
        serde_json::from_slice(&fs::read(root.join("community/speaker-profiles.json"))?)?;
    let mut routes = BTreeMap::new();
    let mut slugs = BTreeSet::new();
    for profile in source.profiles {
        let Some(id) = profile.speaker_id else {
            continue;
        };
        if !slugs.insert(profile.profile_id.clone())
            || !root
                .join(format!("resources/speakers/{}.md", profile.profile_id.0))
                .is_file()
            || routes.insert(id, profile.profile_id).is_some()
        {
            return Err(invalid(
                "speaker navigation",
                "missing profile page, shared profile, or duplicate catalog speaker",
            ));
        }
    }
    if routes.keys().collect::<BTreeSet<_>>()
        != catalog
            .speakers
            .iter()
            .map(|s| &s.id)
            .collect::<BTreeSet<_>>()
    {
        return Err(invalid(
            "speaker navigation",
            "profile routes must match the catalog roster",
        ));
    }
    Ok(routes)
}

// Ignore retrieval timestamps and scheduling changes; include the material used
// to interpret a session so a refresh cannot silently reuse stale topic judgments.
fn context_hash(catalog: &Catalog) -> Result<ContentHash> {
    let mut sessions: Vec<_> = learning_sessions(catalog).collect();
    sessions.sort_by(|a, b| a.id.cmp(&b.id));
    let wire: Vec<_> = sessions.iter().map(|s| serde_json::json!({
        "id":s.id,"title":s.title,"kind":s.kind,"official_url":s.official_url,
        "speaker_ids":s.speaker_ids,
        "detail_hash":s.detail_source.as_ref().map(|d| &d.content_sha256),
        "annotation":s.annotation.as_ref().map(|a| serde_json::json!({"summary":a.summary,"topics":a.topics,"review_status":a.review_status,"source_content_sha256":a.source_content_sha256}))
    })).collect();
    Ok(ContentHash::from_content(&serde_json::to_vec(&wire)?))
}

/// Rendering is available only after catalog and resource validation.
/// ```compile_fail
/// use conference_data::learning::CheckedLearning;
/// let checked = CheckedLearning { sets: vec![] };
/// ```
/// ```no_run
/// use conference_data::learning;
/// let checked = learning::load(std::path::Path::new("."))?;
/// checked.verify(std::path::Path::new("."))?;
/// # Ok::<(), conference_data::Error>(())
/// ```
pub struct CheckedLearning {
    bundle: export::VerifiedBundle,
    sets: Vec<ResourceSet>,
    speaker_routes: BTreeMap<SpeakerId, ProfileSlug>,
}

fn learning_sessions(catalog: &Catalog) -> impl Iterator<Item = &Session> {
    catalog
        .sessions
        .iter()
        .filter(|s| matches!(s.kind, SessionKind::Talk | SessionKind::Workshop))
}

fn validate_sets(sets: &[ResourceSet], catalog: &Catalog) -> Result<()> {
    let expected: BTreeSet<_> = learning_sessions(catalog).map(|s| &s.id).collect();
    let actual: BTreeSet<_> = sets.iter().map(|s| &s.session_id).collect();
    if actual != expected || actual.len() != sets.len() {
        return Err(Error::Validation(
            "learning resources must cover every talk/workshop exactly once".into(),
        ));
    }
    for set in sets {
        if set.resources.is_empty() {
            return Err(invalid("resources", "each session needs a useful resource"));
        }
        let mut ids = BTreeSet::new();
        let mut urls = BTreeSet::new();
        for resource in &set.resources {
            if !ids.insert(&resource.id) || !urls.insert(resource.url.as_str()) {
                return Err(invalid(
                    "resources",
                    format!(
                        "duplicate resource ID or URL in {}",
                        set.session_id.as_str()
                    ),
                ));
            }
            if resource.evidence_urls.is_empty() || resource.authors.is_empty() {
                return Err(invalid(
                    "resource attribution",
                    "author/organization and source evidence required",
                ));
            }
            if resource.checked_on.0 > set.checked_on.0 {
                return Err(invalid(
                    "checked_on",
                    "resource check is newer than its session review",
                ));
            }
            if resource.temporal_status == TemporalStatus::Published
                && resource
                    .published_on
                    .is_some_and(|d| d.0 > resource.checked_on.0)
            {
                return Err(invalid(
                    "published_on",
                    "a future publication cannot be marked published",
                ));
            }
            if resource.temporal_status == TemporalStatus::Published
                && matches!(
                    resource.kind,
                    ResourceKind::ConferenceTalk | ResourceKind::Workshop | ResourceKind::Video
                )
                && resource
                    .event
                    .as_ref()
                    .and_then(|e| e.starts_on)
                    .is_some_and(|d| d.0 > resource.checked_on.0)
            {
                return Err(invalid(
                    "event",
                    "a future session cannot be marked published",
                ));
            }
            if matches!(
                resource.kind,
                ResourceKind::ConferenceTalk | ResourceKind::Workshop
            ) && resource.event.is_none()
            {
                return Err(invalid(
                    "event",
                    "conference talks and workshops need event attribution",
                ));
            }
            if resource.temporal_status == TemporalStatus::Announced {
                let expected_date = resource
                    .event
                    .as_ref()
                    .and_then(|e| e.starts_on.map(|d| d.0))
                    .or(resource.published_on.map(|d| d.0));
                if expected_date.is_none_or(|d| d < resource.checked_on.0) {
                    return Err(invalid(
                        "announcement",
                        "needs a dated event or publication on or after the check date",
                    ));
                }
            }
        }
    }
    Ok(())
}

pub fn load(root: &Path) -> Result<CheckedLearning> {
    let bundle = export::verify(root)?;
    let source_manifest: LearningSourceManifest =
        serde_json::from_slice(&fs::read(root.join("community/learning-manifest.json"))?)?;
    if source_manifest.format_version.as_str() != "1.0.0"
        || source_manifest.catalog_snapshot_id != bundle.manifest().snapshot_id
        || source_manifest.catalog_content_sha256.as_ref() != Some(&context_hash(bundle.catalog())?)
    {
        return Err(invalid(
            "learning manifest",
            "unsupported format or catalog snapshot mismatch; review session joins before updating",
        ));
    }
    let sets = load_sets(root)?;
    validate_sets(&sets, bundle.catalog())?;
    let speaker_routes = speaker_routes(root, bundle.catalog())?;
    Ok(CheckedLearning {
        bundle,
        sets,
        speaker_routes,
    })
}

fn load_sets(root: &Path) -> Result<Vec<ResourceSet>> {
    let mut sets = Vec::new();
    for entry in fs::read_dir(root.join("community/session-resources"))? {
        let path = entry?.path();
        if path.extension().is_none_or(|e| e != "json") {
            continue;
        }
        let set: ResourceSet = serde_json::from_slice(&fs::read(&path)?)?;
        if path.file_stem().and_then(|s| s.to_str()) != Some(set.session_id.as_str()) {
            return Err(invalid("resource path", "filename must match session ID"));
        }
        sets.push(set);
    }
    sets.sort_by(|a, b| a.session_id.cmp(&b.session_id));
    Ok(sets)
}

/// Rebind retrieval-only or schedule-only refreshes without changing research dates.
/// A changed session identity, abstract, speaker, or annotation requires review.
pub fn rebind(root: &Path) -> Result<()> {
    let bundle = export::verify(root)?;
    let path = root.join("community/learning-manifest.json");
    let mut source: LearningSourceManifest = serde_json::from_slice(&fs::read(&path)?)?;
    let current = context_hash(bundle.catalog())?;
    if source.format_version.as_str() != "1.0.0"
        || match &source.catalog_content_sha256 {
            Some(previous) => previous != &current,
            None => source.catalog_snapshot_id != bundle.manifest().snapshot_id,
        }
    {
        return Err(invalid(
            "learning context",
            "session content changed; review resources before replacing the source manifest's snapshot and content fingerprint",
        ));
    }
    validate_sets(&load_sets(root)?, bundle.catalog())?;
    source.catalog_snapshot_id = bundle.manifest().snapshot_id.clone();
    source.catalog_content_sha256 = Some(current);
    let bytes = format!("{}\n", serde_json::to_string_pretty(&source)?);
    let mut temporary = tempfile::NamedTempFile::new_in(
        path.parent()
            .ok_or_else(|| invalid("manifest path", "missing parent"))?,
    )?;
    use std::io::Write;
    temporary.write_all(bytes.as_bytes())?;
    temporary.persist(&path).map_err(|e| Error::Io(e.error))?;
    println!("Learning snapshot rebound after matching session content; research dates preserved");
    Ok(())
}

// Text becomes Markdown only at this output boundary; source text cannot add HTML or links.
fn md(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('`', "\\`")
        .replace('|', "\\|")
}
fn destination(url: &SourceUrl) -> String {
    url.as_str().replace('(', "%28").replace(')', "%29")
}
fn render_resource(resource: &Resource) -> String {
    let authors = resource
        .authors
        .iter()
        .map(|a| md(a.as_str()))
        .collect::<Vec<_>>()
        .join(", ");
    // A public programme or source page does not establish media or event access.
    let access = match (resource.kind, resource.access) {
        (ResourceKind::ConferenceTalk | ResourceKind::Workshop, Access::Open) => {
            "public source page"
        }
        (_, access) => access.label(),
    };
    let mut body = format!(
        "### [{}]({})\n\n{} · {} · {}\n\n**{}.** {}\n",
        md(resource.title.as_str()),
        destination(&resource.url),
        resource.kind.label(),
        authors,
        access,
        resource.relation.label(),
        md(resource.why.as_str())
    );
    if let Some(event) = &resource.event {
        body.push_str(&format!(
            "\n[{} {}]({})",
            md(event.name.as_str()),
            md(&event.edition.0),
            destination(&event.source_url)
        ));
        if let Some(date) = event.starts_on {
            body.push_str(&format!(" · {}", date.0));
        }
        body.push_str(&format!(" · {}.\n", resource.temporal_status.label()));
    } else if resource.temporal_status != TemporalStatus::Published {
        body.push_str(&format!("\n{}.\n", resource.temporal_status.label()));
    }
    if let Some(date) = resource.published_on {
        body.push_str(&format!("\nPublication date: {}.\n", date.0));
    }
    if let Some(caveat) = &resource.caveat {
        body.push_str(&format!("\n{}\n", md(caveat.as_str())));
    }
    let evidence = resource
        .evidence_urls
        .iter()
        .map(|u| {
            format!(
                "[{}]({})",
                url::Url::parse(u.as_str())
                    .ok()
                    .and_then(|u| u.host_str().map(str::to_owned))
                    .unwrap_or_else(|| "Source".to_owned()),
                destination(u)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    body.push_str(&format!(
        "\n{}: {}. Evidence: {}.\n\n",
        resource.verification.label(),
        resource.checked_on.0,
        evidence
    ));
    body
}

fn decision_card(catalog: &Catalog, session: &Session, set: &ResourceSet) -> String {
    let mut out = String::from(
        "## Decide and prepare\n\nThese preparation suggestions are editorial. Check the official description for fit and access.\n\n",
    );
    if let Some(annotation) = &session.annotation {
        if annotation.review_status == AnnotationReview::SourceChanged {
            out.push_str("- The source changed; recheck the summary, prerequisites, and setup.\n");
        } else {
            out.push_str(&format!(
                "- **Focus:** {}\n- **Prerequisites:** {}\n- **Setup:** {}\n",
                md(annotation.summary.as_str()),
                md(annotation
                    .prerequisites
                    .as_ref()
                    .map_or("Not established; ask the organizer.", Prerequisite::as_str)),
                md(annotation.setup.as_ref().map_or(
                    "Not established; check the session page.",
                    SetupRequirement::as_str
                ))
            ));
        }
    }
    out.push_str(&format!(
        "- **Access:** {} Remote participation: {}\n",
        md(session.ticket_requirement.as_ref().map_or(
            "Ticket conditions unconfirmed; check the official page.",
            AccessNote::as_str
        )),
        md(session
            .remote_access
            .as_ref()
            .map_or("unconfirmed.", AccessNote::as_str))
    ));
    let first = set
        .resources
        .iter()
        .find(|r| {
            r.relation == Relation::Preparation
                && r.temporal_status == TemporalStatus::Published
                && r.access == Access::Open
        })
        .or_else(|| {
            set.resources.iter().find(|r| {
                r.temporal_status == TemporalStatus::Published && r.access == Access::Open
            })
        });
    if let Some(resource) = first {
        out.push_str(&format!(
            "- **Start with:** [{}]({}). {}\n",
            md(resource.title.as_str()),
            destination(&resource.url),
            md(resource.why.as_str())
        ));
    }
    let has_topic = |wanted| {
        session
            .annotation
            .as_ref()
            .filter(|a| a.review_status != AnnotationReview::SourceChanged)
            .is_some_and(|a| a.topics.iter().any(|t| t.as_str() == wanted))
    };
    let question = if session.id.as_str() == "talks-game-show" {
        "Which clue helps you distinguish a bug from surprising but valid Rust?"
    } else if has_topic("robotics") || has_topic("embedded") {
        "Which assumptions need evidence from the real hardware, and which can you test in simulation?"
    } else if has_topic("types") {
        "Which rule can the type system enforce, and which still needs a runtime check?"
    } else if has_topic("infrastructure") {
        "What confirms the intended configuration became the actual running state?"
    } else if has_topic("observability") {
        "How much overhead does the measurement add, and which delays does it reveal or miss?"
    } else if has_topic("async") || has_topic("production-reliability") {
        "What happens if this work is cancelled or the process stops halfway through?"
    } else {
        "Which assumption would you test before using this approach in your own project?"
    };
    out.push_str(&format!("- **Bring a question:** {question}\n"));
    let alternatives = catalog
        .sessions
        .iter()
        .filter(|other| {
            other.id != session.id
                && other.slot.start() == session.slot.start()
                && matches!(other.kind, SessionKind::Talk | SessionKind::Workshop)
        })
        .map(|s| format!("[{}]({}.md)", md(s.title.as_str()), s.id.as_str()))
        .collect::<Vec<_>>();
    if !alternatives.is_empty() {
        out.push_str(&format!("- **Other sessions starting at this time:** {}. Choose one; check its full time and access requirements.\n", alternatives.join(" · ")));
    }
    out.push_str("\nAfter studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).\n\n");
    out
}

impl CheckedLearning {
    fn views(&self) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
        let catalog = self.bundle.catalog();
        let snapshot = &self.bundle.manifest().snapshot_id;
        let mut views = BTreeMap::new();
        let mut index = String::from(
            "# Find resources for your session\n\nChoose a talk or workshop, then one useful source. Each page explains the connection\nand links books, articles, papers, videos, or projects.\n\nThese are curated suggestions. Source-page checks do not mean a whole work was\nreviewed, a video played, or a project run. Upcoming sessions are labelled **Announced**.\n\n[Choose a first source](../learning-paths.md) · [Browse topics](../topics/README.md) · [Conference archives](../conferences/README.md) · [Speakers](../speakers/README.md)\n\n| EuroRust session | Resources |\n| --- | ---: |\n",
        );
        let mut pack = format!(
            "# EuroRust learning resources\n\nCatalog snapshot: `{}`. Format: `1.0.0`.\n\nCurated source-checked drafts, with community review pending. Use source dates and\naccess notes; a listing does not prove attendance or a published recording.\n\n",
            snapshot.as_str()
        );
        let mut topics: BTreeMap<Topic, Vec<&Session>> = BTreeMap::new();
        for session in learning_sessions(catalog) {
            let set = self
                .sets
                .iter()
                .find(|s| s.session_id == session.id)
                .ok_or_else(|| invalid("session", "missing checked resource set"))?;
            let mut body = format!(
                "# {}\n\n[Official EuroRust session]({}) · [All session resources](README.md)\n\n{}\n\n",
                md(session.title.as_str()),
                destination(&session.official_url),
                md(set.research_note.as_str())
            );
            let speakers = session
                .speaker_ids
                .iter()
                .map(|id| {
                    let speaker = catalog
                        .speakers
                        .iter()
                        .find(|s| &s.id == id)
                        .ok_or_else(|| invalid("speaker navigation", "missing catalog speaker"))?;
                    let slug = self
                        .speaker_routes
                        .get(id)
                        .ok_or_else(|| invalid("speaker navigation", "missing profile route"))?;
                    Ok(format!(
                        "[{}](../speakers/{}.md)",
                        md(speaker.name.as_str()),
                        slug.0
                    ))
                })
                .collect::<Result<Vec<_>>>()?
                .join(", ");
            if speakers.is_empty() {
                body.push_str("No individual speaker is named in this catalog.\n\n");
            } else {
                body.push_str(&format!("Speakers and their work: {speakers}.\n\n"));
            }
            body.push_str(&decision_card(catalog, session, set));
            body.push_str("## Related talks and workshops\n\n");
            let related: Vec<_> = set.resources.iter().filter(|r| r.event.is_some()).collect();
            if related.is_empty() {
                body.push_str("No specific related conference appearance was verified in this pass. The sources below provide topic background.\n\n");
            }
            for resource in related {
                body.push_str(&render_resource(resource));
            }
            body.push_str("## Read, watch, and try\n\n");
            let other: Vec<_> = set.resources.iter().filter(|r| r.event.is_none()).collect();
            if other.is_empty() {
                body.push_str("Start with the sources linked above.\n\n");
            }
            for resource in other {
                body.push_str(&render_resource(resource));
            }
            body.push_str(&format!("Source review: **{}**. {}. Connections are editorial suggestions.\n\n[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/{}.json)\n", set.checked_on.0, set.review_status, session.id.as_str()));
            views.insert(
                PathBuf::from(format!(
                    "resources/related-talks/{}.md",
                    session.id.as_str()
                )),
                body.into_bytes(),
            );
            index.push_str(&format!(
                "| [{}]({}.md) | {} |\n",
                md(session.title.as_str()),
                session.id.as_str(),
                set.resources.len()
            ));
            pack.push_str(&format!(
                "## {}\n\nSession ID: `{}`. [Official source]({}).\n\n{}\n\n",
                md(session.title.as_str()),
                session.id.as_str(),
                destination(&session.official_url),
                md(set.research_note.as_str())
            ));
            for resource in &set.resources {
                pack.push_str(&render_resource(resource));
            }
            if let Some(annotation) = &session.annotation {
                for topic in &annotation.topics {
                    // Topic values are also used as filenames; reject traversal and arbitrary paths.
                    ResourceId::try_from(topic.as_str().to_owned())?;
                    topics.entry(topic.clone()).or_default().push(session);
                }
            }
        }
        index.push_str("\n[Portable learning pack](../../exports/archive/learning-pack.md) · [JSON](../../exports/archive/session-resources.json) · [Format and limits](../../docs/learning-resources.md)\n");
        views.insert(
            "resources/related-talks/README.md".into(),
            index.into_bytes(),
        );
        pack.truncate(pack.trim_end().len());
        pack.push('\n');
        views.insert("exports/archive/learning-pack.md".into(), pack.into_bytes());
        let mut topic_index = String::from(
            "# Browse Rust topics\n\nThese indexes group EuroRust session reading lists by topic. For a suggested\nstarting order, use [four engineering questions](../learning-paths.md). Tags reflect\ncommunity interpretation, not a promise about the speaker's coverage.\n\n| Topic | Sessions |\n| --- | ---: |\n",
        );
        for (topic, sessions) in topics {
            let mut name = topic.as_str().replace('-', " ");
            if let Some(first) = name.get_mut(..1) {
                first.make_ascii_uppercase();
            }
            if topic.as_str() == "api-design" {
                name = "API design".into();
            }
            if topic.as_str() == "gpu" {
                name = "GPU".into();
            }
            let mut body = format!(
                "# {}\n\nChoose a session to open its reading list. Each session page distinguishes\nearlier material from announced future sessions. For a starting recommendation,\nsee [four engineering questions](../learning-paths.md).\n\n",
                md(&name)
            );
            for session in &sessions {
                body.push_str(&format!(
                    "- [{}](../related-talks/{}.md)\n",
                    md(session.title.as_str()),
                    session.id.as_str()
                ));
            }
            body.push_str("\n[Follow technical questions through conference history](../conferences/history.md#follow-a-technical-question) · [All topics](README.md) · [Conference archives](../conferences/README.md) · [Speaker directory](../speakers/README.md)\n");
            topic_index.push_str(&format!(
                "| [{}]({}.md) | {} |\n",
                md(&name),
                topic.as_str(),
                sessions.len()
            ));
            views.insert(
                format!("resources/topics/{}.md", topic.as_str()).into(),
                body.into_bytes(),
            );
        }
        views.insert(
            "resources/topics/README.md".into(),
            topic_index.into_bytes(),
        );
        // JSON values are an output adapter, after domain validation.
        let json = serde_json::json!({"format_version":"1.0.0","catalog_snapshot_id":snapshot,"curation":"Codex-assisted source-checked drafts; community review pending","sessions":self.sets});
        views.insert(
            "exports/archive/session-resources.json".into(),
            format!("{}\n", serde_json::to_string_pretty(&json)?).into_bytes(),
        );
        let lines = self
            .sets
            .iter()
            .map(serde_json::to_string)
            .collect::<std::result::Result<Vec<_>, _>>()?
            .join("\n")
            + "\n";
        views.insert(
            "exports/archive/session-resources.jsonl".into(),
            lines.into_bytes(),
        );
        let schema = schemars::schema_for!(ResourceSet);
        views.insert(
            "schema/session-resources.schema.json".into(),
            format!("{}\n", serde_json::to_string_pretty(&schema)?).into_bytes(),
        );
        let hashes: BTreeMap<_, _> = views
            .iter()
            .map(|(p, v)| (p.clone(), ContentHash::from_content(v)))
            .collect();
        let manifest = serde_json::json!({"format_version":"1.0.0","catalog_snapshot_id":snapshot,"session_count":SessionCount::from(self.sets.len()),"files":hashes,"limits":"Offline consistency checks; no live link, video playback, attendance, or endorsement claim. Conference archive inventory has separate coverage metadata."});
        views.insert(
            "exports/archive/manifest.json".into(),
            format!("{}\n", serde_json::to_string_pretty(&manifest)?).into_bytes(),
        );
        Ok(views)
    }
    /// Render only validated input. Rerun to recover from an interrupted filesystem write.
    pub fn render(&self, root: &Path) -> Result<()> {
        let views = self.views()?;
        check_owned_paths(root, &views)?;
        for (path, bytes) in views {
            let path = root.join(path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, bytes)?;
        }
        Ok(())
    }
    /// Verify all generated views without changing files or claiming web freshness.
    pub fn verify(&self, root: &Path) -> Result<()> {
        let views = self.views()?;
        check_owned_paths(root, &views)?;
        for (path, expected) in views {
            if fs::read(root.join(&path))? != expected {
                return Err(Error::Validation(format!(
                    "stale learning view: {}; run build-learning",
                    path.display()
                )));
            }
        }
        println!(
            "Checked learning resources for {} sessions and all generated views",
            self.sets.len()
        );
        Ok(())
    }
}

fn check_owned_paths(root: &Path, views: &BTreeMap<PathBuf, Vec<u8>>) -> Result<()> {
    for directory in ["resources/related-talks", "resources/topics"] {
        if !root.join(directory).exists() {
            continue;
        }
        for entry in walkdir::WalkDir::new(root.join(directory)) {
            let entry = entry.map_err(|e| Error::Validation(e.to_string()))?;
            if entry.file_type().is_file() && entry.path().extension().is_some_and(|e| e == "md") {
                let relative = entry
                    .path()
                    .strip_prefix(root)
                    .map_err(|e| Error::Validation(e.to_string()))?;
                if !views.contains_key(relative) {
                    return Err(invalid(
                        "learning views",
                        format!(
                            "obsolete generated page {}; review and remove it before rebuilding",
                            relative.display()
                        ),
                    ));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn resource() -> serde_json::Value {
        serde_json::json!({"id":"reading","title":"A source","kind":"article","url":"https://example.com/article","authors":["An author"],"published_on":null,"access":"open","relation":"preparation","why":"Useful context","evidence_urls":["https://example.com/article"],"checked_on":"2026-10-15","verification":"source_content_checked","temporal_status":"published","event":null,"caveat":null})
    }
    fn fixture() -> (Catalog, Vec<ResourceSet>) {
        let catalog: Catalog =
            serde_json::from_str(include_str!("../../../data/catalog.json")).unwrap();
        let sets = learning_sessions(&catalog).map(|s| serde_json::from_value(serde_json::json!({"session_id":s.id,"checked_on":"2026-10-15","review_status":"source_checked_draft","research_note":"Fixture","resources":[resource()]})).unwrap()).collect();
        (catalog, sets)
    }
    #[test]
    fn decoding_rejects_bad_source_and_identity() {
        let mut value = resource();
        value["url"] = "javascript:alert(1)".into();
        assert!(serde_json::from_value::<Resource>(value).is_err());
        assert!(serde_json::from_str::<ResourceId>("\"../private\"").is_err());
        assert!(serde_json::from_str::<CheckedOn>("\"2026-02-30\"").is_err());
    }
    #[test]
    fn checks_coverage_evidence_dates_and_duplicates() {
        let (catalog, mut sets) = fixture();
        validate_sets(&sets, &catalog).unwrap();
        sets[0].resources[0].evidence_urls.clear();
        assert!(validate_sets(&sets, &catalog).is_err());
        let (catalog, mut sets) = fixture();
        sets[0].resources[0].published_on =
            Some(PublishedOn(NaiveDate::from_ymd_opt(2027, 1, 1).unwrap()));
        assert!(validate_sets(&sets, &catalog).is_err());
        sets[0].resources[0].temporal_status = TemporalStatus::Announced;
        validate_sets(&sets, &catalog).unwrap();
        let duplicate = sets[0].resources[0].clone();
        sets[0].resources.push(duplicate);
        assert!(validate_sets(&sets, &catalog).is_err());
        sets.pop();
        assert!(validate_sets(&sets, &catalog).is_err());
    }
    #[test]
    fn escapes_source_markdown() {
        assert_eq!(md("[x](y)<script>|"), "\\[x\\](y)&lt;script&gt;\\|");
    }

    #[test]
    fn checked_render_detects_stale_views_and_recovers_without_publishing_bad_inputs() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let (catalog, sets) = fixture();
        let manifest =
            export::publish(DraftSnapshot::new(catalog).validate().unwrap(), root).unwrap();
        fs::create_dir_all(root.join("community/session-resources")).unwrap();
        fs::write(
            root.join("community/speaker-profiles.json"),
            include_bytes!("../../../community/speaker-profiles.json"),
        )
        .unwrap();
        let profiles: SpeakerRoutes =
            serde_json::from_slice(include_bytes!("../../../community/speaker-profiles.json"))
                .unwrap();
        fs::create_dir_all(root.join("resources/speakers")).unwrap();
        for profile in profiles.profiles {
            fs::write(
                root.join(format!("resources/speakers/{}.md", profile.profile_id.0)),
                "Profile fixture",
            )
            .unwrap();
        }
        fs::write(root.join("community/learning-manifest.json"),serde_json::to_vec(&serde_json::json!({"format_version":"1.0.0","catalog_snapshot_id":manifest.snapshot_id})).unwrap()).unwrap();
        for set in &sets {
            fs::write(
                root.join(format!(
                    "community/session-resources/{}.json",
                    set.session_id.as_str()
                )),
                serde_json::to_vec(set).unwrap(),
            )
            .unwrap();
        }
        rebind(root).unwrap();
        let checked = load(root).unwrap();
        checked.render(root).unwrap();
        checked.verify(root).unwrap();
        let profile_path = root.join("community/speaker-profiles.json");
        let profile_bytes = fs::read(&profile_path).unwrap();
        let mut profiles: serde_json::Value = serde_json::from_slice(&profile_bytes).unwrap();
        profiles["profiles"][1]["profile_id"] = profiles["profiles"][0]["profile_id"].clone();
        fs::write(&profile_path, serde_json::to_vec(&profiles).unwrap()).unwrap();
        assert!(load(root).is_err());
        fs::write(&profile_path, profile_bytes).unwrap();
        let view = root.join("resources/related-talks/README.md");
        let expected = fs::read(&view).unwrap();
        fs::write(&view, "stale view").unwrap();
        assert!(checked.verify(root).is_err());
        checked.render(root).unwrap();
        assert_eq!(fs::read(&view).unwrap(), expected);
        let orphan = root.join("resources/topics/retired-topic.md");
        fs::write(&orphan, "obsolete").unwrap();
        assert!(checked.verify(root).is_err());
        assert!(checked.render(root).is_err());
        fs::remove_file(orphan).unwrap();
        let input = root.join(format!(
            "community/session-resources/{}.json",
            sets[0].session_id.as_str()
        ));
        fs::write(input, b"{}").unwrap();
        assert!(load(root).and_then(|checked| checked.render(root)).is_err());
        assert_eq!(fs::read(&view).unwrap(), expected);
    }

    #[test]
    fn announcement_dates_and_prepublished_papers_remain_distinct() {
        let (catalog, mut sets) = fixture();
        sets[0].resources[0].temporal_status = TemporalStatus::Announced;
        assert!(validate_sets(&sets, &catalog).is_err());
        sets[0].resources[0].published_on =
            Some(PublishedOn(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()));
        assert!(validate_sets(&sets, &catalog).is_err());
        sets[0].resources[0].temporal_status = TemporalStatus::Published;
        sets[0].resources[0].kind = ResourceKind::Paper;
        sets[0].resources[0].event=Some(serde_json::from_value(serde_json::json!({"name":"Research conference","edition":"2027","starts_on":"2027-01-01","source_url":"https://example.com/event"})).unwrap());
        validate_sets(&sets, &catalog).unwrap();
        sets[0].resources[0].kind = ResourceKind::ConferenceTalk;
        assert!(validate_sets(&sets, &catalog).is_err());
    }

    #[test]
    fn content_fingerprint_ignores_retrieval_but_detects_session_changes() {
        let (mut catalog, _) = fixture();
        let original = context_hash(&catalog).unwrap();
        catalog.sessions[0].source.retrieved_at = Timestamp::now();
        assert_eq!(original, context_hash(&catalog).unwrap());
        catalog.sessions[0].title = "Changed session topic".to_owned().try_into().unwrap();
        assert_ne!(original, context_hash(&catalog).unwrap());
    }
}
