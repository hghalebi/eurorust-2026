use crate::{Error, Result, domain::*, invalid};
use chrono::{Datelike, NaiveDate};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::{Component, Path};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "String", into = "String")]
#[schemars(with = "String", extend("pattern" = "^[a-z0-9]+(-[a-z0-9]+)*$"))]
pub(super) struct ResourceId(String);
impl TryFrom<String> for ResourceId {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        if value.split('-').any(|p| {
            p.is_empty()
                || !p
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        }) {
            return Err(invalid(
                "resource ID",
                "expected lowercase words separated by hyphens",
            ));
        }
        Ok(Self(value))
    }
}
impl From<ResourceId> for String {
    fn from(value: ResourceId) -> Self {
        value.0
    }
}
impl ResourceId {
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "String", into = "String")]
#[schemars(with = "String")]
pub(super) struct PublicPath(String);
impl TryFrom<String> for PublicPath {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        let valid_root = [
            "docs/",
            "resources/",
            "templates/",
            "prompts/",
            "community/",
            "data/",
            "my-conference/",
        ]
        .iter()
        .any(|prefix| value.starts_with(prefix));
        if !valid_root
            || !(value.ends_with(".md") || value.ends_with(".json"))
            || !Path::new(&value)
                .components()
                .all(|c| matches!(c, Component::Normal(_)))
            || value.split('/').any(|c| c.starts_with('.'))
            || !value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"/.-_".contains(&b))
        {
            return Err(invalid(
                "public resource path",
                "expected a contained public Markdown or JSON path",
            ));
        }
        Ok(Self(value))
    }
}
impl From<PublicPath> for String {
    fn from(value: PublicPath) -> Self {
        value.0
    }
}
impl PublicPath {
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

macro_rules! dates {
    ($($name:ident),+) => {$(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
        #[serde(try_from = "String", into = "String")]
        #[schemars(with = "String", extend("format" = "date"))]
        pub(super) struct $name(pub(super) NaiveDate);
        impl TryFrom<String> for $name {
            type Error = Error;
            fn try_from(value: String) -> Result<Self> {
                let date = NaiveDate::parse_from_str(&value, "%Y-%m-%d").map_err(|e| invalid("date", e.to_string()))?;
                if !(1900..=9999).contains(&date.year()) || date.to_string() != value {
                    return Err(invalid("date", "expected YYYY-MM-DD from 1900"));
                }
                Ok(Self(date))
            }
        }
        impl From<$name> for String { fn from(value: $name) -> Self { value.0.to_string() } }
    )+};
}
dates!(ReferenceDate, CheckedOn, ReviewDue, PublishedOn);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub(super) enum Audience {
    Speaker,
    History,
    Attendee,
    Organizer,
    Community,
    Agent,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(super) enum ResourceKind {
    Guide,
    Budget,
    Retrospective,
    Documentation,
    Directory,
    SpeakerNotes,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(super) enum Access {
    FreeWebPage,
    Unknown,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(super) enum ReviewStatus {
    PageChecked,
    Unavailable,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(super) enum ReferenceContext {
    Historical,
    GeneralGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "String", into = "String")]
#[schemars(with = "String", extend("pattern" = "^[a-z]{2,3}(-[A-Za-z0-9]{2,8})*$"))]
pub(super) struct Language(String);
impl TryFrom<String> for Language {
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
impl From<Language> for String {
    fn from(value: Language) -> Self {
        value.0
    }
}
impl Language {
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Review {
    pub checked_on: CheckedOn,
    pub next_review_on: ReviewDue,
    pub status: ReviewStatus,
    pub limits: Vec<Summary>,
}
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Resource {
    pub id: ResourceId,
    pub title: Title,
    pub url: SourceUrl,
    pub publisher: Author,
    pub audiences: Vec<Audience>,
    pub kind: ResourceKind,
    pub language: Language,
    pub access: Access,
    pub reuse: Summary,
    pub why: Summary,
    pub action: Summary,
    pub context: ReferenceContext,
    pub published_on: Option<PublishedOn>,
    pub review: Review,
}
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Route {
    pub audience: Audience,
    pub title: Title,
    pub guide: PublicPath,
    pub example: PublicPath,
    pub template: PublicPath,
    pub outcome: Summary,
    pub steps: Vec<Summary>,
    pub resource_ids: Vec<ResourceId>,
}
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct Collection {
    pub schema_version: SchemaVersion,
    pub reference_date: ReferenceDate,
    pub catalog_snapshot_id: SnapshotId,
    pub maintainer: Author,
    pub routes: Vec<Route>,
    pub resources: Vec<Resource>,
    pub related_collections: Vec<PublicPath>,
}
