# Maintain the conference collection

Use this guide to update data, navigation, or tools.
For corrections and notes, see [CONTRIBUTING](../CONTRIBUTING.md).
For reading and planning, use the [guide index](README.md).

Keep plans, notes, and conference context first. Introduce tools where readers
choose to use them.

## Find the owning file

Read the relevant source and focused checks before expanding the scope.

| Task | Owning material | Verification |
| --- | --- | --- |
| Correct a session | [Annotations](../community/annotations/sessions.json) and official page | [Source policy](source-and-reuse-policy.md), evidence, then refresh |
| Update speaker research | [Profiles JSON](../community/speaker-profiles.json) and [pages](../resources/speakers/README.md) | [Attribution rules](../CONTRIBUTING.md#speaker-profile-corrections); consistent views and catalog joins |
| Add session learning resources | [Curated JSON](../community/session-resources/) and [format guide](learning-resources.md) | `build-learning`, `check-learning`, and source evidence |
| Extend conference archives | [Registry and inventory](../community/conference-archive/) and [directory](../resources/conferences/README.md) | `check-archive`, source links, edition identity, programme counts, and coverage gaps |
| Check times and places | [Logistics JSON](../community/session-logistics.json) and [venue guide](time-and-place.md) | Compare official pages, snapshot, session IDs, times, and sources; preserve unknown rooms |
| Add or move a guide | [Inventory](document-map.json) and [navigation checker](../tools/conference-data/src/navigation.rs) | Incoming links, `check-docs`, historical/private-route fixtures |
| Change collection | [Collector](../tools/conference-data/src/collect.rs) and [fixture](../tools/conference-data/tests/fixtures/schedule.html) | Valid/malformed input, failed publication, `validate` |
| Change schema or lifecycle | [Domain types](../tools/conference-data/src/domain.rs) | Constructor tests; valid and compile-fail rustdoc pairs |
| Change export or recovery | [Exporter](../tools/conference-data/src/export.rs) | Digest, recovery, and failed-publication tests |
| Change itinerary logic | [Itinerary code](../tools/conference-data/src/itinerary.rs) | Availability, stale identity, overlaps, distinct profiles |
| Change flags or output | [CLI](../tools/conference-data/src/main.rs) | Update [reference](cli.md) and [walkthrough](quickstart.md); run affected commands |
| Extend an experiment | [Examples](../experiments/README.md) | Goal, success/failure cases, limits |

## Refresh official data

The inspected site exposes JSON-LD, a sitemap, schedule attributes, and detail
links. The collector reads schedule attributes and linked pages on the official
HTTPS origin. External activity URLs remain links.

Requests follow robots rules, wait at least one second apart, use
ETag/Last-Modified validators, and retry within limits.
Full HTML stays in the ignored cache.
With the [helper installed](quickstart.md#install-a-reusable-binary):

```sh
bin/conference-data refresh
bin/conference-data validate
```

`refresh` fetches sources and applies annotations. Missing required fields,
unsupported kinds, empty schedules, failed detail fetches, broken references,
or removed sessions stop publication. Check removals against official sources
before using `refresh --accept-removals`; reconcile their annotations too.

A changed detail-page hash marks its annotation `source_changed`, so ranking
ignores stale tags. Offline loading rejects mismatched hashes claiming checked
status and annotations lacking detail evidence. Review summaries and prerequisites
against the source.

Then [regenerate all example paths](cli.md#recover-after-a-refresh-or-merge)
and review the diff. The refresh workflow opens a checked proposal without
merging it automatically.

`refresh` and `export` leave speaker profiles untouched.
After roster changes, check speaker/session IDs, research additions, and review
coverage and `roster_snapshot_id`. Change profile research dates only when
rechecking the underlying work.

The [logistics check](data-guide.md#session-logistics-check) is also maintained
separately. Recheck it after a schedule refresh before updating its snapshot or
date. Keep the venue guide consistent and never infer rooms from track labels.

Learning resources also pin their reviewed catalog in
`community/learning-manifest.json`. After a changed snapshot, review session
joins and source changes before running `rebind-learning` and `build-learning`.
The rebind command accepts changed retrieval dates or scheduling when the
session content fingerprint matches. It stops for changed titles, speakers,
full detail-page hashes, or annotations; those require editorial review. Existing
source check dates stay intact. See the [learning guide](learning-resources.md).

## Publish and recover a snapshot

Publication builds the immutable bundle, then atomically renames its directory.
It replaces convenience views individually and writes the active manifest last.
Fetch, parse, or validation failure leaves published files unchanged.

An interruption during view replacement can leave mismatched hashes.
The old manifest still identifies the recovery catalog.
To rebuild offline from a reviewed, valid active catalog:

```sh
bin/conference-data export --catalog data/catalog.json
bin/conference-data validate
```

If that catalog is damaged, pass the immutable catalog named in the last intact
manifest to `export --catalog PATH`. Validation checks active and immutable
hashes. Publication is not one filesystem transaction across all views.

`export` neither fetches newer facts nor applies annotation-file edits.
After refresh, recovery, or merges of generated files, follow the
[regeneration sequence](cli.md#recover-after-a-refresh-or-merge) for the bundle
and all four paths. Preserve immutable snapshots and retained historical plans.

## Checks for maintainers

For content changes, check official links, attribution, session references,
and participation context. For new, moved, or reclassified documents, update the
inventory and incoming links.

```sh
bin/conference-data check-docs
```

For Rust or data changes, run from the repository root with stable Rust:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --release --locked
target/release/conference-data validate
target/release/conference-data check-learning
target/release/conference-data check-archive
target/release/conference-data check-docs
```

Run `check-itinerary` for each changed JSON path.
Parser changes need valid and malformed fixtures.
Type guarantees need valid use and a compile-fail example failing for the intended reason.

## Read evidence in context

| Claim | Evidence |
| --- | --- |
| Version, coverage, retrieval | [Manifest](../data/manifest.json) and per-record sources |
| Bundle integrity | `validate` against active and immutable files |
| Checks at a revision | [Dated receipts](validation.md) |
| Remote checks | [Actions](https://github.com/hghalebi/eurorust-2026/actions), exact commit and completed jobs |
| Published download | [Release](https://github.com/hghalebi/eurorust-2026/releases), matching tag, build, and hashes |
| Participation or learning | [Journal](../journal/README.md) and attributed observations |

Local checks, CI, publication, bookings, and attendance prove different facts.
Keep dated receipts and immutable history; update current navigation as needed.

## Runtime troubleshooting

If commands cannot start, try `git status --short`.
Failure before shell execution points to the command environment.
Check its installation, running version, and process owner through its supported
recovery procedure. Verify recovery with a fresh command.

If commands run but parsing fails, inspect the collector and fixtures.
Keep machine-specific diagnostics local; share portable reproduction steps.
