//! Wire values are checked at the JSON boundary; references are checked on load.
use crate::{Error, Result, domain::*, invalid};
use chrono::NaiveDate;
use chrono_tz::Tz;
use schemars::JsonSchema;
use serde::Deserialize;
use std::path::{Component, Path};

macro_rules! id_type {
    ($($name:ident),+ $(,)?) => {$(
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, JsonSchema)]
        #[serde(try_from = "String")]
        #[schemars(with = "String", extend("pattern" = "^[a-z0-9]+(-[a-z0-9]+)*$"))]
        pub(super) struct $name(pub(super) String);
        impl TryFrom<String> for $name {
            type Error = Error;
            fn try_from(value: String) -> Result<Self> {
                if value.split('-').any(|part| part.is_empty()
                    || !part.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())) {
                    return Err(invalid(stringify!($name), "expected lowercase words separated by hyphens"));
                }
                Ok(Self(value))
            }
        }
    )+};
}
id_type!(
    CommunityId,
    OpportunityId,
    RouteId,
    DiscoveryEventId,
    DiscoverySourceId
);

macro_rules! dates {
    ($($name:ident),+ $(,)?) => {$(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, JsonSchema)]
        #[serde(transparent)]
        pub(super) struct $name(pub(super) NaiveDate);
    )+};
}
dates!(
    ReferenceDate,
    CheckedOn,
    ReviewDue,
    ObservedOn,
    EventDate,
    DeadlineDate
);

