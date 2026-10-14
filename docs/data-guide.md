# Use the data. Keep the evidence.

The public files let a participant browse the conference and a tool build a
traceable itinerary from the same source-linked facts. Start with the
[manifest](../data/manifest.json) for the active snapshot, coverage, validation
time, and file hashes. Use per-record provenance for source retrieval dates.
Use the [quick start](quickstart.md) to produce a plan; the
[CLI reference](cli.md) covers exact flags, profile fields, and scoring.

## Contract

Schema version 1.0.0 describes the [catalog](../data/catalog.json): conference,
sessions, speakers, and venues. The [JSON schema](../schema/catalog.schema.json)
is generated from Rust types. Runtime construction also checks intervals,
references, timezone offsets, schema version, and annotation consistency.

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

When a detail-page hash changes, refresh marks the annotation `source_changed`.
Review its previous summary and prerequisites against the current page. The
baseline stops using those stale topic tags. Offline loading rejects mismatched
hashes that still claim a checked status, or annotations without detail evidence.
The dataset supplies no inferred difficulty ratings, live seat counts, or
unsupported streaming claims.

## Source discovery and refresh

The inspected site exposes conference JSON-LD, a sitemap, schedule attributes,
and detail links. The collector reads the schedule attributes and fetches linked
pages on the official HTTPS origin. External activity URLs are retained as links.

Requests follow robots rules, wait at least one second apart, use
ETag/Last-Modified validators, and have bounded retries. Source HTML stays in the
ignored local cache. With [the binary installed](quickstart.md#install-a-reusable-binary),
retrieve and publish checked facts:

```sh
bin/conference-data refresh
bin/conference-data validate
```

`refresh` applies community annotations as it rebuilds the catalog. Missing
required fields, unsupported kinds, an empty schedule, failed detail fetches,
broken references, and disappeared sessions stop publication. Review removals
against the official site before using `refresh --accept-removals`; reconcile
annotations for removed sessions too.

## Snapshots and recovery

Each immutable bundle under `data/snapshots/` contains data, schemas, exports,
and a manifest. Its identity binds the catalog to the recorded generator-source
and dependency fingerprint. Data and generator versions are recorded separately.

Publication prepares the complete immutable bundle and renames its directory
atomically. It then replaces convenience views individually and writes the active
manifest last. Fetch, parsing, or validation failure leaves published files alone.
An interruption while replacing views can leave a digest mismatch; the old
manifest still identifies the immutable catalog used for recovery.

To rebuild from a checked catalog offline (after
[installing the binary](quickstart.md#install-a-reusable-binary)):

```sh
bin/conference-data export --catalog data/catalog.json
bin/conference-data validate
```

If active views are inconsistent, pass the immutable catalog path recorded in the
last intact manifest to `export --catalog PATH`. Validation compares both active
views and immutable files with their recorded hashes. This protocol does not
provide one filesystem transaction across all view files.

`export` retrieves no newer facts and applies no annotation-file edits. Git
history and release assets retain previous observations; keep their snapshot
identity when intentionally using historical data. After a source refresh or a
merge involving generated files, regenerate the example paths too. The
[complete recovery sequence](cli.md#recover-after-a-refresh-or-merge) keeps the
manifest, views, and all four paths together.

## Code and proof routes

| You are changing… | Owning source | Check the guarantee with… |
| --- | --- | --- |
| Domain values or lifecycle | [domain.rs](../tools/conference-data/src/domain.rs) | Constructors and compile-fail rustdoc |
| Fetching, robots, or extraction | [collect.rs](../tools/conference-data/src/collect.rs) | Synthetic HTML and malformed-layout tests |
| Publication or integrity | [export.rs](../tools/conference-data/src/export.rs) | Digest, recovery, and failed-publication integration tests |
| Itinerary decisions | [itinerary.rs](../tools/conference-data/src/itinerary.rs) | Availability, stale-snapshot, overlap, and profile scenarios |
| Public document routing | [navigation.rs](../tools/conference-data/src/navigation.rs) | Link checks and historical/private dependency fixtures |

Use the evidence for the claim you are making: a checked dataset, remote CI,
published release, confirmed booking, and observed conference lesson each need
their own proof. Find the current routes in the [task index](README.md).
