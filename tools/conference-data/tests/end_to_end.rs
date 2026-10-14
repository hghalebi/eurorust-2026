use conference_data::{
    collect,
    domain::*,
    export,
    itinerary::{self, Profile, Selection},
};
use std::{fs, process::Command};

fn fixture_catalog() -> Catalog {
    let source = Provenance {
        source_url: collect::SCHEDULE.to_owned().try_into().unwrap(),
        retrieved_at: Timestamp::now(),
        source_updated_at: None,
        content_sha256: collect::digest(b"synthetic fixture"),
    };
    let checked = collect::parse_schedule(include_str!("fixtures/schedule.html"), source)
        .unwrap()
        .validate()
        .unwrap();
    let mut catalog: Catalog =
        serde_json::from_value(serde_json::to_value(checked.catalog()).unwrap()).unwrap();
    let mut alternative = catalog.sessions[0].clone();
    alternative.id = "talks-fixture-robotics".to_owned().try_into().unwrap();
    alternative.title = "Robotics fixture".to_owned().try_into().unwrap();
    alternative.official_url = "https://eurorust.eu/talks/fixture-robotics/"
        .to_owned()
        .try_into()
        .unwrap();
    catalog.sessions.push(alternative);
    for (index, topic) in [(0, "async"), (2, "robotics")] {
        let session = &mut catalog.sessions[index];
        session.detail_source = Some(Provenance {
            source_url: session.official_url.clone(),
            retrieved_at: Timestamp::now(),
            source_updated_at: None,
            content_sha256: collect::digest(b"synthetic detail"),
        });
        session.annotation = Some(Annotation {
            session_id: session.id.clone(),
            summary: "Synthetic test summary".to_owned().try_into().unwrap(),
            topics: vec![topic.to_owned().try_into().unwrap()],
            prerequisites: Some("Synthetic prerequisite".to_owned().try_into().unwrap()),
            setup: Some("Synthetic setup".to_owned().try_into().unwrap()),
            author: "Test fixture".to_owned().try_into().unwrap(),
            method: "Synthetic".to_owned().try_into().unwrap(),
            review_status: AnnotationReview::SourceCheckedDraft,
            source_checked_at: Timestamp::now(),
            source_content_sha256: collect::digest(b"synthetic detail"),
            source_url: session.official_url.clone(),
        });
    }
    catalog
}
fn bundle(root: &std::path::Path) -> export::VerifiedBundle {
    export::publish(
        DraftSnapshot::new(fixture_catalog()).validate().unwrap(),
        root,
    )
    .unwrap();
    export::verify(root).unwrap()
}
fn profile(topic: &str) -> Profile {
    serde_json::from_value(serde_json::json!({"name":"Fictional test participant","goals":["Explore a topic"],"topics":[topic],"available_windows":[],"registered_workshops":[],"attendance_mode":null})).unwrap()
}

#[test]
fn participant_profiles_produce_different_checked_choices() {
    let directory = tempfile::tempdir().unwrap();
    let checked = bundle(directory.path());
    let backend = itinerary::recommend(&checked, profile("async")).unwrap();
    let robotics = itinerary::recommend(&checked, profile("robotics")).unwrap();
    assert_ne!(
        backend.selections[0].session_id,
        robotics.selections[0].session_id
    );
    assert!(
        itinerary::check(&checked, &backend)
            .unwrap()
            .iter()
            .any(|s| s.contains("Availability unknown"))
    );
    assert!(itinerary::check(&checked, &robotics).is_ok());
}

#[test]
fn rejects_overlap_stale_snapshot_and_wrong_availability() {
    let directory = tempfile::tempdir().unwrap();
    let checked = bundle(directory.path());
    let mut plan = itinerary::recommend(&checked, profile("async")).unwrap();
    plan.snapshot_id = "different-snapshot".to_owned().try_into().unwrap();
    assert!(
        itinerary::check(&checked, &plan)
            .unwrap_err()
            .to_string()
            .contains("different snapshot")
    );
    plan.snapshot_id = checked.manifest().snapshot_id.clone();
    plan.selections.push(Selection {
        session_id: "talks-fixture-robotics".to_owned().try_into().unwrap(),
        rationale: "Alternative selected concurrently"
            .to_owned()
            .try_into()
            .unwrap(),
        alternatives: vec![],
    });
    assert!(
        itinerary::check(&checked, &plan)
            .unwrap_err()
            .to_string()
            .contains("overlap")
    );
    plan.selections.pop();
    plan.profile.available_windows = vec![
        Slot::new(
            "2026-10-15T11:00:00+02:00".to_owned().try_into().unwrap(),
            Some("2026-10-15T12:00:00+02:00".to_owned().try_into().unwrap()),
        )
        .unwrap(),
    ];
    assert!(
        itinerary::check(&checked, &plan)
            .unwrap_err()
            .to_string()
            .contains("availability")
    );
}

