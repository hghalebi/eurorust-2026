# Use the conference data

The collection offers the same conference context in readable and structured
forms. Choose the format that fits your task, then keep its source links and
dates with the plan or notes you make from it.

## Choose a format

| You want to… | Use this |
| --- | --- |
| Browse times and compare sessions | [Schedule](../exports/schedule.md) |
| Read session context or give it to an agent | [Conference pack](../exports/conference-pack.md) |
| Work with the complete structured dataset | [Catalog JSON](../data/catalog.json) |
| Process sessions one record at a time | [Sessions JSONL](../exports/sessions.jsonl) |
| Identify the recorded version and coverage | [Manifest](../data/manifest.json) |
| Understand the structure expected by your tools | [Catalog schema](../schema/catalog.schema.json), [profile schema](../schema/profile.schema.json), and [itinerary schema](../schema/itinerary.schema.json) |

Markdown is the easiest starting point for reading and most agent conversations.
JSON contains structured conference, session, speaker, and venue records. JSONL
contains one session record per line. The [agent guide](agent-guide.md) shows how
to combine the context with a participant's goals.

## Take a copy for your own use

Read the linked files on GitHub, or save their raw contents for a local tool or
an agent that accepts files. For a complete dated bundle, download
`eurorust-2026-context.zip` from [releases](https://github.com/hghalebi/eurorust-2026/releases)
and unzip it. That context contains documents and data you can use without
installing the separate native tools.

Use files from the same checkout or context bundle. A release is frozen;
the current repository may contain newer information. Keep the manifest with
your copy so another person or agent can identify the version you used.

## Understand dates and review status

A **snapshot** is a recorded version of the dataset. The manifest identifies it,
lists coverage and file hashes, and records validation time. Each session's
source fields record when its schedule and detail information were retrieved.
Validation time and source retrieval time describe different events.

Official links let you check the original announcement. Community summaries
are attributed interpretations with their own review status. A
`source_checked_draft` is a draft checked against a recorded source; community
review may still be pending. If the detail page changes, refresh marks the
annotation `source_changed`; review its previous summary and prerequisites
against the current page before relying on them.

Unknown values stay `null`. The dataset supplies no inferred difficulty ratings,
live seat counts, or unsupported streaming claims. Follow the official session
page for the full abstract and current access details. The
[source and reuse policy](source-and-reuse-policy.md) explains attribution and
what the collection publishes.

## Contract

Schema version 1.0.0 describes the [catalog](../data/catalog.json): conference,
sessions, speakers, and venues. Schemas are generated from Rust types and describe
the serialized form. The supporting tools also check intervals, references,
timezone offsets, schema version, and annotation consistency.

| Field or value | How to read it |
| --- | --- |
| Session ID | Derived from the official URL path. Title or time edits preserve URL-based IDs |
| Service ID | Includes time and label because service entries have no detail page; changing the time changes the ID |
| Speaker ID | Combines a normalized name with a digest to distinguish non-ASCII names |
| `source` | Schedule-page provenance for extracted facts |
| `official_url` | The session's source page |
| `detail_source` | Detail-page retrieval time and content hash |
| `annotation` | Attributed original summary, topics, known preparation, review status, checked date, and source hash |
| `null` | Unknown or not published in this dataset; it does not mean false |

## Source discovery and refresh

The dataset is collected from the official schedule and linked session pages.
The collector follows robots rules, waits between requests, and keeps full
source HTML in an ignored local cache. External activity URLs remain links.

To report a changed detail, use the [correction guide](../community/corrections/README.md).
Maintainers can follow the [collection procedure](maintenance.md#refresh-official-data)
to retrieve sources, apply annotations, and update the published files.

## Snapshots and recovery

Immutable bundles under `data/snapshots/` retain previous data, schemas, exports,
and manifests. A snapshot identity binds the catalog to the recorded generator
and dependency fingerprint. Data and generator versions are recorded separately.
An itinerary's snapshot ID says which version its choices refer to.

For a historical plan, retain a complete context bundle or checkout whose active
manifest matches that plan. The [optional CLI reference](cli.md#interpret-success-warnings-and-errors)
explains how the checker resolves versions. The
[maintenance guide](maintenance.md#publish-and-recover-a-snapshot) preserves the
publication and recovery details, including the limits of file replacement.

## Code and proof routes

For collection, schema, export, or documentation-tool changes, use the
[maintainer task map](maintenance.md#find-the-owning-file).
The [validation receipts](validation.md) record executed checks at specific
revisions. A checked dataset, confirmed booking, and observed conference lesson
each need evidence appropriate to that claim.
