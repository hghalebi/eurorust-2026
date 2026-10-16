use crate::{
    Error, Result,
    domain::{Author, Summary},
};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum DocumentClass {
    Current,
    Reference,
    Example,
    Historical,
    Experimental,
    Generated,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    path: PathBuf,
    class: DocumentClass,
    owner: Author,
    purpose: Summary,
    scope: Summary,
    replacement: Option<PathBuf>,
}
fn public_path(path: &Path) -> bool {
    !path.components().any(|c| {
        matches!(
            c.as_os_str().to_str(),
            Some(".plan" | ".cache" | ".git" | "target" | "bin")
        )
    })
}
fn headings(body: &str) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    let mut counts = BTreeMap::new();
    let mut title = String::new();
    let mut inside = false;
    for event in Parser::new(body) {
        match event {
            Event::Html(html) | Event::InlineHtml(html) => {
                // The generated resource indexes use explicit stable IDs. Only
                // recognize literal empty anchors, not code samples or comments.
                for line in html.lines() {
                    if let Some(id) = line
                        .trim()
                        .strip_prefix("<a id=\"")
                        .and_then(|s| s.strip_suffix("\"></a>").or_else(|| s.strip_suffix("\">")))
                        && !id.is_empty()
                        && id
                            .bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
                    {
                        result.insert(id.to_owned());
                    }
                }
            }
            Event::Start(Tag::Heading { .. }) => {
                inside = true;
                title.clear();
            }
            Event::Text(s) | Event::Code(s) if inside => title.push_str(&s),
            Event::End(TagEnd::Heading(_)) => {
                inside = false;
                let base: String = title
                    .to_lowercase()
                    .chars()
                    .filter_map(|c| {
                        if c == ' ' {
                            Some('-')
                        } else if c.is_alphanumeric() || c == '-' || c == '_' {
                            Some(c)
                        } else {
                            None
                        }
                    })
                    .collect();
                let count = counts.entry(base.clone()).or_insert(0);
                result.insert(if *count == 0 {
                    base
                } else {
                    format!("{base}-{count}")
                });
                *count += 1;
            }
            _ => {}
        }
    }
    result
}
fn check_class(from: &DocumentClass, to: &DocumentClass, label: &str) -> Result<()> {
    if from == &DocumentClass::Current
        && to == &DocumentClass::Historical
        && !["historical", "archive"]
            .iter()
            .any(|s| label.to_lowercase().contains(s))
    {
        return Err(Error::Validation(
            "active navigation targets history without an explicit historical label".into(),
        ));
    }
    Ok(())
}
pub fn check(root: &Path) -> Result<()> {
    let inventory: Vec<Document> =
        serde_json::from_slice(&fs::read(root.join("docs/document-map.json"))?)?;
    let mut classes = BTreeMap::new();
    for doc in &inventory {
        if !public_path(&doc.path)
            || doc.path.is_absolute()
            || doc
                .path
                .components()
                .any(|c| c == std::path::Component::ParentDir)
        {
            return Err(Error::Validation(
                "private or escaping document inventory path".into(),
            ));
        }
        if classes
            .insert(doc.path.clone(), doc.class.clone())
            .is_some()
        {
            return Err(Error::Validation("duplicate inventory document".into()));
        }
        if !root.join(&doc.path).is_file() {
            return Err(Error::Validation(format!(
                "inventory file missing: {}",
                doc.path.display()
            )));
        }
        let _metadata = (&doc.owner, &doc.purpose, &doc.scope);
        if let Some(replacement) = &doc.replacement
            && (!public_path(replacement) || !root.join(replacement).is_file())
        {
            return Err(Error::Validation(
                "replacement guide missing/private".into(),
            ));
        }
    }
    let root = root.canonicalize()?;
    let mut count = 0;
    for entry in walkdir::WalkDir::new(&root)
        .into_iter()
        .filter_entry(|e| public_path(e.path().strip_prefix(&root).unwrap_or(e.path())))
    {
        let entry = entry.map_err(|e| Error::Validation(e.to_string()))?;
        if !entry.file_type().is_file() || entry.path().extension().is_none_or(|e| e != "md") {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(&root)
            .map_err(|e| Error::Validation(e.to_string()))?;
        let generated = relative.starts_with("data/snapshots");
        if !generated && !classes.contains_key(relative) {
            return Err(Error::Validation(format!(
                "document not classified: {}",
                relative.display()
            )));
        }
        let body = fs::read_to_string(entry.path())?;
        if body.contains("/Users/") || body.contains("file://") {
            return Err(Error::Validation(format!(
                "machine-local material in {}",
                relative.display()
            )));
        }
        let mut target = None;
        let mut label = String::new();
        for event in Parser::new(&body) {
            match event {
                Event::Start(Tag::Link { dest_url, .. })
                | Event::Start(Tag::Image { dest_url, .. }) => {
                    target = Some(dest_url.to_string());
                    label.clear();
                }
                Event::Text(s) | Event::Code(s) if target.is_some() => label.push_str(&s),
                Event::End(TagEnd::Link | TagEnd::Image) => {
                    let Some(url) = target.take() else { continue };
                    if url.starts_with("https://")
                        || url.starts_with("http://")
                        || url.starts_with("mailto:")
                    {
                        continue;
                    }
                    let (path, anchor) = url.split_once('#').unwrap_or((&url, ""));
                    if path.starts_with('/') || path.contains(':') {
                        return Err(Error::Validation(format!(
                            "absolute/local link in {}: {url}",
                            relative.display()
                        )));
                    }
                    let destination = if path.is_empty() {
                        entry.path().to_path_buf()
                    } else {
                        entry.path().parent().expect("file parent").join(path)
                    };
                    let destination = if destination.is_dir() {
                        destination.join("README.md")
                    } else {
                        destination
                    };
                    let destination = destination.canonicalize().map_err(|_| {
                        Error::Validation(format!("broken link in {}: {url}", relative.display()))
                    })?;
                    let dest_relative = destination
                        .strip_prefix(&root)
                        .map_err(|_| Error::Validation("link escapes repository".into()))?;
                    if !public_path(dest_relative) {
                        return Err(Error::Validation(
                            "public link depends on private content".into(),
                        ));
                    }
                    if !anchor.is_empty()
                        && destination.extension().is_some_and(|e| e == "md")
                        && !headings(&fs::read_to_string(&destination)?).contains(anchor)
                    {
                        return Err(Error::Validation(format!(
                            "missing anchor in {}: {url}",
                            relative.display()
                        )));
                    }
                    if let (Some(from), Some(to)) =
                        (classes.get(relative), classes.get(dest_relative))
                    {
                        check_class(from, to, &label)?;
                    }
                }
                _ => {}
            }
        }
        count += 1;
    }
    println!("Checked {count} public Markdown files, local links, anchors, and document classes");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn active_history_route_needs_label() {
        assert!(
            check_class(
                &DocumentClass::Current,
                &DocumentClass::Historical,
                "Continue deployment"
            )
            .is_err()
        );
        assert!(
            check_class(
                &DocumentClass::Current,
                &DocumentClass::Historical,
                "Historical deployment record"
            )
            .is_ok()
        );
    }
    #[test]
    fn headings_follow_github_and_preserve_duplicates() {
        let anchors = headings("# First: **hello**\n## First: hello\n");
        assert!(anchors.contains("first-hello"));
        assert!(anchors.contains("first-hello-1"));
        assert!(!public_path(Path::new(".plan/internal.md")));
    }
    #[test]
    fn explicit_resource_anchors_work_but_code_and_comments_do_not() {
        let anchors = headings(
            "<a id=\"source.one\"></a>\n\n<!-- <a id=\"comment\"></a> -->\n\n```html\n<a id=\"sample\"></a>\n```\n",
        );
        assert!(anchors.contains("source.one"));
        assert!(!anchors.contains("comment"));
        assert!(!anchors.contains("sample"));
    }
}
