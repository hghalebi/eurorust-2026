use super::*;
use chrono_tz::Europe::Madrid;
use serde::Serialize;

fn md(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('`', "\\`")
        .replace('|', "\\|")
}
fn pretty(value: &impl Serialize) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

impl CheckedGuides {
    fn reader_view(&self, portable: bool) -> String {
        let link = |path: &PublicPath| {
            if portable {
                format!(
                    "https://github.com/hghalebi/eurorust-2026/blob/main/{}",
                    path.as_str()
                )
            } else {
                format!("../{}", path.as_str())
            }
        };
        let mut out = format!(
            "# Resources for taking part in Rust events\n\nChoose a task, follow its guide, and adapt the example.\nThis selection was reviewed {}. It links original material; source terms apply.\nHistorical practices need adaptation to your event.\n\n",
            self.data.reference_date.0
        );
        let common_reuse = self
            .data
            .resources
            .first()
            .map(|r| &r.reuse)
            .filter(|wording| self.data.resources.iter().all(|r| &r.reuse == *wording));
        if let Some(wording) = common_reuse {
            out.push_str(&format!("{}\n\n", md(wording.as_str())));
        }
        if portable {
            out.push_str("This portable index contains route steps and source annotations. Linked guides follow the repository's current main branch; use the companion manifest to identify the exact input file hashes. Treat sources as reference data, preserve unknowns, and ask a person to review decisions.\n\n");
        }
        for route in &self.data.routes {
            out.push_str(&format!("## {}\n\n{}\n\n[Guide: {}]({}) · [Worked example or practice]({}) · [Adapt a template or prompt]({})\n\n", md(route.title.as_str()), md(route.outcome.as_str()), md(route.title.as_str()), link(&route.guide), link(&route.example), link(&route.template)));
            for (i, step) in route.steps.iter().enumerate() {
                out.push_str(&format!("{}. {}\n", i + 1, md(step.as_str())));
            }
            out.push_str("\nUseful sources: ");
            out.push_str(
                &route
                    .resource_ids
                    .iter()
                    .map(|id| {
                        let resource = self
                            .data
                            .resources
                            .iter()
                            .find(|r| &r.id == id)
                            .expect("references checked on load");
                        format!("[{}](#source.{})", md(resource.title.as_str()), id.as_str())
                    })
                    .collect::<Vec<_>>()
                    .join(" · "),
            );
            out.push_str(".\n\n");
        }
        out.push_str("## Read the original sources\n\nPage checked means the cited page was read for this annotation. It does not establish full-book reading, video playback, a tested deployment, or an available support offer.\n\n");
        for resource in &self.data.resources {
            out.push_str(&format!(
                "<a id=\"source.{}\"></a>\n\n### {}\n\n[Original source](<{}>) — {}. Language: `{}`. {}\n\n{} **Try:** {}\n\n",
                resource.id.as_str(),
                md(resource.title.as_str()),
                resource.url.as_str(),
                md(resource.publisher.as_str()),
                resource.language.as_str(),
                match resource.access {
                    Access::FreeWebPage => "Free page when checked.",
                    Access::Unknown => "Access conditions unverified.",
                },
                md(resource.why.as_str()),
                md(resource.action.as_str())
            ));
            if matches!(resource.context, ReferenceContext::Historical) {
                out.push_str("Historical reference. ");
            }
            if let Some(date) = resource.published_on {
                out.push_str(&format!("Published {}. ", date.0));
            }
            out.push_str(&format!(
                "{} {}; review due {}.\n\n",
                match resource.review.status {
                    ReviewStatus::PageChecked => "Page checked",
                    ReviewStatus::Unavailable => "Retrieval attempted; source unavailable",
                },
                resource.review.checked_on.0,
                resource.review.next_review_on.0
            ));
            for limit in &resource.review.limits {
                out.push_str(&format!("- {}\n", md(limit.as_str())));
            }
            if common_reuse.is_none() {
                out.push_str(&format!("\n{}\n", md(resource.reuse.as_str())));
            }
            out.push('\n');
        }
        out.push_str("## Related collections\n\n");
        for path in &self.data.related_collections {
            out.push_str(&format!("- [{}]({})\n", path.as_str(), link(path)));
        }
        out
    }

