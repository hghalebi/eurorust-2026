# Validation evidence

For the active dataset, read the [manifest](../data/manifest.json) and per-record
provenance. For the current code revision, inspect its completed
[GitHub Actions run](https://github.com/hghalebi/eurorust-2026/actions). The receipts
below record checks at specific times and revisions. Use
[the contribution checks](../CONTRIBUTING.md#checks) to reproduce them.

## Speaking, history, and organizer routes — 2026-10-16

Local verification on macOS arm64, Rust/Cargo 1.98.0, at
`bea36b0a822d886a1428fa31fbd6da8dee17a632`:

- Formatting, Clippy with warnings denied, 39 unit/integration tests, and 17
  rustdoc cases passed, including nine compile-fail cases.
- The installed release helper passed catalog, learning, archive, participation,
  reader-resource, documentation, and all four itinerary checks. Navigation
  covered 228 public Markdown files, local links, anchors, and classification.
- A fresh local clone of that commit contained no private plans, caches,
  binaries, or build output. It built and installed both helpers, passed the
  same checks, reproduced the documented calendar export, and exercised the
  Rust experiments' success and failure exits. Its tracked files stayed clean.
- Independent JSON Schema, hash, and iCalendar checks matched 27 reader-pack
  inputs, eight generated outputs, three 18-session calendars, and the separate
  14-entry event/deadline calendar. Dates, offsets, escaping, UTF-8 line folding,
  and parse/write/parse round trips passed.
- Calendar command probes confirmed that synthetic profile names, goals, and
  reasons are omitted. Stale snapshots, empty selections, unknown end times,
  and overlaps failed without replacing the existing output. A filesystem
  write failure was followed by a successful export to a valid path.
- GitHub's Markdown API rendered the README, two reader-resource indexes, day
  guide, and coverage table. It exposed stripped colon-fragment links; the
  repaired format preserved all 34 source links and their matching targets.
- Archive checks and an independent count found 22 series or tracks, 62
  editions, 662 appearances, 230 material URLs, and 603 speaker-profile/source
  URLs. These are indexed records and links, not unique works or people.

The [day guide](../exports/day-guide.md) contains 53 entries: 48 with a named
venue and five unconfirmed. It retains the logistics companion's original
review date and distinguishes stage labels from physical rooms. The official
catalog, immutable snapshots, and licenses were not changed.

Five independent simulated editorial lenses approved their inspected scope
after corrections. The six routes were walked through editorially; no human
user study or live agent evaluation was performed. Calendar-app imports, video
playback, mobile layout, and assistive technology remain untested. New source
reviews do not renew older archive or logistics evidence. Coverage gaps remain
visible in the [archive inventory](../resources/conferences/coverage.md).

This receipt records local and rendered-HTML checks. Match the published commit
to its completed Actions run for remote CI evidence.

## Reading routes and editorial corrections — 2026-10-15

Local verification on macOS arm64, Rust/Cargo 1.98.0, for the revision based on
`567c9f73ca5d4599e890e2d92c7a67871b6c88a9`:

- Formatting, Clippy with warnings denied, 26 unit/integration tests, and 11
  rustdoc cases passed, including six compile-fail cases.
- The installed release helper passed catalog, learning, archive, documentation,
  and all four itinerary checks. The documentation check covered 192 public
  Markdown files, local paths, anchors, and classification.
- A clean checkout of the staged public files built successfully. Its installed
  executable passed the same data, generated-view, document, and itinerary
  checks without private plans or caches.
- GitHub's Markdown API rendered the README, new reading guide, itinerary
  template, and edition inventory. A secondary checker's RustWeek anchor warning
  was checked against GitHub's rendered HTML: the existing anchor is correct.
- The new snapshot, `eurorust-2026-71f2ee4aea59a1589532`, corrects one community
  setup annotation and adds a schedule navigation link. Official session fields,
  retrieval dates, and existing immutable snapshots are unchanged. Companion
  check dates retain their original scope.

Five simulated editorial reviews found no remaining major defect in their
inspected scope. This is not human community approval or exhaustive source
verification. External source pages were sampled; full books, video playback,
mobile layout, and assistive technology were not tested. Match this revision to
its completed Actions run for remote CI evidence.

## Conference archives and reading lists — 2026-10-15

Local verification on macOS arm64, Rust/Cargo 1.98.0, for the archive addition
based on `24fffd4f8286aa452ccc05d9daf9a13404b51733`:

- All 39 EuroRust talks and workshops have checked resource inputs: 150
  recommendations linking to 128 distinct destinations. Each session includes
  a book, article, paper, or video, alongside other relevant formats.
- Archive references, date bounds, speaker attribution, and programme counts
  passed: 21 series or tracks, 59 editions, and 651 indexed entries. A separate
  reconciliation matched the coverage ledger's 226 material links and 589
  speaker-profile references. These are link counts, not unique works or people.
- Workspace formatting, Clippy with warnings denied, 26 unit/integration tests,
  and 11 rustdoc cases passed, including six compile-fail cases. The release
  executable was installed locally and exercised. Generated views and hashes,
  the official dataset, 189 public Markdown files, and all four published
  itineraries passed their checks.
- Failure checks cover missing resource evidence, stale generated pages,
  incorrect speaker routes, misspelled archive fields, inconsistent dates,
  changed source content during refresh, and invalid input preserving output.
- A fresh public source checkout contained no private plan, cache, local binary,
  or build output. It built successfully, and its installed executable passed
  data, learning, archive, document, and all four itinerary checks. Both workflow
  YAML files and their embedded Bash passed syntax checks.

The official EuroRust catalog and immutable snapshots are unchanged. Source
inspection does not establish full-book review or video playback. The
[archive source ledger](../community/conference-archive/sources.json) distinguishes
543 fetched URLs, 555 source-linked URLs, and three access failures. The latter
are RUSTAsia's HTTP 503 response and two Rust Belt Rust TLS failures. Two RustConf
detail-page requests also timed out during resource link checks; its main
timetable was retrieved and used to reconcile dates. Coverage remains explicit
in the [edition inventory](../resources/conferences/editions.md).

This receipt records local checks. Match the exact published revision to its
completed Actions run for remote CI evidence.

## Documentation walkthrough — 2026-10-14

The revised [quick start](quickstart.md) was reproduced from a fresh public clone
at implementation baseline `dfbfe4c07577f74867ac6a71a690f0984bebe552` using
Rust/Cargo 1.98.0 on macOS arm64. This documentation change leaves that Rust
implementation and the published dataset unchanged.

- The exact README clone/build/validate/recommend/check sequence exited 0.
  Snapshot `eurorust-2026-bd9ff3d15e58a1b2d39f` contained 53 entries; the backend
  example produced 18 selections. Embedded and foundations profiles also
  produced 18 selections each, with distinct choices across the three profiles.
- The complete one-day profile in the walkthrough produced 8 selections and
  passed `check-itinerary`. Repository-local binary installation, the owner and
  agent examples, and all four published itineraries passed their checks.
- In a disposable clone, replacing `data/sessions.json` with an empty array made
  `validate` exit 1. The [recovery sequence](cli.md#recover-after-a-refresh-or-merge)
  restored the bundle, regenerated all four paths, and passed validation and
  itinerary checks. An unknown CLI flag exited 2.
- Workspace formatting, Clippy with warnings denied, all 17 unit/integration
  tests, and 9 rustdoc cases passed. The 5 compile-fail cases remain part of that
  evidence. A release build passed, and the input/retry examples produced their
  documented success and failure exits.
- Both documentation checkers passed on the 48-file public Markdown tree,
  including local links and anchors. The repository checker also verified the
  document inventory and private/historical routing boundaries. Neither checker
  validates every external web link or establishes reader comprehension.

The declared Rust 1.88 minimum was not separately retested. This walkthrough
performed no new live collector refresh, booking, social post, release, or
platform-support test beyond the stated environment. Historical receipts below
retain their original scope; match remote CI to the exact commit you use.

## Recorded checks — 2026-10-14

Local verification on 2026-10-14, macOS arm64, Rust/Cargo 1.98.0:

- Formatting and Clippy with warnings denied passed for the workspace.
- 17 unit/integration tests and 9 rustdoc cases passed, including 5 compile-fail
  cases. Negative cases include malformed source layout, invalid constructors,
  broken references, overlap, stale itinerary identity, corrupt exports, failed
  input preserving the last snapshot, and active-to-history/private doc routes.
- Release builds were installed into the local executable directory and exercised.
  Checked input succeeds; zero timeout exits 1. Simulated work recovers on attempt
  3 with budget 3; budget 2 exhausts with exit 2.
- A live official-source refresh discovered 53 entries: 31 talks, 8 workshops,
  9 side activities, and 5 service entries. All 47 on-origin linked detail pages
  were fetched. All 39 talk/workshop annotations are attributed source-checked
  drafts, with hashes and review dates; community review remains open.
- Snapshot integrity, public Markdown routing, and three distinct fictional
  participant paths with 18 selections each passed their checks.
- Offline hydration rejects stale annotation hashes unless explicitly marked
  `source_changed`; a rejected export preserves the active manifest. Readable
  output checks cover venue, preparation, exact slots, IDs, and snapshot evidence.
- Workflow YAML and embedded Bash syntax checks passed locally.

The manifest owns the exact dataset identity and validation time. Remote CI,
clean-clone verification, and release readback are recorded separately when
completed; a local result alone establishes none of those outcomes.

Use [the checks](../CONTRIBUTING.md#checks) to reproduce verification. Do not infer
remote publication, tickets, or attendance from a local check.

## Public delivery evidence

The reviewed implementation commit `82e6ca4625c5263a3957086a37ec95e4cdfdef2a`
passed [remote CI](https://github.com/hghalebi/eurorust-2026/actions/runs/38583741040).
A fresh clone of the public GitHub repository passed formatting, strict lint,
tests, release build, dataset validation, public document checks, and all four
published paths using its own compiled executables. It contained no private
planning folder. An independently supplied fictional one-day participant profile
produced 8 checked selections, different from the owner's 18-selection example;
optional workshops respected that profile's availability.

The repository is public, and GitHub's implementation commit matches local Git.
The remote tree contains no private planning, source cache, build output or local
executables. Starter [contribution issues](https://github.com/hghalebi/eurorust-2026/issues)
are published. For later changes, inspect the exact current commit in
[Actions](https://github.com/hghalebi/eurorust-2026/actions) and match a release's
build identity and download digests; the dated receipt above is historical evidence.
