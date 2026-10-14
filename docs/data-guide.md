# Dataset and collection guide

## Contract

Schema version 1.0.0 describes [the catalog](../data/catalog.json): conference,
sessions, speakers, and venues. [The schema](../schema/catalog.schema.json) is
generated from the Rust types. Runtime construction adds interval, reference,
timezone, version, and annotation-consistency checks beyond JSON shape.

Session IDs come from official URL paths; service IDs include their time and
label because those entries have no detail page. A service time change therefore
changes its ID. Speaker IDs combine a normalized name with a digest to retain
distinct non-ASCII names. Title/time edits preserve URL-based session IDs.

`source` is the schedule-page provenance for extracted fields. `official_url`
identifies the session page, and `detail_source` records its retrieval and hash.
`annotation` carries separately attributed original paraphrases and topic tags.
It records the source hash and check date. A changed detail page marks the
annotation `source_changed`; its previous prerequisites need review, and the
baseline stops using its topic tags.
Null means unknown or not published in this dataset, not false. There are no
invented difficulty ratings, live seat counts, or per-session streaming claims.

## Source discovery and refresh

The inspected site publishes conference-level JSON-LD, a sitemap, and schedule
elements with start/end offsets and detail links. No session feed is assumed.
The current collector uses those schedule attributes and visits linked official
detail pages. External side-activity links are retained without fetching them.
The origin is restricted to the official HTTPS host. It checks
robots rules, waits at least one second between requests, uses ETag/Last-Modified
cache validators, and bounds retries. Source HTML remains in the ignored cache.

```sh
bin/conference-data refresh
bin/conference-data validate
```

Missing fields, unsupported kinds, empty schedules, failed detail fetches, broken
references, and disappeared sessions fail the refresh before publication.
Review removals against the official site before using `refresh --accept-removals`.
Annotations whose sessions disappeared must be reviewed too.

## Snapshots and recovery

The manifest identifies an immutable snapshot under `data/snapshots/`. A snapshot
contains the catalog, individual data files, schemas, exports, and a manifest.
Snapshot identity binds catalog content and generator-source/dependency
fingerprint. Data and generator version are recorded separately.

The complete immutable bundle is prepared before an atomic directory rename.
Convenience views are replaced individually and the active manifest is written
last. Failed fetch/parse/validation leaves published data alone. An interruption
during view updates can leave a digest mismatch: read the old manifest's immutable
catalog and restore the views using `export --catalog PATH`. This is a file
publication protocol, not a claim of a cross-file filesystem transaction.

```sh
bin/conference-data export --catalog data/catalog.json
bin/conference-data validate
```

Validation checks both convenience views and immutable files against manifest
hashes. Git history and release assets retain earlier source-bound observations.
Offline export reruns validation but does not retrieve newer conference facts.

## Code and proof routes

| Concern | Owning source | Focused evidence |
| --- | --- | --- |
| Domain values and lifecycle | [domain.rs](../tools/conference-data/src/domain.rs) | Constructor tests and compile-fail rustdoc |
| Fetch, robots, extraction | [collect.rs](../tools/conference-data/src/collect.rs) | Synthetic HTML and malformed-layout tests |
| Publication and integrity | [export.rs](../tools/conference-data/src/export.rs) | Snapshot/digest and failed-input integration tests |
| Itinerary decisions | [itinerary.rs](../tools/conference-data/src/itinerary.rs) | Availability, stale-snapshot, overlap, and profile scenarios |
| Public document routing | [navigation.rs](../tools/conference-data/src/navigation.rs) | Link checks and active-to-history negative fixtures |

Freshness, attendance, bookings, prerequisites, CI, and published releases each
need their own evidence. A successful build establishes none of the other facts.
