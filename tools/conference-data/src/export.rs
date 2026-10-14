use crate::{
    Error, Result,
    collect::{digest, load},
    domain::*,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io::Write, path::Path};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub snapshot_id: SnapshotId,
    pub schema_version: SchemaVersion,
    pub generator_version: GeneratorVersion,
    pub generator_source_sha256: ContentHash,
    pub validated_at: Timestamp,
    pub authoritative_catalog: String,
    pub session_count: usize,
    pub detail_count: usize,
    pub files: BTreeMap<String, String>,
}

pub fn snapshot_id(catalog: &Catalog) -> Result<SnapshotId> {
    snapshot_id_with_generator(catalog, &generator_fingerprint())
}

fn snapshot_id_with_generator(catalog: &Catalog, generator: &str) -> Result<SnapshotId> {
    let mut identity = serde_json::to_vec(catalog)?;
    identity.extend_from_slice(generator.as_bytes());
    format!("eurorust-2026-{}", &digest(&identity)[..20]).try_into()
}

fn generator_fingerprint() -> String {
    digest(
        concat!(
            include_str!("domain.rs"),
            include_str!("collect.rs"),
            include_str!("export.rs"),
            include_str!("itinerary.rs"),
            include_str!("../../../Cargo.lock")
        )
        .as_bytes(),
    )
}

fn pretty(value: &impl Serialize) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}
fn cell(value: &str) -> String {
    value.replace('|', "\\|").replace(['\r', '\n'], " ")
}
pub fn markdown(catalog: &Catalog, pack: bool) -> Result<String> {
    let snapshot = snapshot_id(catalog)?;
    let mut out = format!(
        "# EuroRust 2026 {}\n\nGenerated from official schedule facts. Snapshot: `{}`.\nAll times are Europe/Madrid with explicit offsets (+02:00 during the event).\nUnknown prerequisites, remote access, tickets and recording availability remain unknown.\nRecheck official pages before attending. Community summaries are attributed interpretations.\n\n",
        if pack { "conference pack" } else { "schedule" },
        snapshot.as_str()
    );
    let mut day = String::new();
    for session in &catalog.sessions {
        let date = session.slot.start().datetime().date_naive().to_string();
        if date != day {
            day = date;
            out.push_str(&format!("\n## {day}\n\n"));
            if !pack {
                out.push_str(
                    "| Start | End | Session | Stage | ID |\n| --- | --- | --- | --- | --- |\n",
                );
            }
        }
        let start = session.slot.start().datetime().format("%H:%M");
        let end = session
            .slot
            .end()
            .map(|t| t.datetime().format("%H:%M").to_string())
            .unwrap_or_else(|| "unknown".into());
        if !pack {
            out.push_str(&format!(
                "| {start} | {end} | [{}]({}) | {} | `{}` |\n",
                cell(session.title.as_str()),
                session.official_url.as_str(),
                session.stage.as_ref().map_or("unknown", StageName::as_str),
                session.id.as_str()
            ));
            continue;
        }
        out.push_str(&format!("### {}\n\n- ID: `{}`\n- Kind: {:?}\n- Start: {}\n- End: {}\n- Stage: {}\n- Official source: {}\n- Retrieved: {}\n", session.title.as_str(), session.id.as_str(), session.kind, session.slot.start().as_rfc3339(), session.slot.end().map(Timestamp::as_rfc3339).unwrap_or_else(||"unknown".into()), session.stage.as_ref().map_or("unknown", StageName::as_str), session.official_url.as_str(), session.source.retrieved_at.as_rfc3339()));
        let names: Vec<_> = session
            .speaker_ids
            .iter()
            .filter_map(|id| catalog.speakers.iter().find(|s| &s.id == id))
            .map(|s| s.name.as_str())
            .collect();
        out.push_str(&format!("- Speakers: {}\n- Venue: {}\n- Individual remote access: {}\n- Booking status: unknown; check official registration\n", if names.is_empty() { "not listed".into() } else { names.join(", ") }, session.venue_id.as_ref().and_then(|id|catalog.venues.iter().find(|v|&v.id==id)).map_or("unknown; check source", |v| v.name.as_str()), session.remote_access.as_ref().map_or("unknown", AccessNote::as_str)));
        if let Some(annotation) = &session.annotation {
            out.push_str(&format!("- Annotation status: {:?}\n- Annotation source checked: {}\n- Annotation source SHA-256: `{}`\n",annotation.review_status,annotation.source_checked_at.as_rfc3339(),annotation.source_content_sha256.as_str()));
            if annotation.review_status == AnnotationReview::SourceChanged {
                out.push_str("\n**The source changed after this annotation. The following interpretation and setup/prerequisite text are historical; verify the current official page before using them.**\n\n");
            }
            out.push_str(&format!("- Topics (community): {}\n- Prerequisites (source-based paraphrase): {}\n- Setup (source-based paraphrase): {}\n\n{}\n\nAttribution: {}; method: {}; source: {}\n", annotation.topics.iter().map(Topic::as_str).collect::<Vec<_>>().join(", "), annotation.prerequisites.as_ref().map_or("unknown", Prerequisite::as_str), annotation.setup.as_ref().map_or("unknown", SetupRequirement::as_str), annotation.summary.as_str(), annotation.author.as_str(), annotation.method.as_str(), annotation.source_url.as_str()));
        } else {
            out.push_str("\nSummary and prerequisites: unknown; follow the official source.\n");
        }
    }
    Ok(out)
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| Error::Validation("output parent missing".into()))?;
    fs::create_dir_all(parent)?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    temp.write_all(bytes)?;
    temp.as_file().sync_all()?;
    temp.persist(path).map_err(|e| e.error)?;
    Ok(())
}

