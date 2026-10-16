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
| Add communities or opportunities | [Participation JSON](../community/participation.json), [discovery JSON](../community/rust-discovery.json), and [renderer](../tools/conference-data/src/participation/mod.rs) | Source review, `build-participation`, `check-participation`, and `check-docs` |
| Improve speaking or organizer resources | [Shared resource JSON](../community/event-resources.json), [field guide](event-resources.md), and [renderer](../tools/conference-data/src/guides/mod.rs) | `build-guides`, `check-guides`, source review, and route walkthroughs |
| Export selected sessions | [Calendar adapter](../tools/conference-data/src/calendar.rs) and [usage](itinerary-calendar.md) | Snapshot/itinerary validation, privacy, known ends, and independent calendar parsing |
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
and review the diff. The refresh workflow opens a proposal only after its checks
pass. A changed participation or reader-resource pin stops it for editorial
reconciliation; it neither advances those pins nor merges automatically.

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

## Maintain communities and opportunities

Edit [participation.json](../community/participation.json) after checking primary
sources. Follow its [field reference](data-guide.md#community-participation-companion)
and the [contribution rules](../CONTRIBUTING.md#communities-and-participation).
Keep event dates in the discovery registry and application deadlines in the companion.

Review the discovery diff before updating `discovery_sha256`. This command reads
its current byte hash; it does not establish source accuracy:

```sh
shasum -a 256 community/rust-discovery.json
```

After a catalog refresh, review the four routes before changing the companion's
`catalog_snapshot_id`. Neither pin may be advanced merely to silence a check.
Keep earlier source dates unless those sources were inspected again. Update
`calendar_updated_at` when publishing a revised calendar edition, including
changed dates or cancelled items.

With the [compiled helper installed](quickstart.md#install-a-reusable-binary):

```sh
bin/conference-data build-participation
bin/conference-data check-participation
bin/conference-data check-docs
```

The build owns `community/README.md`, the communities and opportunities pages,
the participation schema, and `resources/stay-current/events.ics`. Edit their
source data or renderer, not the outputs. The participation guide, prompt, and
activity template are maintained by hand. Review wording, links, and deadlines
after regeneration.

| Record | Review interval |
| --- | --- |
| Event, call, or support deadline within 30 days | Weekly, and before recommending or exporting it |
| Later announced event or deadline | Monthly, and after a reported change |
| Community or contribution route | Within 90 days, or after a broken-link report; opportunity records currently use a shorter monthly interval |
| Historical observation | Preserve it; append dated corrections |
| Inaccessible or conflicting source | Keep the last supported fact and uncertainty; record the next attempt |

These are editorial review dates, not an installed monitoring job. The offline
check compares them with the snapshot reference date, not today's clock. Before
fresh recommendations or publication, inspect overdue records and dated facts.
Report added, changed, held, and retired entries; retain coverage gaps.

Calendar generation follows [RFC 5545](https://www.rfc-editor.org/rfc/rfc5545):
stable IDs, exclusive end dates, UTC for exact deadlines, escaped text, CRLF, and
folded UTF-8 lines. Tests cover date changes, cancellations, daylight-saving
offsets, duplicate identities, and invalid-input recovery. A cancelled record
leaves the next snapshot; it does not remove an old entry from someone's calendar.
App-specific import behavior needs a separate check.

All outputs are computed before writing and each file is replaced atomically.
A filesystem failure can leave a mixed set. Fix the cause, rebuild from reviewed
inputs, then run the checks. No automatic refresh, delivery, or subscription is supplied.

## Maintain reader resources

Edit [event-resources.json](../community/event-resources.json) and the linked
handwritten guides. The [resource guide](event-resources.md) describes the fields,
source labels, generated outputs, and 90-day review interval.

After a catalog change, review the six routes and sample itineraries before
updating this companion's `catalog_snapshot_id`. The participation companion has
its own pin and review. The [logistics companion](../community/session-logistics.json)
also needs a matching pin and reviewed session, venue, and source references.
Recheck actual source claims before changing any source
date; matching a new snapshot does not renew source evidence.

After these reviews, rebuild in this order:

```sh
bin/conference-data build-participation
bin/conference-data build-guides
bin/conference-data check-participation
bin/conference-data check-guides
bin/conference-data check-docs
```

The reader manifest hashes linked guides, templates, archive/discovery records,
and example itineraries. Rebuild when these inputs change. The pack preserves
the original source-review dates; archive coverage and current event status
still have their own checks. Generation is offline and each output is replaced
atomically. Rebuild and verify after an interrupted write.

The bounded conference refresh workflow does not refresh historical sources,
community opportunities, or reusable resource pages. A pin failure needs the
review above before a checked proposal can be published. See the failed job's
diff, reproduce the refresh locally, reconcile, and run the complete checks.

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
target/release/conference-data check-participation
target/release/conference-data check-guides
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
