use super::*;
use chrono::NaiveDate;
use serde_json::{Value, json};

fn source_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture() -> tempfile::TempDir {
    let temporary = tempfile::tempdir().unwrap();
    let source = source_root();
    for name in [
        "data",
        "exports",
        "schema",
        "resources/topics",
        "resources/related-talks",
    ] {
        for entry in walkdir::WalkDir::new(source.join(name)) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let to = temporary
                    .path()
                    .join(entry.path().strip_prefix(&source).unwrap());
                fs::create_dir_all(to.parent().unwrap()).unwrap();
                fs::copy(entry.path(), to).unwrap();
            }
        }
    }
    fs::create_dir_all(temporary.path().join("community")).unwrap();
    for name in ["participation.json", "rust-discovery.json"] {
        fs::copy(
            source.join("community").join(name),
            temporary.path().join("community").join(name),
        )
        .unwrap();
    }
    temporary
}

fn rewrite(root: &Path, edit: impl FnOnce(&mut Value)) {
    let file = root.join("community/participation.json");
    let mut value: Value = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
    edit(&mut value);
    fs::write(file, serde_json::to_vec(&value).unwrap()).unwrap();
}

#[test]
fn checked_sources_render_and_recover_but_bad_inputs_do_not_write() {
    let fixture = fixture();
    let root = fixture.path();
    let checked = load(root).unwrap();
    checked.render(root).unwrap();
    checked.verify(root).unwrap();
    let calendar = root.join("resources/stay-current/events.ics");
    let expected = fs::read(&calendar).unwrap();
    fs::write(&calendar, "partial write").unwrap();
    assert!(load(root).unwrap().verify(root).is_err());
    load(root).unwrap().render(root).unwrap();
    assert_eq!(fs::read(&calendar).unwrap(), expected);

    let input = root.join("community/participation.json");
    let original = fs::read(&input).unwrap();
    for bad in [
        (
            "unknown route community",
            "community_ids",
            json!(["missing-community"]),
        ),
        (
            "unknown session",
            "session_ids",
            json!(["talks-not-in-catalog"]),
        ),
        (
            "private reading path",
            "reading_path",
            json!(".plan/private.md"),
        ),
        (
            "repeated reference",
            "community_ids",
            json!(["tokio", "tokio"]),
        ),
    ] {
        rewrite(root, |v| v["routes"][0][bad.1] = bad.2);
        assert!(
            load(root).and_then(|c| c.render(root)).is_err(),
            "{}",
            bad.0
        );
        assert_eq!(fs::read(&calendar).unwrap(), expected);
        fs::write(&input, &original).unwrap();
    }
    rewrite(root, |v| {
        v["communities"][1]["id"] = v["communities"][0]["id"].clone()
    });
    assert!(load(root).is_err());
    fs::write(&input, &original).unwrap();
    rewrite(root, |v| {
        v["opportunities"][0]["deadline"]["at"] = json!("2026-11-30T23:59:00-07:00")
    });
    assert!(load(root).is_err());
    fs::write(&input, &original).unwrap();
    rewrite(root, |v| {
        v["event_reviews"]
            .as_array_mut()
            .unwrap()
            .pop()
            .map(|_| ())
            .unwrap()
    });
    assert!(load(root).is_err());
    fs::write(&input, &original).unwrap();
    rewrite(root, |v| {
        v["catalog_snapshot_id"] = json!("unreviewed-snapshot")
    });
    assert!(load(root).is_err());
    fs::write(&input, &original).unwrap();
    // Even a whitespace-only discovery edit requires an explicit reviewed pin.
    let discovery = root.join("community/rust-discovery.json");
    let mut bytes = fs::read(&discovery).unwrap();
    bytes.push(b'\n');
    fs::write(discovery, bytes).unwrap();
    assert!(load(root).is_err());
    assert_eq!(fs::read(&calendar).unwrap(), expected);
}

