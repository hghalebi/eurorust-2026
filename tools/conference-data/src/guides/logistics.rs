//! Read the fields needed by the day guide; preserve the full companion by hash.
use super::*;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(transparent)]
struct LogisticsVenueKey(ResourceId);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(transparent)]
struct LogisticsSourceId(ResourceId);

// These are deliberate read projections of a separately owned companion.
// Additional fields stay in that file and are not republished as new claims.
#[derive(Deserialize)]
struct Logistics {
    format_version: SchemaVersion,
    catalog_snapshot_id: SnapshotId,
    checked_at: Timestamp,
    venues: Vec<Venue>,
    sessions: Vec<SessionLocation>,
    sources: BTreeMap<LogisticsSourceId, Source>,
}
#[derive(Deserialize)]
struct Venue {
    key: LogisticsVenueKey,
    name: VenueName,
    source_ids: Vec<LogisticsSourceId>,
}
#[derive(Deserialize)]
struct SessionLocation {
    session_id: SessionId,
    slot: Slot,
    stage: Option<StageName>,
    venue_key: Option<LogisticsVenueKey>,
    source_ids: Vec<LogisticsSourceId>,
}
#[derive(Deserialize)]
struct Source {
    source_url: SourceUrl,
    retrieved_at: Timestamp,
    content_sha256: ContentHash,
}

pub(super) struct CheckedLogistics {
    checked_at: Timestamp,
    venues: BTreeMap<SessionId, Option<VenueName>>,
}

pub(super) fn load(bytes: &[u8], bundle: &export::VerifiedBundle) -> Result<CheckedLogistics> {
    let data: Logistics = serde_json::from_slice(bytes)?;
    if data.format_version.as_str() != "1.0.0"
        || data.catalog_snapshot_id != bundle.manifest().snapshot_id
    {
        return Err(invalid(
            "day guide logistics",
            "changed snapshot or format; review the separate logistics companion",
        ));
    }
    distinct(data.venues.iter().map(|v| &v.key), "logistics venue")?;
    distinct(
        data.sessions.iter().map(|s| &s.session_id),
        "logistics session",
    )?;
    if data.sessions.len() != bundle.catalog().sessions.len() || data.sources.is_empty() {
        return Err(invalid(
            "logistics coverage",
            "each catalog session needs a source-backed location record",
        ));
    }
    let check_sources = |ids: &[LogisticsSourceId]| -> Result<()> {
        distinct(ids.iter(), "logistics evidence")?;
        if ids.is_empty() || ids.iter().any(|id| !data.sources.contains_key(id)) {
            return Err(invalid(
                "logistics evidence",
                "missing or unknown source reference",
            ));
        }
        Ok(())
    };
    for source in data.sources.values() {
        // Typed decoding validates URLs and hashes even though only provenance
        // links in the existing venue guide are rendered here.
        let _ = (&source.source_url, &source.content_sha256);
        if source.retrieved_at > data.checked_at {
            return Err(invalid(
                "logistics evidence",
                "source retrieved after the recorded review",
            ));
        }
    }
    for venue in &data.venues {
        check_sources(&venue.source_ids)?;
    }
    let mut venues = BTreeMap::new();
    for row in &data.sessions {
        check_sources(&row.source_ids)?;
        let session = bundle
            .catalog()
            .sessions
            .iter()
            .find(|s| s.id == row.session_id)
            .ok_or_else(|| invalid("logistics session", "unknown catalog session"))?;
        if session.slot.start() != row.slot.start()
            || session.slot.end() != row.slot.end()
            || session.stage != row.stage
        {
            return Err(invalid(
                "logistics session",
                "time or stage differs from the reviewed catalog",
            ));
        }
        let venue = row
            .venue_key
            .as_ref()
            .map(|key| {
                data.venues
                    .iter()
                    .find(|v| &v.key == key)
                    .map(|v| v.name.clone())
                    .ok_or_else(|| invalid("logistics venue", "unknown venue key"))
            })
            .transpose()?;
        venues.insert(row.session_id.clone(), venue);
    }
    Ok(CheckedLogistics {
        checked_at: data.checked_at,
        venues,
    })
}

impl CheckedLogistics {
    pub(super) fn checked_at(&self) -> &Timestamp {
        &self.checked_at
    }
    pub(super) fn venue(&self, id: &SessionId) -> Option<&VenueName> {
        self.venues.get(id).and_then(Option::as_ref)
    }
}
