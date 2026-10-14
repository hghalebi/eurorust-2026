use crate::{Error, Result, domain::*, export};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub name: ProfileName,
    pub goals: Vec<Goal>,
    pub topics: Vec<Topic>,
    pub available_windows: Vec<Slot>,
    pub registered_workshops: Vec<SessionId>,
    pub attendance_mode: Option<AttendanceMode>,
}
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttendanceMode {
    InPerson,
    Remote,
}
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Selection {
    pub session_id: SessionId,
    pub rationale: Rationale,
    pub alternatives: Vec<SessionId>,
}
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Itinerary {
    pub snapshot_id: SnapshotId,
    pub profile: Profile,
    pub selections: Vec<Selection>,
}

pub fn recommend(snapshot: &export::VerifiedBundle, profile: Profile) -> Result<Itinerary> {
    let catalog = snapshot.catalog();
    validate_profile(catalog, &profile)?;
    let mut groups = std::collections::BTreeMap::new();
    for session in &catalog.sessions {
        if session.kind != SessionKind::Talk && !profile.registered_workshops.contains(&session.id)
        {
            continue;
        }
        if !profile.available_windows.is_empty()
            && !profile.available_windows.iter().any(|w| {
                w.start() <= session.slot.start()
                    && w.end().is_some_and(|end| {
                        session
                            .slot
                            .end()
                            .is_some_and(|session_end| session_end <= end)
                    })
            })
        {
            continue;
        }
        let mut score = 0;
        let mut matched = Vec::new();
        for topic in &profile.topics {
            if session.annotation.as_ref().is_some_and(|a| {
                a.review_status != AnnotationReview::SourceChanged && a.topics.contains(topic)
            }) || session
                .title
                .as_str()
                .to_lowercase()
                .contains(&topic.as_str().to_lowercase())
            {
                score += 1;
                matched.push(topic.as_str());
            }
        }
        groups
            .entry(session.slot.start().clone())
            .or_insert_with(Vec::new)
            .push((score, session, matched));
    }
    let mut selections = Vec::new();
    let mut last_end: Option<&Timestamp> = None;
    for (_, mut candidates) in groups {
        candidates.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.id.cmp(&b.1.id)));
        let (_, first, matched) = &candidates[0];
        if last_end.is_some_and(|end| first.slot.start() < end) {
            continue;
        }
        if first.slot.end().is_none() {
            continue;
        }
        let reason = if matched.is_empty() {
            "Shared session or no stronger topic match; review its official abstract".to_owned()
        } else {
            format!("Matches requested topics: {}", matched.join(", "))
        };
        selections.push(Selection {
            session_id: first.id.clone(),
            rationale: reason.try_into()?,
            alternatives: candidates
                .iter()
                .skip(1)
                .map(|(_, s, _)| s.id.clone())
                .collect(),
        });
        last_end = first.slot.end();
    }
    let itinerary = Itinerary {
        snapshot_id: snapshot.manifest().snapshot_id.clone(),
        profile,
        selections,
    };
    check(snapshot, &itinerary)?;
    Ok(itinerary)
}

/// Validate schedule feasibility, not booking, prerequisite readiness, or attendance.
pub fn check(snapshot: &export::VerifiedBundle, itinerary: &Itinerary) -> Result<Vec<String>> {
    validate_profile(snapshot.catalog(), &itinerary.profile)?;
    if snapshot.manifest().snapshot_id != itinerary.snapshot_id {
        return Err(Error::Validation(
            "itinerary belongs to a different snapshot; retain that snapshot or regenerate".into(),
        ));
    }
    let mut seen = BTreeSet::new();
    let mut selected = Vec::new();
    let mut warnings = vec![
        "Prerequisite readiness and ticket availability require participant confirmation."
            .to_owned(),
    ];
    if itinerary.profile.available_windows.is_empty() {
        warnings.push(
            "Availability unknown: all conference slots were considered conditionally.".into(),
        );
    }
    if itinerary.profile.attendance_mode.is_none() {
        warnings
            .push("Attendance mode unknown; per-session remote access is not confirmed.".into());
    }
    for selection in &itinerary.selections {
        if !seen.insert(&selection.session_id) {
            return Err(Error::Validation("duplicate itinerary selection".into()));
        }
        let session = snapshot
            .catalog()
            .sessions
            .iter()
            .find(|s| s.id == selection.session_id)
            .ok_or_else(|| {
                Error::Validation(format!("unknown session {}", selection.session_id.as_str()))
            })?;
        for alternative in &selection.alternatives {
            let other = snapshot
                .catalog()
                .sessions
                .iter()
                .find(|s| &s.id == alternative)
                .ok_or_else(|| Error::Validation("unknown alternative".into()))?;
            if other.id == session.id || other.slot.start() != session.slot.start() {
                return Err(Error::Validation(
                    "alternative must be another session in the same slot".into(),
                ));
            }
        }
        if session.kind == SessionKind::Workshop
            && !itinerary.profile.registered_workshops.contains(&session.id)
        {
            warnings.push(format!(
                "{}: workshop registration unconfirmed",
                session.id.as_str()
            ));
        }
        if session
            .annotation
            .as_ref()
            .is_some_and(|a| a.review_status == AnnotationReview::SourceChanged)
        {
            warnings.push(format!(
                "{}: annotation source changed; review current prerequisites and summary",
                session.id.as_str()
            ));
        }
        if session.slot.end().is_none() {
            return Err(Error::Validation(format!(
                "cannot verify overlap for unknown end: {}",
                session.id.as_str()
            )));
        }
        if !itinerary.profile.available_windows.is_empty()
            && !itinerary.profile.available_windows.iter().any(|w| {
                w.start() <= session.slot.start()
                    && w.end()
                        .is_some_and(|e| session.slot.end().is_some_and(|end| end <= e))
            })
        {
            return Err(Error::Validation(
                "selection outside participant availability".into(),
            ));
        }
        if matches!(
            itinerary.profile.attendance_mode,
            Some(AttendanceMode::Remote)
        ) && session.remote_access.is_none()
        {
            warnings.push(format!(
                "{}: individual remote access unknown",
                session.id.as_str()
            ));
        }
        selected.push(session);
    }
    selected.sort_by_key(|s| s.slot.start());
    for pair in selected.windows(2) {
        if pair[0]
            .slot
            .end()
            .is_some_and(|end| end > pair[1].slot.start())
        {
            return Err(Error::Validation(format!(
                "overlap: {} and {}",
                pair[0].id.as_str(),
                pair[1].id.as_str()
            )));
        }
        if pair[0].venue_id != pair[1].venue_id {
            warnings.push(format!(
                "Transfer between {} and {} requires participant-confirmed travel time.",
                pair[0].id.as_str(),
                pair[1].id.as_str()
            ));
        }
    }
    Ok(warnings)
}

fn validate_profile(catalog: &Catalog, profile: &Profile) -> Result<()> {
    if profile
        .available_windows
        .iter()
        .any(|window| window.end().is_none())
    {
        return Err(Error::Validation(
            "availability windows require a known end; use an empty list for unknown availability"
                .into(),
        ));
    }
    let mut registered = BTreeSet::new();
    for id in &profile.registered_workshops {
        if !registered.insert(id)
            || !catalog
                .sessions
                .iter()
                .any(|s| &s.id == id && s.kind == SessionKind::Workshop)
        {
            return Err(Error::Validation(
                "registered workshop must be a distinct known workshop ID".into(),
            ));
        }
    }
    Ok(())
}