#[test]
fn rejects_broken_reference_and_annotation_identity() {
    let mut catalog = fixture_catalog();
    catalog.sessions[0].speaker_ids = vec!["speaker-missing".to_owned().try_into().unwrap()];
    assert!(DraftSnapshot::new(catalog).validate().is_err());
    let mut catalog = fixture_catalog();
    catalog.sessions[0].annotation.as_mut().unwrap().session_id =
        "talks-other".to_owned().try_into().unwrap();
    assert!(DraftSnapshot::new(catalog).validate().is_err());
}

#[test]
fn offline_hydration_rejects_stale_annotation_and_preserves_publication() {
    let directory = tempfile::tempdir().unwrap();
    let checked = bundle(directory.path());
    let before = fs::read(directory.path().join("data/manifest.json")).unwrap();
    let mut catalog = fixture_catalog();
    catalog.sessions[0]
        .detail_source
        .as_mut()
        .unwrap()
        .content_sha256 = collect::digest(b"changed source");
    let input = directory.path().join("changed.json");
    fs::write(&input, serde_json::to_vec(&catalog).unwrap()).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_conference-data"))
        .arg("--root")
        .arg(directory.path())
        .args(["export", "--catalog"])
        .arg(&input)
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("annotation hash differs"));
    assert_eq!(
        before,
        fs::read(directory.path().join("data/manifest.json")).unwrap()
    );
    assert_eq!(
        checked.manifest().snapshot_id,
        export::verify(directory.path())
            .unwrap()
            .manifest()
            .snapshot_id
    );
    catalog.sessions[0]
        .annotation
        .as_mut()
        .unwrap()
        .review_status = AnnotationReview::SourceChanged;
    export::publish(
        DraftSnapshot::new(catalog).validate().unwrap(),
        directory.path(),
    )
    .unwrap();
    let updated = export::verify(directory.path()).unwrap();
    let plan = itinerary::recommend(&updated, profile("async")).unwrap();
    assert!(
        !plan.selections[0]
            .rationale
            .as_str()
            .contains("Matches requested topics")
    );
    assert!(
        itinerary::check(&updated, &plan)
            .unwrap()
            .iter()
            .any(|w| w.contains("annotation source changed"))
    );
    let mut missing = fixture_catalog();
    missing.sessions[0].detail_source = None;
    assert!(DraftSnapshot::new(missing).validate().is_err());
}

#[test]
fn readable_outputs_preserve_venue_preparation_and_exact_itinerary_evidence() {
    let directory = tempfile::tempdir().unwrap();
    let checked = bundle(directory.path());
    let schedule = fs::read_to_string(directory.path().join("exports/schedule.md")).unwrap();
    assert!(schedule.contains("Synthetic prerequisite"));
    assert!(schedule.contains("Synthetic setup"));
    assert!(schedule.contains(checked.catalog().venues[0].name.as_str()));
    let plan = itinerary::recommend(&checked, profile("async")).unwrap();
    let text = itinerary::markdown(&checked, &plan).unwrap();
    assert!(text.contains(plan.snapshot_id.as_str()));
    assert!(text.contains(plan.selections[0].session_id.as_str()));
    assert!(text.contains(checked.catalog().sessions[0].official_url.as_str()));
    assert!(text.contains(&checked.catalog().sessions[0].slot.start().as_rfc3339()));
    assert!(text.contains("Synthetic prerequisite"));
    assert!(text.contains("Same-slot alternative"));
    assert!(text.contains("Availability unknown"));
}

