//! Sourced participation records and deterministic human and calendar views.
mod calendar;
mod model;
mod views;

use crate::{Error, Result, domain::*, export, invalid};
use chrono::{Datelike, Offset};
use model::*;
use std::{collections::BTreeSet, fs, io::Write, path::Path};

/// Only a complete, checked input set can render public views.
/// ```compile_fail
/// use conference_data::participation::CheckedParticipation;
/// let checked = CheckedParticipation {};
/// ```
/// ```no_run
/// use conference_data::participation;
/// let checked = participation::load(std::path::Path::new("."))?;
/// checked.verify(std::path::Path::new("."))?;
/// # Ok::<(), conference_data::Error>(())
/// ```
pub struct CheckedParticipation {
    data: Participation,
    discovery: Discovery,
}

fn unique<'a, T: Ord + ?Sized + 'a>(
    values: impl Iterator<Item = &'a T>,
    kind: &'static str,
) -> Result<BTreeSet<&'a T>> {
    let mut found = BTreeSet::new();
    for value in values {
        if !found.insert(value) {
            return Err(invalid(kind, "duplicate identity or reference"));
        }
    }
    Ok(found)
}

fn check_day(day: chrono::NaiveDate) -> Result<()> {
    if !(1900..=9999).contains(&day.year()) {
        return Err(invalid("date", "expected a four-digit year from 1900"));
    }
    Ok(())
}

fn check_review(review: &Review, reference: ReferenceDate) -> Result<()> {
    check_day(review.checked_on.0)?;
    check_day(review.next_review_on.0)?;
    if review.evidence_urls.is_empty()
        || review.checked_on.0 > reference.0
        || review.next_review_on.0 < reference.0
        || review.next_review_on.0 <= review.checked_on.0
        || (review.next_review_on.0 - review.checked_on.0).num_days() > 90
        || (review.source_review == ReviewResult::Unavailable && review.limits.is_empty())
    {
        return Err(invalid(
            "review",
            format!(
                "missing evidence, invalid review interval, or unexplained unavailable source: {} (checked {}, due {})",
                review
                    .evidence_urls
                    .first()
                    .map(SourceUrl::as_str)
                    .unwrap_or("no source"),
                review.checked_on.0,
                review.next_review_on.0
            ),
        ));
    }
    unique(
        review.evidence_urls.iter().map(SourceUrl::as_str),
        "evidence URL",
    )?;
    Ok(())
}

fn check_access(access: &Access) -> Result<()> {
    if access.spoken_languages.as_ref().is_some_and(Vec::is_empty)
        || (access.room.is_some() && access.venue.is_none())
    {
        return Err(invalid(
            "access",
            "use null for unknown languages; a room needs a venue",
        ));
    }
    Ok(())
}

fn check_deadline(deadline: &Deadline) -> Result<()> {
    check_day(deadline.day())?;
    if let Deadline::Instant { at, timezone, .. } = deadline {
        let local = at.datetime().with_timezone(&timezone.0);
        if local.offset().fix() != *at.datetime().offset()
            || at.datetime().timestamp_subsec_nanos() != 0
        {
            return Err(invalid(
                "deadline",
                "UTC offset disagrees with the named timezone on this date",
            ));
        }
    }
    Ok(())
}

fn check_due(
    review: &Review,
    relevant: Option<chrono::NaiveDate>,
    reference: ReferenceDate,
) -> Result<()> {
    let near = relevant.is_some_and(|day| (0..=30).contains(&(day - reference.0).num_days()));
    let maximum = if near { 7 } else { 30 };
    if (review.next_review_on.0 - review.checked_on.0).num_days() > maximum {
        return Err(invalid(
            "review due",
            format!(
                "review interval exceeds {maximum} days for {}",
                review
                    .evidence_urls
                    .first()
                    .map(SourceUrl::as_str)
                    .unwrap_or("no source")
            ),
        ));
    }
    Ok(())
}

