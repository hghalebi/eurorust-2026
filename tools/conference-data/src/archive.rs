//! Offline checks for curated conference inventories, separate from the EuroRust catalog.
use crate::{
    Error, Result,
    domain::{AnnotationReview, Author, PersonName, SchemaVersion, SnapshotId, Summary, Title},
    invalid,
};
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

macro_rules! identifier {
    ($($name:ident),+) => {$(
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
        #[serde(try_from = "String", into = "String")]
        struct $name(String);
        impl TryFrom<String> for $name {
            type Error = Error;
            fn try_from(value: String) -> Result<Self> {
                if value.is_empty() || !value.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-') {
                    return Err(invalid(stringify!($name), "expected a lowercase identifier"));
                }
                Ok(Self(value))
            }
        }
        impl From<$name> for String { fn from(value: $name) -> Self { value.0 } }
    )+};
}
identifier!(SeriesId, EditionId, AppearanceId);

// Source identifiers and labels are not local slugs. Preserve their spelling
// while applying the shared nonempty, single-line text boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
struct SourceSessionId(Summary);
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
struct SourceLanguage(Summary);
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
struct Region(Summary);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct ArchivePage(String);
impl TryFrom<String> for ArchivePage {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        if !value.starts_with("resources/conferences/")
            || !value.ends_with(".md")
            || value.contains('\\')
            || value.chars().any(char::is_control)
            || value.split('/').any(|part| matches!(part, "" | "." | ".."))
        {
            return Err(invalid(
                "ArchivePage",
                "expected a Markdown path within resources/conferences",
            ));
        }
        Ok(Self(value))
    }
}
impl From<ArchivePage> for String {
    fn from(value: ArchivePage) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct ArchiveUrl(String);
impl TryFrom<String> for ArchiveUrl {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        let url = url::Url::parse(&value).map_err(|e| invalid("ArchiveUrl", e.to_string()))?;
        if !matches!(url.scheme(), "http" | "https")
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
        {
            return Err(invalid(
                "ArchiveUrl",
                "expected a public HTTP(S) URL without credentials",
            ));
        }
        Ok(Self(value))
    }
}
impl From<ArchiveUrl> for String {
    fn from(value: ArchiveUrl) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
struct EventDate(NaiveDate);
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
struct ReviewDate(NaiveDate);
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
struct EditionYear(u16);
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
struct EntryCount(usize);
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
struct IndexedCount(usize);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SeriesKind {
    Conference,
    Track,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EditionStatus {
    Announced,
    PastProgramme,
    CurrentProgramme,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum CoverageStatus {
    Indexed,
    Partial,
    SourceOnly,
    Blocked,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum AppearanceKind {
    Talk,
    Workshop,
    Panel,
    Other,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MaterialKind {
    Video,
    Slides,
    Code,
    Transcript,
    Reading,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MaterialAvailability {
    LinkedBySource,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(from = "bool", into = "bool")]
enum Cancellation {
    Cancelled,
    NotCancelled,
}
impl From<bool> for Cancellation {
    fn from(value: bool) -> Self {
        if value {
            Self::Cancelled
        } else {
            Self::NotCancelled
        }
    }
}
impl From<Cancellation> for bool {
    fn from(value: Cancellation) -> Self {
        matches!(value, Cancellation::Cancelled)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Series {
    id: SeriesId,
    name: Title,
    kind: SeriesKind,
    url: ArchiveUrl,
    editions: Vec<EditionId>,
    region: Option<Region>,
    #[serde(default)]
    notes: Vec<Summary>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Coverage {
    status: CoverageStatus,
    programme_session_count: Option<EntryCount>,
    indexed_session_count: IndexedCount,
    gaps: Vec<Summary>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Edition {
    id: EditionId,
    series_id: SeriesId,
    name: Title,
    year: EditionYear,
    starts_on: Option<EventDate>,
    ends_on: Option<EventDate>,
    status: EditionStatus,
    programme_url: ArchiveUrl,
    speakers_url: Option<ArchiveUrl>,
    recordings_url: Option<ArchiveUrl>,
    source_urls: Vec<ArchiveUrl>,
    coverage: Coverage,
    checked_on: ReviewDate,
    #[serde(default)]
    language: Vec<SourceLanguage>,
    #[serde(default)]
    notes: Vec<Summary>,
    page: Option<ArchivePage>,
    #[serde(default)]
    material_urls: Vec<Material>,
    cancelled: Option<Cancellation>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Material {
    kind: MaterialKind,
    url: ArchiveUrl,
    title: Title,
    evidence_url: Option<ArchiveUrl>,
    availability: Option<MaterialAvailability>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SpeakerLink {
    name: PersonName,
    url: ArchiveUrl,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TranslatedTitle {
    text: Title,
    language: SourceLanguage,
    attribution: Author,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Appearance {
    id: AppearanceId,
    edition_id: EditionId,
    title: Title,
    kind: AppearanceKind,
    speakers: Vec<PersonName>,
    date: Option<EventDate>,
    source_url: ArchiveUrl,
    materials: Vec<Material>,
    #[serde(default)]
    speaker_links: Vec<SpeakerLink>,
    #[serde(default)]
    source_urls: Vec<ArchiveUrl>,
    notes: Option<Summary>,
    checked_on: ReviewDate,
    source_session_id: Option<SourceSessionId>,
    translated_title: Option<TranslatedTitle>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SeriesFile {
    format_version: SchemaVersion,
    checked_on: ReviewDate,
    review_status: AnnotationReview,
    eurorust_snapshot_id: SnapshotId,
    series: Vec<Series>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EditionsFile {
    format_version: SchemaVersion,
    checked_on: ReviewDate,
    review_status: AnnotationReview,
    eurorust_snapshot_id: SnapshotId,
    editions: Vec<Edition>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AppearancesFile {
    format_version: SchemaVersion,
    checked_on: ReviewDate,
    review_status: AnnotationReview,
    eurorust_snapshot_id: SnapshotId,
    appearances: Vec<Appearance>,
}

fn validate(series: &[Series], editions: &[Edition], appearances: &[Appearance]) -> Result<()> {
    let series_ids: BTreeSet<_> = series.iter().map(|s| &s.id).collect();
    let edition_ids: BTreeMap<_, _> = editions.iter().map(|e| (&e.id, e)).collect();
    let appearance_ids: BTreeSet<_> = appearances.iter().map(|a| &a.id).collect();
    if series_ids.len() != series.len()
        || edition_ids.len() != editions.len()
        || appearance_ids.len() != appearances.len()
    {
        return Err(invalid(
            "archive identity",
            "duplicate series, edition, or appearance ID",
        ));
    }
    for series in series {
        let expected: BTreeSet<_> = editions
            .iter()
            .filter(|e| e.series_id == series.id)
            .map(|e| &e.id)
            .collect();
        let listed: BTreeSet<_> = series.editions.iter().collect();
        if expected != listed || listed.len() != series.editions.len() {
            return Err(invalid(
                "series editions",
                format!("broken edition list for {}", series.id.0),
            ));
        }
    }
    for edition in editions {
        if !series_ids.contains(&edition.series_id) || edition.source_urls.is_empty() {
            return Err(invalid(
                "edition",
                "unknown series or missing source evidence",
            ));
        }
        if let (Some(start), Some(end)) = (edition.starts_on, edition.ends_on)
            && end.0 < start.0
        {
            return Err(invalid("edition dates", "end precedes start"));
        }
        match edition.status {
            EditionStatus::PastProgramme
                if i32::from(edition.year.0) > edition.checked_on.0.year()
                    || edition
                        .ends_on
                        .or(edition.starts_on)
                        .is_some_and(|d| d.0 > edition.checked_on.0) =>
            {
                return Err(invalid(
                    "edition status",
                    "future edition cannot be marked past",
                ));
            }
            EditionStatus::Announced
                if i32::from(edition.year.0) < edition.checked_on.0.year()
                    || edition
                        .ends_on
                        .or(edition.starts_on)
                        .is_some_and(|d| d.0 < edition.checked_on.0) =>
            {
                return Err(invalid(
                    "edition status",
                    "passed edition cannot be marked announced",
                ));
            }
            EditionStatus::CurrentProgramme
                if edition.starts_on.is_none()
                    || edition.ends_on.is_none()
                    || edition
                        .starts_on
                        .is_some_and(|d| d.0 > edition.checked_on.0)
                    || edition.ends_on.is_some_and(|d| d.0 < edition.checked_on.0) =>
            {
                return Err(invalid(
                    "edition status",
                    "current programme date is outside its event window",
                ));
            }
            _ => {}
        }
        let count = appearances
            .iter()
            .filter(|a| a.edition_id == edition.id)
            .count();
        if count != edition.coverage.indexed_session_count.0 {
            return Err(invalid(
                "archive coverage",
                format!("indexed count disagrees for {}", edition.id.0),
            ));
        }
        if edition
            .coverage
            .programme_session_count
            .is_some_and(|n| count > n.0)
        {
            return Err(invalid(
                "archive coverage",
                "indexed count exceeds observed programme count",
            ));
        }
        if edition.coverage.status == CoverageStatus::Indexed
            && edition
                .coverage
                .programme_session_count
                .is_none_or(|n| count != n.0)
        {
            return Err(invalid(
                "archive coverage",
                "indexed status requires a reconciled programme denominator",
            ));
        }
        if edition.coverage.status != CoverageStatus::Indexed && edition.coverage.gaps.is_empty() {
            return Err(invalid(
                "archive coverage",
                "incomplete inventory needs an explicit gap",
            ));
        }
    }
    for appearance in appearances {
        if appearance.speaker_links.iter().any(|link| {
            !appearance
                .speakers
                .iter()
                .any(|name| name.as_str() == link.name.as_str())
        }) {
            return Err(invalid(
                "speaker attribution",
                format!(
                    "profile name does not match appearance speaker in {}",
                    appearance.id.0
                ),
            ));
        }
        let edition = edition_ids
            .get(&appearance.edition_id)
            .ok_or_else(|| invalid("appearance", "unknown edition"))?;
        if let Some(date) = appearance.date
            && (edition.starts_on.is_some_and(|s| date.0 < s.0)
                || edition.ends_on.is_some_and(|e| date.0 > e.0))
        {
            return Err(invalid(
                "appearance date",
                format!("outside edition bounds: {}", appearance.id.0),
            ));
        }
    }
    Ok(())
}

pub fn check(root: &Path) -> Result<()> {
    let directory = root.join("community/conference-archive");
    let series: SeriesFile = serde_json::from_slice(&fs::read(directory.join("series.json"))?)?;
    let editions: EditionsFile =
        serde_json::from_slice(&fs::read(directory.join("editions.json"))?)?;
    let appearances: AppearancesFile =
        serde_json::from_slice(&fs::read(directory.join("appearances.json"))?)?;
    for version in [
        &series.format_version,
        &editions.format_version,
        &appearances.format_version,
    ] {
        if version.as_str() != "1.0.0" {
            return Err(invalid("archive format", "unsupported version"));
        }
    }
    validate(&series.series, &editions.editions, &appearances.appearances)?;
    println!(
        "Checked {} conference series, {} editions, {} indexed appearances; coverage remains edition-specific",
        series.series.len(),
        editions.editions.len(),
        appearances.appearances.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn appearance() -> serde_json::Value {
        serde_json::json!({
            "id":"example-talk", "edition_id":"example-2026", "title":"A talk",
            "kind":"talk", "speakers":["A Speaker"], "date":"2026-10-15",
            "source_url":"https://example.com/talk",
            "materials":[{
                "kind":"video", "url":"https://example.com/video", "title":"Recording",
                "evidence_url":"https://example.com/talk", "availability":"linked_by_source"
            }],
            "speaker_links":[{"name":"A Speaker","url":"https://example.com/speaker"}],
            "source_urls":["https://example.com/programme"], "notes":null,
            "checked_on":"2026-10-15", "source_session_id":"d1-P-1027-0",
            "translated_title":{"text":"A translated title","language":"en","attribution":"organizer"}
        })
    }

    fn file(key: &str, records: serde_json::Value) -> serde_json::Value {
        let mut value = serde_json::json!({
            "format_version":"1.0.0", "checked_on":"2026-10-15",
            "review_status":"source_checked_draft", "eurorust_snapshot_id":"eurorust-2026-example"
        });
        value[key] = records;
        value
    }

    #[test]
    fn retains_intentional_archive_metadata() {
        let source = appearance();
        let decoded: Appearance = serde_json::from_value(source.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), source);
        serde_json::from_value::<AppearancesFile>(file("appearances", serde_json::json!([source])))
            .unwrap();

        let series = serde_json::json!({
            "id":"example", "name":"Example", "kind":"conference", "url":"https://example.com",
            "editions":["example-2026"], "region":"Europe / online", "notes":["Source context"]
        });
        let source = file("series", serde_json::json!([series]));
        let decoded: SeriesFile = serde_json::from_value(source.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), source);

        let edition = serde_json::json!({
            "id":"example-2026", "series_id":"example", "name":"Example 2026", "year":2026,
            "starts_on":null, "ends_on":null, "status":"unknown", "cancelled":true,
            "programme_url":"https://example.com/programme", "speakers_url":null,
            "recordings_url":null, "source_urls":["https://example.com"],
            "coverage":{"status":"source_only","programme_session_count":null,"indexed_session_count":0,"gaps":["Cancelled"]},
            "checked_on":"2026-10-15", "language":["en"], "notes":["Cancelled by organizer"],
            "page":"resources/conferences/example/2026.md",
            "material_urls":[{"kind":"slides","url":"https://example.com/slides","title":"Slide bundle"}]
        });
        let decoded: EditionsFile =
            serde_json::from_value(file("editions", serde_json::json!([edition]))).unwrap();
        assert!(matches!(
            decoded.editions[0].cancelled,
            Some(Cancellation::Cancelled)
        ));
        let encoded = serde_json::to_value(decoded).unwrap();
        assert_eq!(encoded["editions"][0]["cancelled"], true);
        assert_eq!(
            encoded["editions"][0]["page"],
            "resources/conferences/example/2026.md"
        );
        assert_eq!(encoded["editions"][0]["material_urls"][0]["kind"], "slides");
        assert!(
            ArchivePage::try_from("resources/conferences/../../.plan/private.md".to_owned())
                .is_err()
        );
    }

    #[test]
    fn rejects_unknown_fields_in_records_nested_metadata_and_file_headers() {
        fn rejects<T: serde::de::DeserializeOwned>(value: serde_json::Value, field: &str) {
            let error = serde_json::from_value::<T>(value)
                .err()
                .expect("unknown field must fail decoding");
            assert!(
                error
                    .to_string()
                    .contains(&format!("unknown field `{field}`"))
            );
        }

        let mut misspelled = appearance();
        let links = misspelled
            .as_object_mut()
            .unwrap()
            .remove("speaker_links")
            .unwrap();
        misspelled["speaker_lniks"] = links;
        rejects::<Appearance>(misspelled, "speaker_lniks");

        let mut nested = appearance();
        nested["translated_title"]["translator"] = "Unmodeled metadata".into();
        rejects::<Appearance>(nested, "translator");

        let mut material = appearance();
        material["materials"][0]["evidence_urll"] = "https://example.com/evidence".into();
        rejects::<Appearance>(material, "evidence_urll");

        let mut header = file("appearances", serde_json::json!([appearance()]));
        header["review_stats"] = "source_checked_draft".into();
        rejects::<AppearancesFile>(header, "review_stats");
    }

    #[test]
    fn rejects_unproved_coverage_and_broken_references() {
        let series: Vec<Series> = serde_json::from_value(serde_json::json!([{"id":"example","name":"Example","kind":"conference","url":"https://example.com","editions":["example-2026"]}])).unwrap();
        let mut editions: Vec<Edition> = serde_json::from_value(serde_json::json!([{"id":"example-2026","series_id":"example","name":"Example 2026","year":2026,"starts_on":null,"ends_on":null,"status":"unknown","programme_url":"https://example.com","speakers_url":null,"recordings_url":null,"source_urls":["https://example.com"],"coverage":{"status":"source_only","programme_session_count":null,"indexed_session_count":0,"gaps":["Programme unavailable"]},"checked_on":"2026-10-15"}])).unwrap();
        validate(&series, &editions, &[]).unwrap();
        editions[0].coverage.status = CoverageStatus::Indexed;
        assert!(validate(&series, &editions, &[]).is_err());
        editions[0].coverage.programme_session_count = Some(EntryCount(0));
        validate(&series, &editions, &[]).unwrap();
        editions[0].series_id = SeriesId::try_from("missing".to_owned()).unwrap();
        assert!(validate(&series, &editions, &[]).is_err());
        assert!(ArchiveUrl::try_from("javascript:alert(1)".to_owned()).is_err());
    }
}