#[test]
fn readable_optional_workshops_respect_participant_availability() {
    let directory = tempfile::tempdir().unwrap();
    let mut catalog = fixture_catalog();
    let mut workshop = catalog.sessions[0].clone();
    workshop.id = "workshops-fixture".to_owned().try_into().unwrap();
    workshop.kind = SessionKind::Workshop;
    workshop.slot = Slot::new(
        "2026-10-14T10:00:00+02:00".to_owned().try_into().unwrap(),
        Some("2026-10-14T16:00:00+02:00".to_owned().try_into().unwrap()),
    )
    .unwrap();
    workshop.annotation = None;
    workshop.detail_source = None;
    catalog.sessions.push(workshop);
    export::publish(
        DraftSnapshot::new(catalog).validate().unwrap(),
        directory.path(),
    )
    .unwrap();
    let checked = export::verify(directory.path()).unwrap();
    let unconstrained = itinerary::recommend(&checked, profile("async")).unwrap();
    assert!(
        itinerary::markdown(&checked, &unconstrained)
            .unwrap()
            .contains("workshops-fixture")
    );
    let mut constrained = profile("async");
    constrained.available_windows = vec![
        Slot::new(
            "2026-10-15T10:00:00+02:00".to_owned().try_into().unwrap(),
            Some("2026-10-15T18:00:00+02:00".to_owned().try_into().unwrap()),
        )
        .unwrap(),
    ];
    let plan = itinerary::recommend(&checked, constrained).unwrap();
    let text = itinerary::markdown(&checked, &plan).unwrap();
    assert!(!text.contains("workshops-fixture"));
    assert!(!text.contains("October 17"));
}

#[test]
fn modified_view_is_detected_and_offline_export_recovers() {
    let directory = tempfile::tempdir().unwrap();
    let checked = bundle(directory.path());
    let immutable = directory
        .path()
        .join(&checked.manifest().authoritative_catalog);
    fs::write(directory.path().join("data/sessions.json"), b"[]").unwrap();
    assert!(export::verify(directory.path()).is_err());
    let old = collect::load(&immutable).unwrap();
    export::publish(old, directory.path()).unwrap();
    assert!(export::verify(directory.path()).is_ok());
}

#[test]
fn actual_binary_rejects_bad_input_without_replacing_last_snapshot() {
    let directory = tempfile::tempdir().unwrap();
    let _checked = bundle(directory.path());
    let before = fs::read(directory.path().join("data/manifest.json")).unwrap();
    let input = directory.path().join("invalid.json");
    fs::write(&input, b"{}").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_conference-data"))
        .arg("--root")
        .arg(directory.path())
        .args(["export", "--catalog"])
        .arg(input)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert_eq!(
        before,
        fs::read(directory.path().join("data/manifest.json")).unwrap()
    );
    assert!(export::verify(directory.path()).is_ok());
    let good = Command::new(env!("CARGO_BIN_EXE_conference-data"))
        .arg("--root")
        .arg(directory.path())
        .arg("validate")
        .output()
        .unwrap();
    assert!(good.status.success());
}

#[test]
fn manifest_generator_binding_detects_tampering() {
    let directory = tempfile::tempdir().unwrap();
    let checked = bundle(directory.path());
    assert_eq!(checked.manifest().session_count, SessionCount::from(3));
    assert!(export::verify(directory.path()).is_ok());
    let path = directory
        .path()
        .join(&checked.manifest().authoritative_catalog);
    assert_eq!(collect::load(&path).unwrap().catalog().sessions.len(), 3);
    let mut manifest = serde_json::to_value(checked.manifest()).unwrap();
    manifest["generator_source_sha256"] =
        serde_json::json!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    fs::write(
        directory.path().join("data/manifest.json"),
        serde_json::to_vec(&manifest).unwrap(),
    )
    .unwrap();
    assert!(export::verify(directory.path()).is_err());
}

#[test]
fn document_fixtures_reject_active_history_and_private_dependencies() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    fs::create_dir(root.join("docs")).unwrap();
    let inventory = serde_json::json!([
        {"path":"README.md","class":"current","owner":"Fixture","purpose":"Active guide","scope":"Test","replacement":null},
        {"path":"history.md","class":"historical","owner":"Fixture","purpose":"Archived record","scope":"Test","replacement":null}
    ]);
    fs::write(
        root.join("docs/document-map.json"),
        serde_json::to_vec(&inventory).unwrap(),
    )
    .unwrap();
    fs::write(root.join("history.md"), "# Historical record\n").unwrap();
    fs::write(root.join("README.md"), "# Guide\n[Continue](history.md)\n").unwrap();
    assert!(conference_data::navigation::check(root).is_err());
    fs::write(
        root.join("README.md"),
        "# Guide\n[Historical record](history.md)\n",
    )
    .unwrap();
    assert!(conference_data::navigation::check(root).is_ok());
    fs::create_dir(root.join(".plan")).unwrap();
    fs::write(root.join(".plan/private.md"), "# Private\n").unwrap();
    fs::write(
        root.join("README.md"),
        "# Guide\n[Required](.plan/private.md)\n",
    )
    .unwrap();
    assert!(conference_data::navigation::check(root).is_err());
}
