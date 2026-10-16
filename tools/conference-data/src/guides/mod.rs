//! Six reader routes and sourced practices, checked before producing shared views.
mod logistics;
mod model;
mod views;

use crate::{Error, Result, calendar, domain::*, export, invalid, itinerary};
use model::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::Path,
};

const SAMPLE_PLANS: [&str; 3] = ["foundations", "backend", "embedded"];

/// Private fields ensure a collection has passed local reference and date checks.
/// This state does not prove the external sources are still current.
/// ```compile_fail
/// use conference_data::guides::CheckedGuides;
/// let checked = CheckedGuides {};
/// ```
/// ```no_run
/// let checked = conference_data::guides::load(std::path::Path::new("."))?;
/// checked.verify(std::path::Path::new("."))?;
/// # Ok::<(), conference_data::Error>(())
/// ```
pub struct CheckedGuides {
    data: Collection,
    bundle: export::VerifiedBundle,
    inputs: BTreeMap<PublicPath, ContentHash>,
    calendars: BTreeMap<String, calendar::CheckedCalendar>,
    logistics: logistics::CheckedLogistics,
}

fn read_public(root: &Path, relative: &PublicPath) -> Result<Vec<u8>> {
    let canonical_root = root.canonicalize()?;
    let path = root.join(relative.as_str()).canonicalize()?;
    let contained = path
        .strip_prefix(&canonical_root)
        .map_err(|_| invalid("resource path", "symlink leaves repository"))?;
    // Check the resolved path too: a public alias must not lead into private data.
    PublicPath::try_from(contained.to_string_lossy().into_owned())?;
    Ok(fs::read(path)?)
}

fn distinct<'a, T: Ord + ?Sized + 'a>(
    items: impl Iterator<Item = &'a T>,
    kind: &'static str,
) -> Result<()> {
    let mut seen = BTreeSet::new();
    for item in items {
        if !seen.insert(item) {
            return Err(invalid(kind, "duplicate identity or reference"));
        }
    }
    Ok(())
}

pub fn load(root: &Path) -> Result<CheckedGuides> {
    let input: PublicPath = "community/event-resources.json".to_owned().try_into()?;
    let bytes = read_public(root, &input)?;
    let data: Collection = serde_json::from_slice(&bytes)?;
    let bundle = export::verify(root)?;
    if data.schema_version.as_str() != "1.0.0"
        || data.catalog_snapshot_id != bundle.manifest().snapshot_id
    {
        return Err(invalid(
            "resource version",
            "unsupported format or changed catalog; review the routes before updating their pin",
        ));
    }
    distinct(data.routes.iter().map(|r| &r.audience), "route audience")?;
    distinct(data.resources.iter().map(|r| &r.id), "resource ID")?;
    distinct(
        data.resources.iter().map(|r| r.url.as_str()),
        "resource URL",
    )?;
    distinct(data.related_collections.iter(), "related collection")?;
    if data.routes.len() != 6 || data.resources.is_empty() {
        return Err(invalid(
            "reader routes",
            "provide all six audiences and at least one resource",
        ));
    }
    for resource in &data.resources {
        let review = &resource.review;
        distinct(resource.audiences.iter(), "resource audience")?;
        if resource.audiences.is_empty()
            || review.limits.is_empty()
            || matches!(
                (review.status, resource.access),
                (ReviewStatus::Unavailable, Access::FreeWebPage)
            )
            || review.checked_on.0 > data.reference_date.0
            || review.next_review_on.0 <= data.reference_date.0
            || (review.next_review_on.0 - review.checked_on.0).num_days() > 90
            || resource
                .published_on
                .is_some_and(|d| d.0 > review.checked_on.0)
        {
            return Err(invalid(
                "resource review",
                format!(
                    "invalid dates, audience, or missing limits: {}",
                    resource.id.as_str()
                ),
            ));
        }
    }
    let mut paths = BTreeSet::new();
    for route in &data.routes {
        if route.steps.is_empty() || route.resource_ids.is_empty() {
            return Err(invalid(
                "reader route",
                "missing steps or resource references",
            ));
        }
        distinct(route.resource_ids.iter(), "route resource")?;
        for id in &route.resource_ids {
            if !data
                .resources
                .iter()
                .any(|r| &r.id == id && r.audiences.contains(&route.audience))
            {
                return Err(invalid(
                    "route resource",
                    format!("unknown resource or audience mismatch: {}", id.as_str()),
                ));
            }
        }
        for path in [&route.guide, &route.example, &route.template] {
            if !path.as_str().ends_with(".md") {
                return Err(invalid(
                    "route document",
                    "expected a Markdown guide, example, or template",
                ));
            }
            paths.insert(path.clone());
        }
    }
    paths.extend(data.related_collections.iter().cloned());
    let mut inputs = BTreeMap::from([(input, ContentHash::from_content(&bytes))]);
    for path in paths {
        inputs.insert(
            path.clone(),
            ContentHash::from_content(&read_public(root, &path)?),
        );
    }
    let logistics_path: PublicPath = "community/session-logistics.json".to_owned().try_into()?;
    let logistics_bytes = read_public(root, &logistics_path)?;
    let logistics = logistics::load(&logistics_bytes, &bundle)?;
    inputs.insert(logistics_path, ContentHash::from_content(&logistics_bytes));
    let mut calendars = BTreeMap::new();
    for name in SAMPLE_PLANS {
        let path: PublicPath = format!("community/paths/{name}-itinerary.json").try_into()?;
        let bytes = read_public(root, &path)?;
        let plan: itinerary::Itinerary = serde_json::from_slice(&bytes)?;
        calendars.insert(name.to_owned(), calendar::from_itinerary(&bundle, &plan)?);
        inputs.insert(path, ContentHash::from_content(&bytes));
    }
    Ok(CheckedGuides {
        data,
        bundle,
        inputs,
        calendars,
        logistics,
    })
}

impl CheckedGuides {
    /// Compute every output before writing. Each file is replaced atomically;
    /// verify detects a partial batch, and rerendering repairs it.
    pub fn render(&self, root: &Path) -> Result<()> {
        for (relative, bytes) in self.views()? {
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

    pub fn verify(&self, root: &Path) -> Result<()> {
        for (relative, expected) in self.views()? {
            if fs::read(root.join(&relative))? != expected {
                return Err(Error::Validation(format!(
                    "stale guide view: {relative}; run build-guides"
                )));
            }
        }
        println!(
            "Checked {} reader routes, {} sourced resources, day guide, and {} example calendars; offline only",
            self.data.routes.len(),
            self.data.resources.len(),
            self.calendars.len()
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests;
