//! RFC 5545 snapshot: all-day event spans and separately labelled deadlines.
use super::{CheckedParticipation, model::*};
use chrono::Utc;

fn text(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace('\n', "\\n")
        .replace(';', "\\;")
        .replace(',', "\\,")
}

// RFC 5545 section 3.1: 75 octets including continuation whitespace.
// Split only at UTF-8 boundaries; unfolding must recover the exact content.
fn line(output: &mut String, value: &str) {
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

impl CheckedParticipation {
    pub(super) fn calendar(&self) -> String {
        let mut lines = vec![
            "BEGIN:VCALENDAR".to_owned(),
            "VERSION:2.0".to_owned(),
            "PRODID:-//hghalebi/eurorust-2026//Rust community snapshot//EN".to_owned(),
            "CALSCALE:GREGORIAN".to_owned(),
        ];
        let stamp = self
            .data
            .calendar_updated_at
            .datetime()
            .with_timezone(&Utc)
            .format("%Y%m%dT%H%M%SZ")
            .to_string();
        for event in &self.discovery.events {
            if !matches!(event.status, EventStatus::Announced | EventStatus::Ongoing) {
                continue;
            }
            let (Some(start), Some(end)) = (event.starts_on, event.ends_on) else {
                continue;
            };
            let review = &self
                .data
                .event_reviews
                .iter()
                .find(|r| r.event_id == event.id)
                .unwrap()
                .review;
            if end.0 < self.data.reference_date.0
                || review.source_review != ReviewResult::PageChecked
            {
                continue;
            }
            let notes = event
                .notes
                .iter()
                .map(|n| n.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            lines.extend([
                "BEGIN:VEVENT".to_owned(),
                format!("UID:event-{}@eurorust-2026.hghalebi.github", event.id.0),
                format!("DTSTAMP:{stamp}"),
                format!("DTSTART;VALUE=DATE:{}", start.0.format("%Y%m%d")),
                // Inclusive source end becomes exclusive calendar end.
                format!("DTEND;VALUE=DATE:{}", end.0.succ_opt().unwrap().format("%Y%m%d")),
                format!("SUMMARY:{}", text(event.name.as_str())),
                format!("URL:{}", event.url.as_str()),
                format!("DESCRIPTION:{}", text(&format!(
                    "Independent directory snapshot {}. Event dates checked {}. Participation page checked {}. Review due {}. Dates only; session times, rooms, registration, and attendance are not established by this entry. {} Source: {}",
                    self.data.reference_date.0, event.checked_on.0, review.checked_on.0,
                    review.next_review_on.0, notes, event.url.as_str()))),
                "TRANSP:TRANSPARENT".to_owned(),
            ]);
            let location = [
                event.venue.as_ref().map(|v| v.as_str()),
                event.location.as_ref().map(|l| l.as_str()),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(", ");
            if !location.is_empty() {
                lines.push(format!("LOCATION:{}", text(&location)));
            }
            lines.push("END:VEVENT".to_owned());
        }
        for opportunity in &self.data.opportunities {
            if opportunity.status != OpportunityStatus::Open {
                continue;
            }
            let Some(deadline) = &opportunity.deadline else {
                continue;
            };
            lines.extend([
                "BEGIN:VEVENT".to_owned(),
                format!(
                    "UID:deadline-{}@eurorust-2026.hghalebi.github",
                    opportunity.id.0
                ),
                format!("DTSTAMP:{stamp}"),
            ]);
            let precision = match deadline {
                Deadline::Instant { at, timezone, .. } => {
                    lines.push(format!(
                        "DTSTART:{}",
                        at.datetime().with_timezone(&Utc).format("%Y%m%dT%H%M%SZ")
                    ));
                    format!(
                        "Organizer timezone: {}. Original deadline: {}.",
                        timezone.0.name(),
                        deadline.source_text()
                    )
                }
                Deadline::Date { on, .. } => {
                    lines.push(format!("DTSTART;VALUE=DATE:{}", on.0.format("%Y%m%d")));
                    format!(
                        "Date reminder only; cutoff time and timezone unverified. Original wording: {}.",
                        deadline.source_text()
                    )
                }
            };
            lines.extend([
                format!("SUMMARY:{}", text(&format!("Submission deadline: {}", opportunity.title.as_str()))),
                format!("URL:{}", opportunity.url.as_str()),
                format!("DESCRIPTION:{}", text(&format!(
                    "{} Page checked {}; review due {}. Recheck the organizer before submitting. This is a deadline reminder, not an attendance slot. Source: {}",
                    precision, opportunity.review.checked_on.0, opportunity.review.next_review_on.0, opportunity.url.as_str()))),
                "TRANSP:TRANSPARENT".to_owned(),
                "END:VEVENT".to_owned(),
            ]);
        }
        lines.push("END:VCALENDAR".to_owned());
        let mut output = String::new();
        for value in lines {
            line(&mut output, &value);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_cannot_inject_properties_and_utf8_folding_is_lossless() {
        let value = format!(
            "DESCRIPTION:{}",
            text(&format!(
                "{};comma,slash\\\r\nBEGIN:VEVENT",
                "🦀é".repeat(30)
            ))
        );
        let mut wire = String::new();
        line(&mut wire, &value);
        assert!(wire.split("\r\n").all(|l| l.len() <= 75));
        assert_eq!(wire.replace("\r\n ", "").trim_end_matches("\r\n"), value);
        assert!(!wire.contains("\r\nBEGIN:VEVENT"));
        assert!(value.contains("\\;comma\\,slash\\\\\\nBEGIN:VEVENT"));
    }
}