fn validate(
    data: &Participation,
    discovery: &Discovery,
    catalog: &Catalog,
    root: &Path,
) -> Result<()> {
    check_day(data.reference_date.0)?;
    if data.schema_version.as_str() != "1.0.0"
        || discovery.reference_date.0 > data.reference_date.0
        || data.calendar_updated_at.datetime().date_naive() < data.reference_date.0
        || data.communities.is_empty()
        || data.opportunities.is_empty()
        || data.routes.is_empty()
    {
        return Err(invalid(
            "participation",
            "unsupported version, invalid reference date, or empty collection",
        ));
    }
    let DiscoveryVersion = discovery.format_version;
    let communities = unique(data.communities.iter().map(|c| &c.id), "community")?;
    let opportunities = unique(data.opportunities.iter().map(|o| &o.id), "opportunity")?;
    let events = unique(discovery.events.iter().map(|e| &e.id), "event")?;
    unique(discovery.sources.iter().map(|s| &s.id), "discovery source")?;
    unique(data.routes.iter().map(|r| &r.id), "route")?;
    unique(
        data.communities.iter().map(|c| c.url.as_str()),
        "community URL",
    )?;
    let reviews = unique(
        data.event_reviews.iter().map(|r| &r.event_id),
        "event review",
    )?;
    if events != reviews {
        return Err(invalid(
            "event reviews",
            "each discovery event needs exactly one participation review",
        ));
    }
    let categories = unique(data.coverage.iter().map(|c| &c.category), "coverage")?;
    if categories.len() != 13 {
        return Err(invalid(
            "coverage",
            "describe coverage or a gap for all thirteen categories",
        ));
    }
    for community in &data.communities {
        check_review(&community.review, data.reference_date)?;
        check_access(&community.access)?;
        if community.source_languages.is_empty() || community.topics.is_empty() {
            return Err(invalid(
                "community",
                "page languages and useful topic connections are required",
            ));
        }
        unique(community.topics.iter(), "community topic")?;
        for topic in &community.topics {
            let slug = RouteId::try_from(topic.as_str().to_owned())?;
            if !root
                .join(format!("resources/topics/{}.md", slug.0))
                .is_file()
            {
                return Err(invalid("community topic", "unknown topic page"));
            }
        }
        if let Some(id) = &community.discovery_source_id
            && !discovery
                .sources
                .iter()
                .any(|s| &s.id == id && s.url == community.url)
        {
            return Err(invalid(
                "discovery source",
                "missing source or different canonical URL",
            ));
        }
        if let Some(date) = community.activity.observed_on {
            check_day(date.0)?;
            if date.0 > community.review.checked_on.0 {
                return Err(invalid(
                    "activity",
                    "an observed activity cannot be in the future",
                ));
            }
        }
    }
    for opportunity in &data.opportunities {
        check_review(&opportunity.review, data.reference_date)?;
        check_access(&opportunity.access)?;
        if opportunity
            .community_id
            .as_ref()
            .is_some_and(|id| !communities.contains(id))
            || opportunity
                .event_id
                .as_ref()
                .is_some_and(|id| !events.contains(id))
        {
            return Err(invalid(
                "opportunity",
                "unknown community or event reference",
            ));
        }
        if opportunity.status == OpportunityStatus::Open
            && opportunity.review.source_review != ReviewResult::PageChecked
        {
            return Err(invalid(
                "opportunity",
                "an open call needs an inspected source",
            ));
        }
        if let Some(deadline) = &opportunity.deadline {
            check_deadline(deadline)?;
            if opportunity.status == OpportunityStatus::Open
                && deadline.day() < data.reference_date.0
            {
                return Err(invalid("opportunity", "past deadline still labelled open"));
            }
        }
        check_due(
            &opportunity.review,
            opportunity.deadline.as_ref().map(Deadline::day),
            data.reference_date,
        )?;
    }
    for route in &data.routes {
        if route.session_ids.is_empty()
            || route.community_ids.is_empty()
            || route.opportunity_ids.is_empty()
        {
            return Err(invalid(
                "route",
                "connect at least one session, community, and action",
            ));
        }
        unique(route.session_ids.iter(), "route session")?;
        unique(route.community_ids.iter(), "route community")?;
        unique(route.opportunity_ids.iter(), "route opportunity")?;
        if route
            .session_ids
            .iter()
            .any(|id| !catalog.sessions.iter().any(|s| &s.id == id))
            || route
                .community_ids
                .iter()
                .any(|id| !communities.contains(id))
            || route
                .opportunity_ids
                .iter()
                .any(|id| !opportunities.contains(id))
        {
            return Err(invalid(
                "route",
                "unknown session, community, or opportunity",
            ));
        }
        if !route
            .session_ids
            .iter()
            .any(|id| route.reading_path.0 == format!("resources/related-talks/{}.md", id.as_str()))
        {
            return Err(invalid(
                "reading path",
                "must belong to a referenced session",
            ));
        }
        let page = root.join(&route.reading_path.0).canonicalize()?;
        if !page.starts_with(root.canonicalize()?) || !page.is_file() {
            return Err(invalid(
                "reading path",
                "expected a file inside this checkout",
            ));
        }
        // Each suggested action must connect to a community on the same route.
        for id in &route.opportunity_ids {
            let action = data.opportunities.iter().find(|o| &o.id == id).unwrap();
            if !action
                .community_id
                .as_ref()
                .is_some_and(|c| route.community_ids.contains(c))
            {
                return Err(invalid("route", "action belongs to a different community"));
            }
        }
    }
    for event in &discovery.events {
        if event.evidence_urls.is_empty() || event.checked_on.0 > discovery.reference_date.0 {
            return Err(invalid("event", "missing evidence or future source check"));
        }
        match (event.starts_on, event.ends_on) {
            (Some(start), Some(end)) => {
                check_day(start.0)?;
                check_day(end.0)?;
                if start > end || end.0.succ_opt().is_none_or(|d| d.year() > 9999) {
                    return Err(invalid(
                        "event dates",
                        "reversed or unrepresentable date range",
                    ));
                }
            }
            (None, None)
                if !matches!(event.status, EventStatus::Announced | EventStatus::Ongoing) => {}
            _ => {
                return Err(invalid(
                    "event dates",
                    "dated events need both start and inclusive end",
                ));
            }
        }
        let reviewed = data
            .event_reviews
            .iter()
            .find(|r| r.event_id == event.id)
            .unwrap();
        check_review(&reviewed.review, data.reference_date)?;
        check_due(
            &reviewed.review,
            event.starts_on.map(|d| d.0),
            data.reference_date,
        )?;
    }
    Ok(())
}