#[test]
fn calendar_preserves_source_precision_and_skips_unknown_or_closed_items() {
    let checked = load(&source_root()).unwrap();
    let calendar = checked.calendar().replace("\r\n ", "");
    assert!(calendar.contains("DTSTART:20261201T075900Z\r\n"));
    assert!(calendar.contains("DTSTART:20270110T225900Z\r\n"));
    assert!(calendar.contains("DTSTART;VALUE=DATE:20261116\r\n"));
    assert!(calendar.contains("cutoff time and timezone unverified"));
    assert!(!calendar.contains("UID:deadline-rustchina-cfp-closed"));
    assert!(!calendar.contains("UID:deadline-rustnation-cfp"));
    assert!(!calendar.contains("UID:event-rustnation-workshops-2027"));
    assert!(!calendar.contains("UID:event-fosdem-rust-2027"));
    assert!(!calendar.contains("RRULE"));
    let ids: Vec<_> = calendar.lines().filter(|l| l.starts_with("UID:")).collect();
    assert_eq!(ids.len(), ids.iter().collect::<BTreeSet<_>>().len());
}

#[test]
fn changed_dates_keep_identity_and_cancellations_leave_the_next_snapshot() {
    let mut checked = load(&source_root()).unwrap();
    let before = checked.calendar().replace("\r\n ", "");
    let event = &mut checked.discovery.events[0];
    let uid = format!("UID:event-{}@eurorust-2026.hghalebi.github", event.id.0);
    event.starts_on = Some(EventDate(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()));
    event.ends_on = Some(EventDate(NaiveDate::from_ymd_opt(2027, 1, 1).unwrap()));
    let catalog = export::verify(&source_root()).unwrap();
    validate(
        &checked.data,
        &checked.discovery,
        catalog.catalog(),
        &source_root(),
    )
    .unwrap();
    let moved = checked.calendar().replace("\r\n ", "");
    assert!(before.contains(&uid) && moved.contains(&uid));
    assert!(moved.contains("DTSTART;VALUE=DATE:20261231\r\nDTEND;VALUE=DATE:20270102\r\n"));
    checked.discovery.events[0].status = EventStatus::Cancelled;
    assert!(!checked.calendar().contains(&uid));
    checked.discovery.events[0].status = EventStatus::Announced;
    checked.discovery.events[0].starts_on =
        Some(EventDate(NaiveDate::from_ymd_opt(2028, 2, 28).unwrap()));
    checked.discovery.events[0].ends_on =
        Some(EventDate(NaiveDate::from_ymd_opt(2028, 2, 29).unwrap()));
    assert!(checked.calendar().contains("DTEND;VALUE=DATE:20280301"));
}

#[test]
fn deadlines_validate_named_zone_at_the_actual_instant() {
    let check = |at: &str, timezone: &str| {
        check_deadline(
            &serde_json::from_value::<Deadline>(json!({
                "kind":"instant", "at":at, "timezone":timezone, "source_text":"Test deadline"
            }))
            .unwrap(),
        )
    };
    assert!(check("2026-11-01T01:30:00-07:00", "America/Los_Angeles").is_ok());
    assert!(check("2026-11-01T01:30:00-08:00", "America/Los_Angeles").is_ok());
    assert!(check("2026-07-01T12:00:00-08:00", "America/Los_Angeles").is_err());
    assert!(check("2026-03-29T02:30:00+01:00", "Europe/Paris").is_err());
    assert!(Timezone::try_from("Europe/NotAPlace".to_owned()).is_err());
}

#[test]
fn unknown_is_not_false_and_unavailable_needs_an_explanation() {
    let checked = load(&source_root()).unwrap();
    let mut data = checked.data.clone();
    let catalog = export::verify(&source_root()).unwrap();
    data.communities[0].access.spoken_languages = Some(vec![]);
    assert!(validate(&data, &checked.discovery, catalog.catalog(), &source_root()).is_err());
    data.communities[0].access.spoken_languages = None;
    data.communities[0].review.source_review = ReviewResult::Unavailable;
    data.communities[0].review.limits.clear();
    assert!(validate(&data, &checked.discovery, catalog.catalog(), &source_root()).is_err());
    data.communities[0].review.source_review = ReviewResult::PageChecked;
    data.communities[0].activity.observed_on =
        Some(ObservedOn(NaiveDate::from_ymd_opt(2027, 1, 1).unwrap()));
    assert!(validate(&data, &checked.discovery, catalog.catalog(), &source_root()).is_err());
}