/// Prepare a complete immutable snapshot, then update convenience views and publish its manifest last.
pub fn publish(snapshot: ValidatedSnapshot, root: &Path) -> Result<Manifest> {
    let catalog = snapshot.catalog();
    let id = snapshot_id(catalog)?;
    let mut files = BTreeMap::new();
    files.insert("data/catalog.json".to_owned(), pretty(catalog)?);
    files.insert(
        "data/conference.json".to_owned(),
        pretty(&catalog.conference)?,
    );
    files.insert("data/sessions.json".to_owned(), pretty(&catalog.sessions)?);
    files.insert("data/speakers.json".to_owned(), pretty(&catalog.speakers)?);
    files.insert("data/venues.json".to_owned(), pretty(&catalog.venues)?);
    files.insert(
        "schema/catalog.schema.json".to_owned(),
        pretty(&schemars::schema_for!(Catalog))?,
    );
    files.insert(
        "schema/profile.schema.json".to_owned(),
        pretty(&schemars::schema_for!(crate::itinerary::Profile))?,
    );
    files.insert(
        "schema/itinerary.schema.json".to_owned(),
        pretty(&schemars::schema_for!(crate::itinerary::Itinerary))?,
    );
    files.insert(
        "exports/schedule.md".to_owned(),
        markdown(catalog, false)?.into_bytes(),
    );
    files.insert(
        "exports/conference-pack.md".to_owned(),
        markdown(catalog, true)?.into_bytes(),
    );
    let mut jsonl = Vec::new();
    for session in &catalog.sessions {
        serde_json::to_writer(&mut jsonl, session)?;
        jsonl.push(b'\n');
    }
    files.insert("exports/sessions.jsonl".to_owned(), jsonl);
    let manifest = Manifest {
        snapshot_id: id.clone(),
        schema_version: catalog.schema_version.clone(),
        generator_version: env!("CARGO_PKG_VERSION").to_owned().try_into()?,
        generator_source_sha256: generator_fingerprint().try_into()?,
        validated_at: Timestamp::now(),
        authoritative_catalog: format!("data/snapshots/{}/data/catalog.json", id.as_str()),
        session_count: catalog.sessions.len(),
        detail_count: catalog
            .sessions
            .iter()
            .filter(|s| s.detail_source.is_some())
            .count(),
        files: files
            .iter()
            .map(|(path, bytes)| (path.clone(), digest(bytes)))
            .collect(),
    };
    let snapshots = root.join("data/snapshots");
    fs::create_dir_all(&snapshots)?;
    let destination = snapshots.join(id.as_str());
    if !destination.exists() {
        let stage = tempfile::tempdir_in(&snapshots)?;
        for (path, bytes) in &files {
            atomic_write(&stage.path().join(path), bytes)?;
        }
        atomic_write(&stage.path().join("manifest.json"), &pretty(&manifest)?)?;
        fs::rename(stage.path(), &destination)?;
    }
    for (path, bytes) in &files {
        atomic_write(&root.join(path), bytes)?;
    }
    atomic_write(&root.join("data/manifest.json"), &pretty(&manifest)?)?;
    Ok(manifest)
}