/// Read pinned sources and validate references before making views available.
pub fn load(root: &Path) -> Result<CheckedParticipation> {
    let data: Participation =
        serde_json::from_slice(&fs::read(root.join("community/participation.json"))?)?;
    let bytes = fs::read(root.join("community/rust-discovery.json"))?;
    if ContentHash::from_content(&bytes) != data.discovery_sha256 {
        return Err(invalid(
            "discovery pin",
            "discovery changed; review the companion before updating its hash",
        ));
    }
    let discovery: Discovery = serde_json::from_slice(&bytes)?;
    let bundle = export::verify(root)?;
    if bundle.manifest().snapshot_id != data.catalog_snapshot_id {
        return Err(invalid(
            "catalog pin",
            "session snapshot changed; review route connections",
        ));
    }
    validate(&data, &discovery, bundle.catalog(), root)?;
    Ok(CheckedParticipation { data, discovery })
}

impl CheckedParticipation {
    /// Replace individual outputs atomically, after computing all views successfully.
    /// A failed filesystem write may leave a mixed set; verify detects it and a
    /// successful rebuild repairs it. Invalid input causes no writes.
    pub fn render(&self, root: &Path) -> Result<()> {
        let views = self.views()?;
        for (relative, bytes) in views {
            let path = root.join(relative);
            let parent = path
                .parent()
                .ok_or_else(|| invalid("output", "missing parent"))?;
            fs::create_dir_all(parent)?;
            let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
            temporary.write_all(&bytes)?;
            temporary.persist(path).map_err(|e| e.error)?;
        }
        Ok(())
    }

    /// Check reproducibility offline; this does not establish current availability.
    pub fn verify(&self, root: &Path) -> Result<()> {
        for (relative, expected) in self.views()? {
            if fs::read(root.join(relative))? != expected {
                return Err(Error::Validation(format!(
                    "stale participation view: {relative}; run build-participation"
                )));
            }
        }
        println!(
            "Checked {} communities, {} opportunities, {} routes, {} event reviews, and calendar views; offline only",
            self.data.communities.len(),
            self.data.opportunities.len(),
            self.data.routes.len(),
            self.data.event_reviews.len()
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests;
