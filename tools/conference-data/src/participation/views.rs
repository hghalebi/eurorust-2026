use super::{CheckedParticipation, model::*};
use crate::{Result, domain::SourceUrl};
use std::collections::BTreeMap;

fn md(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('\u{60}', "\\\u{60}")
        .replace('|', "\\|")
}
fn link(label: &str, url: &SourceUrl) -> String {
    format!("[{}](<{}>)", md(label), url.as_str())
}
fn anchor(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .filter_map(|ch| {
            if ch == ' ' {
                Some('-')
            } else if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                Some(ch)
            } else {
                None
            }
        })
        .collect()
}
fn access_notes(access: &Access) -> String {
    let mut notes = vec![access.mode.label().to_owned()];
    if let Some(value) = &access.cost {
        notes.push(md(value.as_str()));
    }
    if let Some(value) = &access.spoken_languages {
        notes.push(format!(
            "spoken languages: {}",
            value
                .iter()
                .map(|l| l.0.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if let Some(value) = &access.captions {
        notes.push(md(value.as_str()));
    }
    if let Some(value) = &access.eligibility {
        notes.push(md(value.as_str()));
    }
    if let Some(value) = &access.equipment {
        notes.push(md(value.as_str()));
    }
    if let Some(value) = &access.venue {
        notes.push(format!("venue: {}", md(value.as_str())));
    }
    if let Some(value) = &access.room {
        notes.push(format!("room: {}", md(value.as_str())));
    }
    notes.join(" · ")
}
fn review_notes(review: &Review, primary: &SourceUrl) -> String {
    let evidence = review
        .evidence_urls
        .iter()
        .filter(|u| *u != primary)
        .map(|u| {
            let parsed = url::Url::parse(u.as_str()).expect("validated source URL");
            link(parsed.host_str().unwrap(), u)
        })
        .collect::<Vec<_>>()
        .join(", ");
    let mut result = String::new();
    for limit in &review.limits {
        result.push_str(&format!("{}\n\n", md(limit.as_str())));
    }
    result.push_str(&format!(
        "{}: {} · next review {}.",
        review.source_review.label(),
        review.checked_on.0,
        review.next_review_on.0
    ));
    if !evidence.is_empty() {
        result.push_str(&format!(" Further evidence: {evidence}."));
    }
    result.push_str("\n\n");
    result
}
impl CheckedParticipation {
    pub(super) fn views(&self) -> Result<BTreeMap<&'static str, Vec<u8>>> {
        let mut schema = serde_json::to_vec_pretty(&schemars::schema_for!(Participation))?;
        schema.push(b'\n');
        Ok(BTreeMap::from([
            ("schema/participation.schema.json", schema),
            ("community/README.md", self.routes_page().into_bytes()),
            (
                "resources/stay-current/communities.md",
                self.communities_page().into_bytes(),
            ),
            (
                "resources/stay-current/opportunities.md",
                self.opportunities_page().into_bytes(),
            ),
            (
                "resources/stay-current/events.ics",
                self.calendar().into_bytes(),
            ),
        ]))
    }

    fn routes_page(&self) -> String {
        let mut body = format!(
            "# Find your next Rust conversation\n\nTurn a EuroRust question into a discussion, a small contribution, or something to try.\nThese paths extend my interests in production agents, types, infrastructure, and\nrobotics. Adapt them to your own work.\n\n| You want to | Start here |\n| --- | --- |\n| Find people working on your problem | [Communities by topic and region](../resources/stay-current/communities.md) |\n| Learn or contribute with others | [Practical first steps](../resources/stay-current/participate.md) |\n| Speak, join an activity, or check support | [Calls and participation opportunities](../resources/stay-current/opportunities.md) |\n| Plan with an agent | [Community prompt](../prompts/find-my-community.md) and [structured records](participation.json) |\n\nChecked through **{}**. These are editorial connections, not speaker endorsements\nor evidence of attendance. Joining, availability, and access need a current check.\n\n## Four paths from a session to a community\n\n",
            self.data.reference_date.0
        );
        for route in &self.data.routes {
            let Relationship::EditorialSuggestion = route.relationship;
            let communities = route
                .community_ids
                .iter()
                .map(|id| {
                    let c = self.data.communities.iter().find(|c| &c.id == id).unwrap();
                    format!(
                        "[{}](../resources/stay-current/communities.md#{})",
                        md(c.name.as_str()),
                        anchor(c.name.as_str())
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            let actions = route
                .opportunity_ids
                .iter()
                .map(|id| {
                    let o = self
                        .data
                        .opportunities
                        .iter()
                        .find(|o| &o.id == id)
                        .unwrap();
                    format!(
                        "[{}](../resources/stay-current/opportunities.md#{})",
                        md(o.title.as_str()),
                        anchor(o.title.as_str())
                    )
                })
                .collect::<Vec<_>>()
                .join("; ");
            body.push_str(&format!(
                "### {}\n\n[Start with the session reading](../{}), then explore {}.\n\n**Try:** {}\n\n**Smaller step:** {}\n\nPreparation: {}\n\nParticipation routes: {}.\n\n",
                md(route.title.as_str()), route.reading_path.0, communities, md(route.first_step.as_str()),
                md(route.small_step.as_str()), md(route.prerequisites.as_str()), actions));
        }
        body.push_str("## Make something useful together\n\nTry a reading club, a small experiment, a documentation sprint, a demo, pairing,\nor a topic discussion. The [activity template](../templates/community-activity.md)\nhelps you propose one and record what actually happened. No activity is announced\nby this collection.\n\nStart with one source and one question. Share a correction, a reproducible result,\nor a useful limitation through the [contribution guide](../CONTRIBUTING.md#communities-and-participation).\nKeep private conversations and contact details private.\n\n[More learning sources](../resources/stay-current/README.md) · [Conference archives](../resources/conferences/README.md) · [Coverage and source limits](../resources/stay-current/communities.md#coverage-and-gaps)\n\nThis page is generated from [participation.json](participation.json).\n");
        body
    }

    fn communities_page(&self) -> String {
        let mut body = format!(
            "# Find a Rust community\n\nChoose a group by the question you want to explore. Start by reading its public\nmaterial and participation rules; chat access and available help need their own check.\n\n{} community routes · source review through **{}**.\n\n[Four personal learning paths](../../community/README.md) · [First contributions and learning](participate.md) · [Calls and activities](opportunities.md)\n\n## Choose a starting point\n\n- [Discussion and support](#discussion-and-support): user help, language proposals, and organizer routes.\n- [Project communities](#project-communities): async services, types, infrastructure, robotics, security, games, and WebAssembly.\n- [Learning together](#learning-together): exercises, workshops, and education.\n- [Regional groups](#regional-groups): local and language communities, with activity limits.\n\nLanguages below describe inspected pages, not future meeting language.\nUnless stated, cost, captions, eligibility, equipment, venue, and room are unknown.\nA page check establishes a route; it does not establish recent group activity.\n\n",
            self.data.communities.len(),
            self.data.reference_date.0
        );
        for (heading, kinds) in [
            (
                "Discussion and support",
                vec![CommunityKind::Organization, CommunityKind::Forum],
            ),
            (
                "Project communities",
                vec![CommunityKind::Team, CommunityKind::Project],
            ),
            ("Learning together", vec![CommunityKind::Learning]),
            ("Regional groups", vec![CommunityKind::Regional]),
        ] {
            body.push_str(&format!("## {heading}\n\n"));
            for c in self
                .data
                .communities
                .iter()
                .filter(|c| kinds.contains(&c.kind))
            {
                let topics = c
                    .topics
                    .iter()
                    .map(|t| format!("[{}](../topics/{}.md)", md(t.as_str()), t.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ");
                body.push_str(&format!("### {}\n\n{} · {} · {} · pages: {}\n\n{} For {}.\n\n**First step:** {}\n\nTopics: {}. Access: {}.\n\n",
                    link(c.name.as_str(), &c.url), c.affiliation.label(), c.kind.label(),
                    c.location.as_ref().map(|l| md(l.as_str())).unwrap_or_else(|| c.region.label().to_owned()),
                    c.source_languages.iter().map(|l| l.0.as_str()).collect::<Vec<_>>().join(", "),
                    md(c.purpose.as_str()), md(c.audience.as_str()), md(c.first_step.as_str()),
                    topics, access_notes(&c.access)));
                if let Some(date) = c.activity.observed_on {
                    body.push_str(&format!(
                        "Activity evidence: {} · {} {}\n\n",
                        date.0,
                        md(c.activity.note.as_str()),
                        link("Activity source", &c.activity.evidence_url)
                    ));
                }
                if let Some(url) = &c.contribution_url {
                    body.push_str(&format!("{}.\n\n", link("Contribution guide", url)));
                }
                if let Some(url) = &c.code_of_conduct_url {
                    body.push_str(&format!("{}.\n\n", link("Participation rules", url)));
                }
                body.push_str(&review_notes(&c.review, &c.url));
            }
        }
        body.push_str("## Coverage and gaps\n\nThis is a researched selection. A covered category has a useful route, not exhaustive\ncoverage. Regional searches included Europe, Africa, Asia, the Americas, and Oceania.\nEnglish-language sources remain strongest.\n\n| Area | Coverage | Evidence or gap |\n| --- | --- | --- |\n");
        for entry in &self.data.coverage {
            body.push_str(&format!(
                "| {} | {} | {} |\n",
                md(entry.area.as_str()),
                entry.status.label(),
                md(entry.note.as_str())
            ));
        }
        body.push('\n');
        for gap in &self.data.regional_gaps {
            body.push_str(&format!("- {}\n", md(gap.as_str())));
        }
        body.push_str(&format!("\n{}\n\n[Suggest a community or correction](../../CONTRIBUTING.md#communities-and-participation).\nGenerated from [participation.json](../../community/participation.json); linked works keep their own terms.\n", md(self.data.review_status.as_str())));
        body
    }

    fn opportunities_page(&self) -> String {
        let mut body = format!(
            "# Speak, contribute, or join an activity\n\nFind a call for talks, a learning route, or practical support.\nStatuses below are a snapshot as of **{}**. Recheck the linked organizer or project\nbefore making a decision. An open call does not promise selection or funding.\n\n[Open calls](#open-calls) · [Announced activities](#announced-activities) · [Participation routes](#ongoing-participation-routes) · [Past examples](#past-examples) · [Closed calls](#closed-calls)\n\n[Communities](communities.md) · [Ways to participate](participate.md) · [Conference dates](conferences.md)\n\n## Use the calendar\n\nDownload [events.ics](events.ics) and import it into a separate calendar.\nIt contains announced or ongoing event dates from the [discovery directory](../../community/rust-discovery.json)\nand open calls with verified deadlines. Event dates are all-day spans; they are not\nsession timetables. Exact deadlines use UTC in the file and retain the organizer's\ntimezone in their description. A date-only deadline is a reminder with an unknown cutoff time.\n\nThis is a downloadable snapshot, with no automatic refresh. Replace the imported\ncalendar when updating; a later import may leave old entries behind. Unknown dates,\ntentative events, closed calls, and unverified recurring meetings are excluded.\nNo alarms, registrations, or subscriptions are created by this repository.\n\nUnless stated, costs, spoken language, captions, equipment, venue, room, and eligibility\nare unknown. Follow each event link for its date and location; a submission deadline\nis separate from the event it concerns.\n\n",
            self.data.reference_date.0
        );
        for status in [
            OpportunityStatus::Open,
            OpportunityStatus::Announced,
            OpportunityStatus::Route,
            OpportunityStatus::Historical,
            OpportunityStatus::Closed,
        ] {
            body.push_str(&format!("## {}\n\n", status.label()));
            if status == OpportunityStatus::Route {
                body.push_str("These are ways to take part, not confirmed open places, issues, or scheduled meetings.\n\n");
            }
            for o in self
                .data
                .opportunities
                .iter()
                .filter(|o| o.status == status)
            {
                body.push_str(&format!(
                    "### {}\n\n{} · {}\n\n{}\n\n",
                    link(o.title.as_str(), &o.url),
                    o.kind.label(),
                    access_notes(&o.access),
                    md(o.action.as_str())
                ));
                if let Some(deadline) = &o.deadline {
                    body.push_str(&format!("**Deadline:** {}.", md(deadline.source_text())));
                    match deadline {
                        Deadline::Instant { timezone, .. } => {
                            body.push_str(&format!(" Timezone: {}.\n\n", timezone.0.name()))
                        }
                        Deadline::Date { .. } => body.push_str(" Date only.\n\n"),
                    }
                } else if o.status == OpportunityStatus::Open {
                    body.push_str("**Deadline:** unverified; check the submission page.\n\n");
                }
                if let Some(id) = &o.event_id {
                    let event = self.discovery.events.iter().find(|e| &e.id == id).unwrap();
                    body.push_str(&format!(
                        "Event: {}. See [conference dates and limits](conferences.md).\n\n",
                        link(event.name.as_str(), &event.url)
                    ));
                }
                body.push_str(&review_notes(&o.review, &o.url));
            }
        }
        body.push_str("## What the event review established\n\nAll existing event and watch entries received a separate participation review.\nThese notes do not replace their original date checks or turn a watch item into a\nconfirmed event. Access applications and rooms remain unknown unless sourced.\n\n<details>\n<summary>Read the event-by-event findings</summary>\n\n| Event or watch item | Participation finding | Checked | Next review |\n| --- | --- | --- | --- |\n");
        for r in &self.data.event_reviews {
            let event = self
                .discovery
                .events
                .iter()
                .find(|e| e.id == r.event_id)
                .unwrap();
            let limits = r
                .review
                .limits
                .iter()
                .map(|l| md(l.as_str()))
                .collect::<Vec<_>>()
                .join(" ");
            body.push_str(&format!(
                "| {} | {} {} ({}) | {} | {} |\n",
                link(event.name.as_str(), &event.url),
                md(r.note.as_str()),
                limits,
                r.review.source_review.label(),
                r.review.checked_on.0,
                r.review.next_review_on.0
            ));
        }
        body.push_str("\n</details>\n\n[Suggest an opportunity or correction](../../CONTRIBUTING.md#communities-and-participation).\nGenerated from [participation.json](../../community/participation.json) and the pinned\n[discovery directory](../../community/rust-discovery.json). Source-page checks do not\nverify registration, eligibility, app imports, or attendance.\n");
        body
    }
}
