# Use the conference data

Choose a format for your task. Keep its sources and dates with anything you make.

## Choose a format

| Task | File |
| --- | --- |
| Compare session times | [Schedule](../exports/schedule.md) |
| Read session context or give it to an agent | [Conference pack](../exports/conference-pack.md) |
| Use structured conference, session, speaker, and venue records | [Catalog JSON](../data/catalog.json) |
| Process one session per line | [Sessions JSONL](../exports/sessions.jsonl) |
| Check venues, rooms, and source conflicts | [Venue guide](time-and-place.md) · [Logistics JSON](../community/session-logistics.json) |
| Explore speakers' work | [Speaker directory](../resources/speakers/README.md) · [Profiles JSON](../community/speaker-profiles.json) |
| Read related talks and additional sources | [Session resources](../resources/related-talks/README.md) · [Learning data guide](learning-resources.md) |
| Explore other editions | [Conference archive](../resources/conferences/README.md) · [Edition data](../community/conference-archive/editions.json) |
| Check version and coverage | [Manifest](../data/manifest.json) |
| Check expected structure | [Catalog schema](../schema/catalog.schema.json) · [Participant preferences](../schema/profile.schema.json) · [Itinerary](../schema/itinerary.schema.json) |

Start with Markdown for reading and most agent conversations. The
[agent guide](agent-guide.md) explains how to add your goals.

## Take a copy

Read on GitHub or save the raw files. For a dated bundle, download
`eurorust-2026-context.zip` from [releases](https://github.com/hghalebi/eurorust-2026/releases)
and unzip it. No native tools are needed to read the context.

Use files from one checkout or bundle and keep the manifest. Releases are
frozen; the current repository may have newer information.

## Read dates and review status

A **snapshot** is a recorded dataset version. Its manifest lists coverage,
hashes, and validation time. Per-session source fields record when schedule
and detail pages were retrieved. Validation does not refresh those sources.

Community summaries are attributed interpretations. `source_checked_draft`
means checked against a recorded source, with community review still possible.
A changed detail page marks its annotation `source_changed`; review the summary
and prerequisites against the current page before relying on them.

Unknown values remain `null`. The data has no inferred difficulty ratings,
live seat counts, or unsupported streaming claims. Use official pages for full
abstracts and current access. See the [source policy](source-and-reuse-policy.md).

## Speaker research companion

[Profiles JSON](../community/speaker-profiles.json) and the
[speaker directory](../resources/speakers/README.md) cover 43 catalog speakers
and two speakers named on the official meetup page.

This separate document uses `format_version: "1.0.0"` and sits outside the
catalog schema and manifest hashes. Its `roster_snapshot_id` identifies the
catalog used for matching; it does not include profiles in that immutable
snapshot. Keep `checked_on` and the Git revision when reusing profiles.
Earlier release bundles may not include them.

| Field | Meaning |
| --- | --- |
| `coverage` | Catalog and additional meetup speaker counts |
| `curation` | Method, review status, policy, and limits |
| `profiles[].profile_id` | Local profile ID, distinct from a catalog speaker ID |
| `speaker_id` | Exact catalog join key; `null` for the two meetup speakers |
| `participation` | `catalog_speaker` or `meetup_speaker` |
| `page` | Repository-relative Markdown path |
| `checked_on`, `review_status` | Research date and draft status; community and speaker review are pending |
| `conference_sessions` | Session IDs, titles, and official URLs; get times from the catalog |
| `identity_evidence` | Public sources linking the person to their work and accounts |
| `items` | Selected accounts, blogs, books, projects, publications, talks, and professional work |
| `gaps` | What this search could not verify; absence does not establish nonexistence |

Items contain `kind`, `label`, `url`, `note`, and an attribution `evidence`
URL. Publications also specify `format`: `article`, `paper`, `preprint`,
`slides`, `thesis_record`, or `proceedings_chapter`. A paper link may lead to
an author's manuscript; check its publication page for venue and version.

Notes distinguish coauthorship, maintenance, contribution, future appearances,
and access limits. A listed account does not imply recent activity.
Profiles support reading suggestions, not schedule authority, bookings, or
permission to contact someone. [Suggest a correction](../CONTRIBUTING.md#speaker-profile-corrections).

## Session logistics check

[Logistics JSON](../community/session-logistics.json), format 1.0.0, records a
dated check of all 53 entries. It adds venue addresses, arrival details, missing
rooms, and source conflicts. The [venue guide](time-and-place.md) is its readable summary.

Join `sessions[].session_id` to the catalog and `venue_key` to `venues[].key`.
These venue keys belong to the companion; they are not additional catalog records.
`slot` and `stage` copy the checked catalog. `room: null` means no room assignment
was found in the checked sources. Keep location status and notes with each choice.

`source_ids` reference `sources`, which records URLs, retrieval times, and hashes.
`issues` preserves conflicts and excluded pages. The source-checked draft awaits
community and organizer review; it confirms neither bookings nor attendance.

Require a matching `catalog_snapshot_id` before combining it with a schedule.
It sits outside catalog schemas and manifest hashes. Keep `checked_at` and the
Git revision. Refresh/export do not update it; recheck sources before changing
its date or snapshot. Earlier release bundles may omit it.

## Contract

Catalog schema 1.0.0 covers conference, session, speaker, and venue records.
Schemas describe the serialized Rust types. The tools also check intervals,
references, timezone offsets, schema version, and annotation consistency.

| Field or value | Meaning |
| --- | --- |
| Session ID | Official URL path; title or time edits preserve URL-based IDs |
| Service ID | Time and label, because no detail page exists; a time change changes the ID |
| Speaker ID | Normalized name plus digest to distinguish non-ASCII names |
| `source` | Schedule-page provenance |
| `official_url` | Session source page |
| `detail_source` | Detail-page retrieval time and content hash |
| `annotation` | Attributed summary, topics, preparation, review status, checked date, and source hash |
| `null` | Unknown or unpublished here; not false |

## Source discovery and refresh

The collector reads the official schedule and linked session pages. It follows
robots rules, spaces requests, and stores full HTML in an ignored local cache.
External activity URLs remain links.

[Report a changed detail](../community/corrections/README.md) or follow the
[maintainer refresh procedure](maintenance.md#refresh-official-data).

## Snapshots and recovery

Immutable bundles in `data/snapshots/` retain data, schemas, exports, and
manifests. Snapshot identity binds the catalog to its generator and dependency
fingerprint. Data and generator versions are separate.

An itinerary names its snapshot. To check a historical plan, retain a complete
context bundle or checkout whose active manifest matches it. See
[version handling](cli.md#interpret-success-warnings-and-errors) and
[publication and recovery](maintenance.md#publish-and-recover-a-snapshot),
including file-replacement limits.

## Code and proof routes

Use the [maintainer task map](maintenance.md#find-the-owning-file) for tool changes.
[Validation receipts](validation.md) record checks at specific revisions.
Dataset checks, bookings, and observed lessons each need their own evidence.
