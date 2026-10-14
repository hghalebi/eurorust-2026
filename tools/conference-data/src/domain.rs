use crate::{Result, invalid};
use chrono::{DateTime, FixedOffset};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

macro_rules! text_value {
    ($($name:ident),+ $(,)?) => {$ (
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
        #[serde(try_from = "String", into = "String")]
        #[schemars(with = "String")]
        pub struct $name(String);
        impl TryFrom<String> for $name {
            type Error = crate::Error;
            fn try_from(value: String) -> Result<Self> {
                if value.trim().is_empty() || value.chars().any(char::is_control) {
                    return Err(invalid(stringify!($name), "expected nonempty text without control characters"));
                }
                Ok(Self(value))
            }
        }
        impl From<$name> for String { fn from(value: $name) -> Self { value.0 } }
        impl $name { pub fn as_str(&self) -> &str { &self.0 } }
    )+};
}

text_value!(
    Title,
    PersonName,
    VenueName,
    StageName,
    Summary,
    Prerequisite,
    SetupRequirement,
    AccessNote,
    Topic,
    Author,
    Method,
    ProfileName,
    Goal,
    Rationale,
    SchemaVersion,
    GeneratorVersion,
    Address
);

macro_rules! identifier {
    ($($name:ident),+ $(,)?) => {$ (
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
        #[serde(try_from = "String", into = "String")]
        #[schemars(with = "String")]
        pub struct $name(String);
        impl TryFrom<String> for $name {
            type Error = crate::Error;
            fn try_from(value: String) -> Result<Self> {
                if value.is_empty() || !value.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-') {
                    return Err(invalid(stringify!($name), "expected a lowercase URL-derived identifier"));
                }
                Ok(Self(value))
            }
        }
        impl From<$name> for String { fn from(value: $name) -> Self { value.0 } }
        impl $name { pub fn as_str(&self) -> &str { &self.0 } }
    )+};
}

/// Session identity is distinct from speaker identity.
/// ```
/// use conference_data::domain::SessionId;
/// let id = SessionId::try_from("talks-example".to_owned()).unwrap();
/// assert_eq!(id.as_str(), "talks-example");
/// ```
/// ```compile_fail
/// use conference_data::domain::{SessionId, SpeakerId};
/// let speaker = SpeakerId::try_from("speaker-example".to_owned()).unwrap();
/// let session: SessionId = speaker;
/// ```
pub fn identity_contract() {}

identifier!(SessionId, SpeakerId, VenueId, SnapshotId);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "String", into = "String")]
#[schemars(with = "String")]
pub struct ContentHash(String);
impl TryFrom<String> for ContentHash {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self> {
        if value.len() != 64
            || !value
                .bytes()
                .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
        {
            return Err(invalid(
                "ContentHash",
                "expected 64 lowercase hexadecimal SHA-256 characters",
            ));
        }
        Ok(Self(value))
    }
}
impl From<ContentHash> for String {
    fn from(value: ContentHash) -> Self {
        value.0
    }
}
impl ContentHash {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "String", into = "String")]
#[schemars(with = "String")]
pub struct SourceUrl(String);
impl TryFrom<String> for SourceUrl {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self> {
        let parsed = url::Url::parse(&value).map_err(|e| invalid("SourceUrl", e.to_string()))?;
        if parsed.scheme() != "https"
            || parsed.host_str().is_none()
            || !parsed.username().is_empty()
            || parsed.password().is_some()
        {
            return Err(invalid(
                "SourceUrl",
                "expected an HTTPS URL without credentials",
            ));
        }
        Ok(Self(parsed.into()))
    }
}
impl From<SourceUrl> for String {
    fn from(value: SourceUrl) -> Self {
        value.0
    }
}
impl SourceUrl {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "String", into = "String")]
#[schemars(with = "String")]
pub struct Timestamp(DateTime<FixedOffset>);
impl TryFrom<String> for Timestamp {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self> {
        DateTime::parse_from_rfc3339(&value)
            .map(Self)
            .map_err(|e| invalid("Timestamp", e.to_string()))
    }
}
impl From<Timestamp> for String {
    fn from(value: Timestamp) -> Self {
        value.0.to_rfc3339()
    }
}
impl Timestamp {
    pub fn now() -> Self {
        Self(chrono::Utc::now().fixed_offset())
    }
    pub fn datetime(&self) -> &DateTime<FixedOffset> {
        &self.0
    }
    pub fn as_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "SlotWire")]
