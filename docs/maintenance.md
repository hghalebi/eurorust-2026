# Maintain the conference collection

Use this guide when updating the published data, documentation structure, or
supporting tools. For a correction, note, translation, or learning path, start
with [CONTRIBUTING](../CONTRIBUTING.md). For using the collection, start with
[the reader guide](README.md).

Keep the public experience centered on plans, notes, and reusable conference
context. Introduce installation or code only where a reader has chosen that task.

## Find the owning file

Choose the relevant guide section and input below. Read the owning source and
focused check, then expand only when needed.

| Task | Owning material | Guide and verification |
| --- | --- | --- |
| Correct or annotate a session | [Annotations](../community/annotations/sessions.json) and the official page | [Source policy](source-and-reuse-policy.md), source evidence, then refresh |
| Add or correct a speaker's work | [Speaker profiles JSON](../community/speaker-profiles.json) and [profile pages](../resources/speakers/README.md) | [Attribution and correction rules](../CONTRIBUTING.md#speaker-profile-corrections); keep both views consistent and verify catalog joins |
| Add a guide or change navigation | [Document inventory](document-map.json) and [navigation checker](../tools/conference-data/src/navigation.rs) | Update incoming links; run `check-docs`; retain historical/private-route fixtures |
| Change fetching, robots handling, or extraction | [Collector](../tools/conference-data/src/collect.rs) and [synthetic fixture](../tools/conference-data/tests/fixtures/schedule.html) | Valid/malformed parser cases, failed-publication checks, then `validate` |
| Change a schema, value, or lifecycle rule | [Domain types](../tools/conference-data/src/domain.rs) | Constructor tests and valid/compile-fail rustdoc pairs |
| Change export, integrity, or recovery | [Exporter](../tools/conference-data/src/export.rs) | Digest, recovery, and failed-publication integration tests |
| Change itinerary selection or checks | [Itinerary code](../tools/conference-data/src/itinerary.rs) | Availability, stale identity, overlap, and distinct-profile scenarios |
| Change command flags, paths, or output | [CLI entry point](../tools/conference-data/src/main.rs) | Keep the [reference](cli.md) and [optional walkthrough](quickstart.md) accurate; execute affected commands |
| Extend a learning example | [Experiments](../experiments/README.md) | State the learning goal, demonstrate success/failure, and explain the limits |

## Refresh official data

The inspected site exposes conference JSON-LD, a sitemap, schedule attributes,
and detail links. The collector reads schedule attributes and fetches linked
pages on the official HTTPS origin. External activity URLs are retained as links.

Requests follow robots rules, wait at least one second apart, use
ETag/Last-Modified validators, and have bounded retries. Full source HTML stays
in the ignored local cache. With the
[local helper installed](quickstart.md#install-a-reusable-binary), run:

```sh
bin/conference-data refresh
bin/conference-data validate
```

`refresh` retrieves official sources and applies community annotations. Missing
required fields, unsupported kinds, an empty schedule, failed detail fetches,
broken references, and disappeared sessions stop publication. Review removals
against official sources before using `refresh --accept-removals`; reconcile
annotations for removed sessions too.

When a detail-page hash changes, refresh marks the annotation `source_changed`.
The itinerary helper stops using its stale topic tags. Offline catalog loading
rejects mismatched hashes that still claim a checked status, or annotations
without detail evidence. Review the summary and prerequisites against the source.

After refreshing, follow the [complete regeneration sequence](cli.md#recover-after-a-refresh-or-merge)
to keep all example itineraries on the same snapshot. Review the resulting diff.
The refresh workflow opens a checked proposal; it does not merge it automatically.

Speaker research is edited separately. `refresh` and `export` do not update
`community/speaker-profiles.json` or its profile pages. When the roster changes,
compare speaker and session IDs, research any additions, and review the
companion's coverage and `roster_snapshot_id`. Preserve per-profile research
dates unless the underlying work was rechecked; a schedule refresh is not a
new review of a speaker's bibliography.

## Publish and recover a snapshot

Publication prepares the complete immutable bundle and renames its directory
atomically. It then replaces convenience views individually and writes the active
manifest last. Fetch, parsing, or validation failure leaves published files alone.
An interruption while replacing views can leave a digest mismatch; the old
manifest still identifies the immutable catalog used for recovery.

To rebuild from a reviewed, valid active catalog offline:

```sh
bin/conference-data export --catalog data/catalog.json
bin/conference-data validate
```

If the active catalog is damaged, pass the immutable catalog path recorded in
the last intact manifest to `export --catalog PATH`. Validation compares active
views and immutable files with their recorded hashes. Publication does not
provide one filesystem transaction across all view files.

`export` retrieves no newer facts and applies no annotation-file edits. After a
refresh, recovery, or merge of generated files, use the
[full regeneration sequence](cli.md#recover-after-a-refresh-or-merge) for the
bundle and all four example paths. Preserve immutable snapshots and deliberately
retained historical plans.

## Checks for maintainers

For a content change, verify official links, attribution, session references,
and participation context. If a document is new, moved, or reclassified, update
the inventory and incoming routes. The local navigation check is:

```sh
bin/conference-data check-docs
```

For Rust or dataset changes, run the existing workspace checks from the
repository root with a current stable Rust toolchain:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --release --locked
target/release/conference-data validate
target/release/conference-data check-docs
```

Run `check-itinerary` for each changed JSON path. Parser changes need a focused
fixture with valid and malformed input. Type guarantees need both valid use
and a compile-fail example that fails for the intended reason.

## Read evidence in context

| Claim | Evidence to use |
| --- | --- |
| Recorded version, coverage, and retrieval | [Manifest](../data/manifest.json) plus per-record provenance |
| Snapshot and export integrity | `validate` against the manifest and its immutable files |
| Checks performed at a particular revision | [Dated validation receipts](validation.md) |
| Current remote checks | [GitHub Actions](https://github.com/hghalebi/eurorust-2026/actions), matched to the exact commit and completed jobs |
| A published download | [Release](https://github.com/hghalebi/eurorust-2026/releases), with matching tag, build identity, and archive hashes |
| Conference participation or learning | [Journal](../journal/README.md) and attributed observations |

Local checks, remote CI, published files, bookings, and attendance establish
different facts. Keep dated receipts and immutable history; update current
navigation when newer evidence arrives.

## Runtime troubleshooting

If commands cannot start, try a small read-only probe such as `git status --short`.
A failure before shell execution belongs to the agent's command environment:
check its installation, running version, and process owner using its supported
recovery procedure. Prove recovery with a fresh command before drawing
conclusions about repository code.

If commands run and parsing fails, use the collector and fixture route above.
Keep machine-specific diagnostics local; contribute portable reproduction steps.