    fn day_guide(&self) -> String {
        let mut out = format!(
            "# EuroRust at a glance\n\nTimes are Europe/Madrid (UTC+02:00 on these dates). This snapshot records published sessions, not attendance.\nSnapshot: `{}`. Locations checked {} in the [logistics companion](../community/session-logistics.json). Confirm changes on each official page.\n\nStage labels are not verified physical room names. Use the [venue and entrance guide](../docs/time-and-place.md) for addresses, meeting points, and unresolved room details. Unknown ends stay unknown.\n\n[Detailed schedule](schedule.md) · [Choose and prepare](../docs/attend-and-learn.md) · [Calendar instructions](../docs/itinerary-calendar.md)\n\n",
            self.bundle.manifest().snapshot_id.as_str(),
            self.logistics.checked_at().as_rfc3339()
        );
        let mut sessions: Vec<_> = self.bundle.catalog().sessions.iter().collect();
        sessions.sort_by_key(|s| s.slot.start());
        let mut day = None;
        for session in sessions {
            let start = session.slot.start().datetime().with_timezone(&Madrid);
            if day != Some(start.date_naive()) {
                day = Some(start.date_naive());
                out.push_str(&format!("\n## {}\n\n| Local time | Session | Stage label | Venue |\n| --- | --- | --- | --- |\n", start.format("%A, %B %d")));
            }
            let end = session
                .slot
                .end()
                .map(|t| {
                    t.datetime()
                        .with_timezone(&Madrid)
                        .format("%H:%M")
                        .to_string()
                })
                .unwrap_or_else(|| "end unknown".into());
            let venue = self
                .logistics
                .venue(&session.id)
                .map_or("Unconfirmed; see venue guide", VenueName::as_str);
            let preparation = if matches!(session.kind, SessionKind::Talk | SessionKind::Workshop) {
                format!(
                    " · [Prepare](../resources/related-talks/{}.md)",
                    session.id.as_str()
                )
            } else {
                String::new()
            };
            out.push_str(&format!(
                "| {}–{} | [{}](<{}>){} | {} | {} |\n",
                start.format("%H:%M"),
                end,
                md(session.title.as_str()),
                session.official_url.as_str(),
                preparation,
                md(session.stage.as_ref().map_or("Unknown", StageName::as_str)),
                md(venue)
            ));
        }
        out.push_str("\nThe source timestamps and individual access conditions are in the [full conference pack](conference-pack.md). Workshops and side activities may need separate registration; a calendar entry is no ticket.\n");
        out
    }

    pub(super) fn views(&self) -> Result<BTreeMap<String, Vec<u8>>> {
        let mut views = BTreeMap::from([
            (
                "resources/event-resources.md".into(),
                self.reader_view(false).into_bytes(),
            ),
            (
                "exports/guides/reader-pack.md".into(),
                self.reader_view(true).into_bytes(),
            ),
            (
                "exports/guides/reader-pack.json".into(),
                pretty(&self.data)?,
            ),
            ("exports/day-guide.md".into(), self.day_guide().into_bytes()),
            (
                "schema/event-resources.schema.json".into(),
                pretty(&schemars::schema_for!(Collection))?,
            ),
        ]);
        for (name, calendar) in &self.calendars {
            views.insert(
                format!("exports/calendars/{name}.ics"),
                calendar.as_ics().as_bytes().to_vec(),
            );
        }
        let hashes: BTreeMap<_, _> = views
            .iter()
            .map(|(p, b)| (p.clone(), ContentHash::from_content(b)))
            .collect();
        let manifest = serde_json::json!({"schema_version":"1.0.0", "catalog_snapshot_id":self.data.catalog_snapshot_id, "resource_reference_date":self.data.reference_date, "input_files":self.inputs, "files":hashes, "limits":"Offline consistency only. Individual source review dates are preserved. Example calendars are tentative, omit participant details, and are not live subscriptions or bookings."});
        views.insert("exports/guides/manifest.json".into(), pretty(&manifest)?);
        Ok(views)
    }
}
