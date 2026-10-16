//! Calendar adapters preserve public source facts and omit participant details.
use crate::{Result, domain::*, export::VerifiedBundle, invalid, itinerary};
use chrono::Utc;
use std::{fs, io::Write, path::Path};

pub(crate) fn text(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace('\n', "\\n")
        .replace(';', "\\;")
        .replace(',', "\\,")
}

// RFC 5545 section 3.1: 75 octets, including continuation whitespace.
pub(crate) fn line(output: &mut String, value: &str) {
    let mut width = 0;
    for ch in value.chars() {
        if width + ch.len_utf8() > 75 {
            output.push_str("\r\n ");
            width = 1;
        }
        output.push(ch);
        width += ch.len_utf8();
    }
    output.push_str("\r\n");
}

/// A calendar can be written only after its itinerary and known slot ends pass checks.
/// It proves snapshot consistency, not bookings, access, attendance, or travel feasibility.
/// ```compile_fail
/// use conference_data::calendar::CheckedCalendar;
/// let calendar = CheckedCalendar { content: String::new() };
/// ```
/// ```no_run
/// let root = std::path::Path::new(".");
/// let bundle = conference_data::export::verify(root)?;
/// let plan = serde_json::from_slice(&std::fs::read(".plan/itinerary.json")?)?;
/// conference_data::calendar::from_itinerary(&bundle, &plan)?.write(root.join(".plan/selected.ics").as_path())?;
/// # Ok::<(), conference_data::Error>(())
/// ```
pub struct CheckedCalendar {
    content: String,
}

pub fn from_itinerary(
    snapshot: &VerifiedBundle,
    plan: &itinerary::Itinerary,
) -> Result<CheckedCalendar> {
    itinerary::check(snapshot, plan)?;
    if plan.selections.is_empty() {
        return Err(invalid("calendar", "select at least one session"));
    }
    let catalog = snapshot.catalog();
    let mut content = String::new();
    for value in [
        "BEGIN:VCALENDAR",
        "VERSION:2.0",
        "PRODID:-//hghalebi/eurorust-2026//Selected sessions//EN",
        "CALSCALE:GREGORIAN",
    ] {
        line(&mut content, value);
    }
    for selection in &plan.selections {
        let session = catalog
            .sessions
            .iter()
            .find(|s| s.id == selection.session_id)
            .ok_or_else(|| invalid("calendar", "selection missing from checked snapshot"))?;
        let end = session
            .slot
            .end()
            .ok_or_else(|| invalid("calendar", "session end is unknown"))?;
        let stamp = |time: &Timestamp| {
            time.datetime()
                .with_timezone(&Utc)
                .format("%Y%m%dT%H%M%SZ")
                .to_string()
        };
        // Only public catalog facts enter the output. Never serialize a profile or rationale.
        let location = session
            .venue_id
            .as_ref()
            .and_then(|id| catalog.venues.iter().find(|v| &v.id == id))
            .map(|v| v.name.as_str())
            .unwrap_or("Venue unconfirmed");
        let description = format!(
            "Tentative personal selection; confirm tickets, access, setup, room and travel time with the organizer. No booking or attendance is recorded. Stage label: {} (not a verified physical room). Snapshot: {}. Schedule retrieved: {}. Source: {}",
            session.stage.as_ref().map_or("unknown", StageName::as_str),
            snapshot.manifest().snapshot_id.as_str(),
            session.source.retrieved_at.as_rfc3339(),
            session.official_url.as_str()
        );
        for value in [
            "BEGIN:VEVENT".to_owned(),
            format!(
                "UID:session-{}@eurorust-2026.hghalebi.github",
                session.id.as_str()
            ),
            format!("DTSTAMP:{}", stamp(&session.source.retrieved_at)),
            format!("DTSTART:{}", stamp(session.slot.start())),
            format!("DTEND:{}", stamp(end)),
            format!("SUMMARY:{}", text(session.title.as_str())),
            format!("URL:{}", session.official_url.as_str()),
            format!("LOCATION:{}", text(location)),
            format!("DESCRIPTION:{}", text(&description)),
            "STATUS:TENTATIVE".to_owned(),
            "TRANSP:TRANSPARENT".to_owned(),
            "END:VEVENT".to_owned(),
        ] {
            line(&mut content, &value);
        }
    }
    line(&mut content, "END:VCALENDAR");
    Ok(CheckedCalendar { content })
}

impl CheckedCalendar {
    pub fn as_ics(&self) -> &str {
        &self.content
    }

    /// Atomic replacement of one file. Invalid input never reaches this operation.
    pub fn write(&self, output: &Path) -> Result<()> {
        let parent = output
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        fs::create_dir_all(parent)?;
        let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
        temporary.write_all(self.content.as_bytes())?;
        temporary.persist(output).map_err(|e| e.error)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn selected_calendar_omits_profile_and_alternatives_and_rejects_stale_input() {
        let root = root();
        let bundle = crate::export::verify(&root).unwrap();
        let mut plan: itinerary::Itinerary =
            serde_json::from_slice(&fs::read(root.join("my-conference/itinerary.json")).unwrap())
                .unwrap();
        plan.profile.name = "PRIVATE-NAME".to_owned().try_into().unwrap();
        plan.profile.goals = vec!["PRIVATE-GOAL".to_owned().try_into().unwrap()];
        for selection in &mut plan.selections {
            selection.rationale = "PRIVATE-RATIONALE".to_owned().try_into().unwrap();
        }
        let calendar = from_itinerary(&bundle, &plan).unwrap();
        let unfolded = calendar.as_ics().replace("\r\n ", "");
        assert!(!unfolded.contains("PRIVATE-"));
        assert_eq!(
            unfolded.matches("BEGIN:VEVENT").count(),
            plan.selections.len()
        );
        assert_eq!(
            unfolded.matches("STATUS:TENTATIVE").count(),
            plan.selections.len()
        );
        for choice in &plan.selections {
            for alternative in &choice.alternatives {
                assert!(!unfolded.contains(&format!("UID:session-{}@", alternative.as_str())));
            }
        }
        let directory = tempfile::tempdir().unwrap();
        let output = directory.path().join("selected.ics");
        calendar.write(&output).unwrap();
        plan.snapshot_id = "unreviewed-snapshot".to_owned().try_into().unwrap();
        assert!(
            from_itinerary(&bundle, &plan)
                .and_then(|c| c.write(&output))
                .is_err()
        );
        assert_eq!(fs::read_to_string(output).unwrap(), calendar.as_ics());
    }
}
