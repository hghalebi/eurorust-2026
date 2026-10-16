use super::*;
use serde_json::{Value, json};

fn fixture() -> tempfile::TempDir {
    let temporary = tempfile::tempdir().unwrap();
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for name in [
        "data",
        "exports",
        "resources",
        "community",
        "docs",
        "prompts",
        "templates",
        "schema",
        "my-conference",
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
    temporary
}

#[test]
fn invalid_references_dates_and_private_paths_preserve_existing_outputs() {
    let temporary = fixture();
    let root = temporary.path();
    load(root).unwrap().render(root).unwrap();
    let output = root.join("exports/guides/reader-pack.json");
    let expected = fs::read(&output).unwrap();
    let input = root.join("community/event-resources.json");
    let original = fs::read(&input).unwrap();
    let collection: Collection = serde_json::from_slice(&original).unwrap();
    // Keep the invalid cases relative to the reviewed fixture as it advances.
    let after_reference: CheckedOn = collection
        .reference_date
        .0
        .succ_opt()
        .unwrap()
        .to_string()
        .try_into()
        .unwrap();
    let after_review_window: ReviewDue = collection.resources[0]
        .review
        .checked_on
        .0
        .checked_add_days(chrono::Days::new(91))
        .unwrap()
        .to_string()
        .try_into()
        .unwrap();
    let edits = [
        ("/routes/0/guide", json!("docs/../.plan/private.md")),
        ("/routes/0/guide", json!(".plan/private.md")),
        ("/routes/0/guide", json!("/tmp/private.md")),
        ("/routes/0/resource_ids/0", json!("missing-resource")),
        ("/resources/0/review/checked_on", json!(after_reference)),
        (
            "/resources/0/review/next_review_on",
            json!(after_review_window),
        ),
        ("/resources/0/review/checked_on", json!("2026-02-30")),
        ("/resources/0/review/limits", json!([])),
        ("/resources/0/review/status", json!("unavailable")),
        (
            "/resources/0/url",
            json!("https://user:password@example.com/"),
        ),
        ("/catalog_snapshot_id", json!("unreviewed-snapshot")),
    ];
    for (pointer, replacement) in edits {
        let mut value: Value = serde_json::from_slice(&original).unwrap();
        *value.pointer_mut(pointer).unwrap() = replacement;
        fs::write(&input, serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(
            load(root).and_then(|c| c.render(root)).is_err(),
            "{pointer}"
        );
        assert_eq!(fs::read(&output).unwrap(), expected);
    }
    fs::write(&input, &original).unwrap();
    load(root).unwrap().verify(root).unwrap();
    // A successful HTTP/source check cannot make an arbitrary enum variant valid.
    let mut value: Value = serde_json::from_slice(&original).unwrap();
    value["resources"][0]["review"]["status"] = json!("speaker_endorsed");
    assert!(serde_json::from_value::<Collection>(value).is_err());
}

#[test]
fn regeneration_repairs_partial_outputs_and_detects_changed_linked_guides() {
    let temporary = fixture();
    let root = temporary.path();
    load(root).unwrap().render(root).unwrap();
    let output = root.join("exports/guides/reader-pack.md");
    let expected = fs::read(&output).unwrap();
    fs::write(&output, "interrupted write").unwrap();
    assert!(load(root).unwrap().verify(root).is_err());
    load(root).unwrap().render(root).unwrap();
    assert_eq!(fs::read(&output).unwrap(), expected);
    let guide = root.join("resources/speaking/README.md");
    let mut bytes = fs::read(&guide).unwrap();
    bytes.extend_from_slice(b"\nChanged source guide.\n");
    fs::write(guide, bytes).unwrap();
    assert!(load(root).unwrap().verify(root).is_err());
    load(root).unwrap().render(root).unwrap();
    load(root).unwrap().verify(root).unwrap();
}

#[test]
fn resource_anchors_survive_title_collisions_and_unavailable_is_not_free() {
    let temporary = fixture();
    let root = temporary.path();
    let input = root.join("community/event-resources.json");
    let mut value: Value = serde_json::from_slice(&fs::read(&input).unwrap()).unwrap();
    value["resources"][0]["title"] = value["routes"][0]["title"].clone();
    value["resources"][0]["review"]["status"] = json!("unavailable");
    value["resources"][0]["access"] = json!("unknown");
    value["resources"][0]["why"] =
        json!("Compare <em>claims</em> & [links](https://example.org) with `code`.");
    fs::write(input, serde_json::to_vec(&value).unwrap()).unwrap();
    let views = load(root).unwrap().views().unwrap();
    let view = String::from_utf8(views["resources/event-resources.md"].clone()).unwrap();
    assert!(view.contains("[Give a talk or teach](#source.rustfest-first-talk)"));
    assert!(view.contains("<a id=\"source.rustfest-first-talk\"></a>"));
    let card = view
        .split("<a id=\"source.rustfest-first-talk\"></a>")
        .nth(1)
        .unwrap()
        .split("<a id=")
        .next()
        .unwrap();
    assert!(card.contains("Access conditions unverified."));
    assert!(!card.contains("Free page when checked."));
    assert!(card.contains(
        "Compare &lt;em&gt;claims&lt;/em&gt; &amp; \\[links\\](https://example.org) with \\`code\\`."
    ));
    assert!(!card.contains("<em>claims</em>"));
}

#[cfg(unix)]
#[test]
fn public_alias_cannot_read_private_or_external_files() {
    let temporary = fixture();
    let root = temporary.path();
    fs::create_dir_all(root.join(".plan")).unwrap();
    fs::write(root.join(".plan/private.md"), "private").unwrap();
    let alias = root.join("resources/alias.md");
    std::os::unix::fs::symlink(root.join(".plan/private.md"), &alias).unwrap();
    let path: PublicPath = "resources/alias.md".to_owned().try_into().unwrap();
    assert!(read_public(root, &path).is_err());
    fs::remove_file(&alias).unwrap();
    let outside = tempfile::NamedTempFile::new().unwrap();
    std::os::unix::fs::symlink(outside.path(), &alias).unwrap();
    assert!(read_public(root, &path).is_err());
}

#[test]
fn day_guide_uses_checked_logistics_and_preserves_it_on_bad_references() {
    let temporary = fixture();
    let root = temporary.path();
    load(root).unwrap().render(root).unwrap();
    let output = root.join("exports/day-guide.md");
    let expected = fs::read_to_string(&output).unwrap();
    assert!(expected.contains("Bikini Barcelona"));
    assert!(expected.contains("Helsing Office, One Parc Central"));
    assert!(expected.contains("Unconfirmed; see venue guide"));
    assert_eq!(expected.matches("## ").count(), 4);
    let input = root.join("community/session-logistics.json");
    let original = fs::read(&input).unwrap();
    for (pointer, replacement) in [
        ("/catalog_snapshot_id", json!("unreviewed-snapshot")),
        ("/sessions/0/venue_key", json!("unknown-venue")),
        ("/sessions/0/source_ids/0", json!("unknown-source")),
        ("/sessions/0/stage", json!("Invented room")),
        ("/sessions/0/slot/start", json!("2026-10-14T09:00:00+02:00")),
        (
            "/sources/schedule/retrieved_at",
            json!("2027-01-01T00:00:00Z"),
        ),
    ] {
        let mut value: Value = serde_json::from_slice(&original).unwrap();
        *value.pointer_mut(pointer).unwrap() = replacement;
        fs::write(&input, serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(
            load(root).and_then(|c| c.render(root)).is_err(),
            "{pointer}"
        );
        assert_eq!(fs::read_to_string(&output).unwrap(), expected);
    }
}