pub struct Slot {
    start: Timestamp,
    end: Option<Timestamp>,
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SlotWire {
    start: Timestamp,
    end: Option<Timestamp>,
}
impl TryFrom<SlotWire> for Slot {
    type Error = crate::Error;
    fn try_from(wire: SlotWire) -> Result<Self> {
        Self::new(wire.start, wire.end)
    }
}
impl Slot {
    pub fn new(start: Timestamp, end: Option<Timestamp>) -> Result<Self> {
        if end.as_ref().is_some_and(|end| end <= &start) {
            return Err(invalid("Slot", "end must follow start"));
        }
        Ok(Self { start, end })
    }
    pub fn start(&self) -> &Timestamp {
        &self.start
    }
    pub fn end(&self) -> Option<&Timestamp> {
        self.end.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SessionKind {
    Talk,
    Workshop,
    SideActivity,
    Service,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub source_url: SourceUrl,
    pub retrieved_at: Timestamp,
    pub source_updated_at: Option<Timestamp>,
    pub content_sha256: ContentHash,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Annotation {
    pub session_id: SessionId,
    pub summary: Summary,
    pub topics: Vec<Topic>,
    pub prerequisites: Option<Prerequisite>,
    pub setup: Option<SetupRequirement>,
    pub author: Author,
    pub method: Method,
    pub review_status: AnnotationReview,
    pub source_checked_at: Timestamp,
    pub source_content_sha256: ContentHash,
    pub source_url: SourceUrl,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationReview {
    SourceCheckedDraft,
    CommunityReviewed,
    SourceChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Session {
    pub id: SessionId,
    pub title: Title,
    pub official_url: SourceUrl,
    pub kind: SessionKind,
    pub slot: Slot,
    pub stage: Option<StageName>,
    pub speaker_ids: Vec<SpeakerId>,
    pub venue_id: Option<VenueId>,
    pub source: Provenance,
    pub registration_url: Option<SourceUrl>,
    pub ticket_requirement: Option<AccessNote>,
    pub remote_access: Option<AccessNote>,
    pub recording_url: Option<SourceUrl>,
    pub detail_source: Option<Provenance>,
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Speaker {
    pub id: SpeakerId,
    pub name: PersonName,
    pub source_url: SourceUrl,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Venue {
    pub id: VenueId,
    pub name: VenueName,
    pub address: Option<Address>,
    pub source_url: SourceUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Conference {
    pub name: Title,
    pub timezone: ConferenceTimezone,
    pub source_url: SourceUrl,
    pub schedule_url: SourceUrl,
    pub starts_at: Timestamp,
    pub ends_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum ConferenceTimezone {
    #[serde(rename = "Europe/Madrid")]
    EuropeMadrid,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: SchemaVersion,
    pub conference: Conference,
    pub sessions: Vec<Session>,
    pub speakers: Vec<Speaker>,
    pub venues: Vec<Venue>,
}

pub struct DraftSnapshot(Catalog);
/// Only a checked draft can construct this publication capability.
/// ```compile_fail
/// use conference_data::domain::ValidatedSnapshot;
/// let snapshot = ValidatedSnapshot { catalog: todo!() };
/// ```
pub struct ValidatedSnapshot {
    catalog: Catalog,
}
impl DraftSnapshot {
    pub fn new(catalog: Catalog) -> Self {
        Self(catalog)
    }
    pub fn validate(self) -> Result<ValidatedSnapshot> {
        let catalog = self.0;
        if catalog.schema_version.as_str() != "1.0.0" {
            return Err(invalid("SchemaVersion", "unsupported schema version"));
        }
        if catalog.sessions.is_empty() {
            return Err(crate::Error::Validation("empty conference".into()));
        }
        let mut ids = BTreeSet::new();
        let speakers: BTreeSet<_> = catalog.speakers.iter().map(|s| &s.id).collect();
        let venues: BTreeSet<_> = catalog.venues.iter().map(|v| &v.id).collect();
        if speakers.len() != catalog.speakers.len() || venues.len() != catalog.venues.len() {
            return Err(crate::Error::Validation(
                "duplicate speaker or venue identifier".into(),
            ));
        }
        for session in &catalog.sessions {
            if !ids.insert(&session.id) {
                return Err(crate::Error::Validation(format!(
                    "duplicate session {}",
                    session.id.as_str()
                )));
            }
            if session.speaker_ids.iter().any(|id| !speakers.contains(id))
                || session
                    .venue_id
                    .as_ref()
                    .is_some_and(|id| !venues.contains(id))
            {
                return Err(crate::Error::Validation(format!(
                    "broken reference in {}",
                    session.id.as_str()
                )));
            }
            if session.slot.start() < &catalog.conference.starts_at
                || session.slot.end().unwrap_or(session.slot.start()) > &catalog.conference.ends_at
            {
                return Err(crate::Error::Validation(format!(
                    "session outside conference: {}",
                    session.id.as_str()
                )));
            }
            for time in std::iter::once(session.slot.start()).chain(session.slot.end()) {
                let local = time.datetime().with_timezone(&chrono_tz::Europe::Madrid);
                if local.offset().fix().local_minus_utc()
                    != time.datetime().offset().local_minus_utc()
                {
                    return Err(invalid(
                        "Timestamp",
                        "schedule offset must match Europe/Madrid",
                    ));
                }
            }
            if session
                .annotation
                .as_ref()
                .is_some_and(|a| a.session_id != session.id || a.source_url != session.official_url)
            {
                return Err(crate::Error::Validation(
                    "annotation source or identity mismatch".into(),
                ));
            }
        }
        Ok(ValidatedSnapshot { catalog })
    }
}
impl ValidatedSnapshot {
    pub fn catalog(&self) -> &Catalog {
        &self.catalog
    }
}

use chrono::Offset;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decoding_checks_construction() {
        assert!(serde_json::from_str::<SessionId>("\"Bad ID\"").is_err());
        assert!(serde_json::from_str::<Title>("\"\"").is_err());
        assert!(serde_json::from_str::<ContentHash>("\"invalid-digest\"").is_err());
        assert!(serde_json::from_str::<SourceUrl>("\"http://example.com\"").is_err());
        assert!(
            serde_json::from_str::<Slot>(
                r#"{"start":"2026-10-15T11:00:00+02:00","end":"2026-10-15T10:00:00+02:00"}"#
            )
            .is_err()
        );
        let slot: Slot =
            serde_json::from_str(r#"{"start":"2026-10-15T11:00:00+02:00","end":null}"#).unwrap();
        assert!(slot.end().is_none());
    }
}