macro_rules! choices {
    ($name:ident { $($variant:ident => $label:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
        #[serde(rename_all = "snake_case")]
        pub(super) enum $name { $($variant),+ }
        impl $name {
            pub(super) fn label(self) -> &'static str { match self { $(Self::$variant => $label),+ } }
        }
    };
}
choices!(CommunityKind { Organization => "organization", Forum => "forum", Team => "team", Project => "project", Learning => "learning", Regional => "regional group" });
choices!(Affiliation { RustProject => "Rust Project", Independent => "independent", Project => "project community", Adjacent => "related ecosystem" });
choices!(Region { Global => "online / global", Europe => "Europe", Africa => "Africa", Asia => "Asia", Americas => "Americas", Oceania => "Oceania" });
choices!(Mode { Online => "online", InPerson => "in person", Mixed => "online and in person", Unknown => "mode unverified" });
choices!(ReviewResult { PageChecked => "page checked", Unavailable => "source unavailable" });
choices!(OpportunityKind { CallForProposals => "call for proposals", Hackathon => "hackathon", Discussion => "discussion", Contribution => "contribution", Mentoring => "mentoring", Learning => "learning", Organizing => "organizing", Support => "support", Attendance => "attendance" });
choices!(OpportunityStatus { Open => "Open calls", Announced => "Announced activities", Route => "Ongoing participation routes", Historical => "Past examples", Closed => "Closed calls" });
choices!(CoverageStatus { Covered => "covered", Partial => "partial" });

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(super) enum Relationship {
    EditorialSuggestion,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum EventStatus {
    Announced,
    Ongoing,
    Tentative,
    NextDateUnconfirmed,
    Cancelled,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(try_from = "String")]
#[schemars(with = "String", extend("pattern" = "^[a-z]{2,3}(-[A-Za-z0-9]{2,8})*$"))]
pub(super) struct LanguageCode(pub(super) String);
impl TryFrom<String> for LanguageCode {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        let mut parts = value.split('-');
        let first = parts.next().unwrap_or_default();
        if !(2..=3).contains(&first.len())
            || !first.bytes().all(|b| b.is_ascii_lowercase())
            || parts.any(|p| {
                !(2..=8).contains(&p.len()) || !p.bytes().all(|b| b.is_ascii_alphanumeric())
            })
        {
            return Err(invalid(
                "language",
                "expected a language code such as en or pt-BR",
            ));
        }
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, JsonSchema)]
#[serde(try_from = "String")]
#[schemars(with = "String", extend("pattern" = "^C(0[1-9]|1[0-3])$"))]
pub(super) struct CategoryId(pub(super) String);
impl TryFrom<String> for CategoryId {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        if !(1..=13).any(|n| value == format!("C{n:02}")) {
            return Err(invalid("category", "expected C01 through C13"));
        }
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(try_from = "String")]
#[schemars(with = "String")]
pub(super) struct ReadingPath(pub(super) String);
impl TryFrom<String> for ReadingPath {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        if !value.starts_with("resources/related-talks/")
            || !value.ends_with(".md")
            || !Path::new(&value)
                .components()
                .all(|c| matches!(c, Component::Normal(_)))
            || !value
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"/.-".contains(&b))
        {
            return Err(invalid(
                "reading path",
                "expected a public session reading page",
            ));
        }
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(try_from = "String")]
#[schemars(with = "String")]
pub(super) struct Timezone(pub(super) Tz);
impl TryFrom<String> for Timezone {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        value
            .parse()
            .map(Self)
            .map_err(|_| invalid("timezone", "expected an IANA timezone"))
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum Deadline {
    Instant {
        at: Timestamp,
        timezone: Timezone,
        source_text: Summary,
    },
    Date {
        on: DeadlineDate,
        source_text: Summary,
    },
}
impl Deadline {
    pub(super) fn day(&self) -> NaiveDate {
        match self {
            Self::Instant { at, .. } => at.datetime().date_naive(),
            Self::Date { on, .. } => on.0,
        }
    }
    pub(super) fn source_text(&self) -> &str {
        match self {
            Self::Instant { source_text, .. } | Self::Date { source_text, .. } => {
                source_text.as_str()
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Review {
    pub checked_on: CheckedOn,
    pub next_review_on: ReviewDue,
    pub source_review: ReviewResult,
    pub evidence_urls: Vec<SourceUrl>,
    pub limits: Vec<Summary>,
}
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Access {
    pub mode: Mode,
    pub cost: Option<AccessNote>,
    pub spoken_languages: Option<Vec<LanguageCode>>,
    pub captions: Option<AccessNote>,
    pub eligibility: Option<AccessNote>,
    pub equipment: Option<SetupRequirement>,
    pub venue: Option<VenueName>,
    pub room: Option<StageName>,
}
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Activity {
    pub observed_on: Option<ObservedOn>,
    pub evidence_url: SourceUrl,
    pub note: Summary,
}
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Community {
    pub id: CommunityId,
    pub name: Title,
    pub kind: CommunityKind,
    pub affiliation: Affiliation,
    pub region: Region,
    pub location: Option<Address>,
    pub source_languages: Vec<LanguageCode>,
    pub topics: Vec<Topic>,
    pub url: SourceUrl,
    pub discovery_source_id: Option<DiscoverySourceId>,
    pub purpose: Summary,
    pub first_step: Summary,
    pub audience: Summary,
    pub contribution_url: Option<SourceUrl>,
    pub code_of_conduct_url: Option<SourceUrl>,
    pub access: Access,
    pub activity: Activity,
    pub review: Review,
}
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Opportunity {
    pub id: OpportunityId,
    pub title: Title,
    pub kind: OpportunityKind,
    pub status: OpportunityStatus,
    pub url: SourceUrl,
    pub community_id: Option<CommunityId>,
    pub event_id: Option<DiscoveryEventId>,
    pub deadline: Option<Deadline>,
    pub action: Summary,
    pub access: Access,
    pub review: Review,
}
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Route {
    pub id: RouteId,
    pub title: Title,
    pub session_ids: Vec<SessionId>,
    pub reading_path: ReadingPath,
    pub community_ids: Vec<CommunityId>,
    pub opportunity_ids: Vec<OpportunityId>,
    pub first_step: Summary,
    pub small_step: Summary,
    pub prerequisites: Prerequisite,
    pub relationship: Relationship,
}
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct EventReview {
    pub event_id: DiscoveryEventId,
    pub note: Summary,
    pub review: Review,
}
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Coverage {
    pub category: CategoryId,
    pub area: Title,
    pub status: CoverageStatus,
    pub note: Summary,
}
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Participation {
    pub schema_version: SchemaVersion,
    pub reference_date: ReferenceDate,
    pub calendar_updated_at: Timestamp,
    pub catalog_snapshot_id: SnapshotId,
    pub discovery_sha256: ContentHash,
    pub review_status: Summary,
    pub communities: Vec<Community>,
    pub opportunities: Vec<Opportunity>,
    pub routes: Vec<Route>,
    pub event_reviews: Vec<EventReview>,
    pub coverage: Vec<Coverage>,
    pub regional_gaps: Vec<Summary>,
}

// Narrow projections of the separately owned discovery file. New fields there
// do not change this companion; its exact bytes remain pinned by SHA-256.
#[derive(Debug, Deserialize)]
pub(super) struct Discovery {
    pub format_version: DiscoveryVersion,
    pub reference_date: ReferenceDate,
    pub events: Vec<DiscoveryEvent>,
    pub sources: Vec<DiscoverySource>,
}
#[derive(Debug, Deserialize)]
#[serde(try_from = "u64")]
pub(super) struct DiscoveryVersion;
impl TryFrom<u64> for DiscoveryVersion {
    type Error = Error;
    fn try_from(value: u64) -> Result<Self> {
        if value != 1 {
            return Err(invalid(
                "discovery version",
                "only format version 1 is supported",
            ));
        }
        Ok(Self)
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct DiscoverySource {
    pub id: DiscoverySourceId,
    pub url: SourceUrl,
}
#[derive(Debug, Deserialize)]
pub(super) struct DiscoveryEvent {
    pub id: DiscoveryEventId,
    pub name: Title,
    pub status: EventStatus,
    pub starts_on: Option<EventDate>,
    pub ends_on: Option<EventDate>,
    pub location: Option<Address>,
    pub venue: Option<VenueName>,
    pub url: SourceUrl,
    pub notes: Vec<Summary>,
    pub checked_on: CheckedOn,
    pub evidence_urls: Vec<SourceUrl>,
}