pub struct VerifiedBundle {
    manifest: Manifest,
    snapshot: ValidatedSnapshot,
}
impl VerifiedBundle {
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
    pub fn catalog(&self) -> &Catalog {
        self.snapshot.catalog()
    }
}

pub fn verify(root: &Path) -> Result<VerifiedBundle> {
    let manifest: Manifest = serde_json::from_slice(&fs::read(root.join("data/manifest.json"))?)?;
    let required = [
        "data/catalog.json",
        "data/conference.json",
        "data/sessions.json",
        "data/speakers.json",
        "data/venues.json",
        "schema/catalog.schema.json",
        "schema/profile.schema.json",
        "schema/itinerary.schema.json",
        "exports/schedule.md",
        "exports/conference-pack.md",
        "exports/sessions.jsonl",
    ];
    if manifest.files.len() != required.len()
        || required
            .iter()
            .any(|path| !manifest.files.contains_key(*path))
    {
        return Err(Error::Validation(
            "manifest must cover every published data/schema/export view".into(),
        ));
    }
    let authoritative = root.join(&manifest.authoritative_catalog);
    if manifest.authoritative_catalog
        != format!(
            "data/snapshots/{}/data/catalog.json",
            manifest.snapshot_id.as_str()
        )
    {
        return Err(Error::Validation("invalid snapshot path".into()));
    }
    let checked = load(&authoritative)?;
    if snapshot_id_with_generator(checked.catalog(), manifest.generator_source_sha256.as_str())?
        != manifest.snapshot_id
        || checked.catalog().sessions.len() != manifest.session_count
        || checked.catalog().schema_version != manifest.schema_version
        || checked
            .catalog()
            .sessions
            .iter()
            .filter(|s| s.detail_source.is_some())
            .count()
            != manifest.detail_count
    {
        return Err(Error::Validation("manifest identity/count mismatch".into()));
    }
    let snapshot_root = authoritative
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| Error::Validation("snapshot parent missing".into()))?;
    for (path, hash) in &manifest.files {
        if path.contains("..") || Path::new(path).is_absolute() {
            return Err(Error::Validation("invalid manifest file path".into()));
        }
        if digest(&fs::read(root.join(path))?) != *hash
            || digest(&fs::read(snapshot_root.join(path))?) != *hash
        {
            return Err(Error::Validation(format!("export digest mismatch: {path}")));
        }
    }
    let live = load(&root.join("data/catalog.json"))?;
    if snapshot_id_with_generator(live.catalog(), manifest.generator_source_sha256.as_str())?
        != manifest.snapshot_id
    {
        return Err(Error::Validation(
            "active catalog differs from snapshot".into(),
        ));
    }
    Ok(VerifiedBundle {
        manifest,
        snapshot: checked,
    })
}

pub fn diff(old: &ValidatedSnapshot, new: &ValidatedSnapshot) -> Result<serde_json::Value> {
    let normalize = |s: &Session| -> Result<serde_json::Value> {
        let mut value = serde_json::to_value(s)?;
        value
            .as_object_mut()
            .expect("session object")
            .remove("source");
        for field in ["source", "detail_source"] {
            if let Some(object) = value.get_mut(field).and_then(|v| v.as_object_mut()) {
                object.remove("retrieved_at");
                object.remove("source_updated_at");
            }
        }
        Ok(value)
    };
    let old: BTreeMap<_, _> = old
        .catalog()
        .sessions
        .iter()
        .map(|s| Ok((s.id.as_str(), normalize(s)?)))
        .collect::<Result<_>>()?;
    let new: BTreeMap<_, _> = new
        .catalog()
        .sessions
        .iter()
        .map(|s| Ok((s.id.as_str(), normalize(s)?)))
        .collect::<Result<_>>()?;
    Ok(serde_json::json!({
        "added": new.keys().filter(|id|!old.contains_key(**id)).collect::<Vec<_>>(),
        "removed": old.keys().filter(|id|!new.contains_key(**id)).collect::<Vec<_>>(),
        "changed": new.iter().filter(|(id,value)|old.get(**id).is_some_and(|old|old!=*value)).map(|(id,_)|id).collect::<Vec<_>>()
    }))
}
