# Validation evidence

For the active dataset, read the [manifest](../data/manifest.json) and per-record
provenance. For the current code revision, inspect its completed
[GitHub Actions run](https://github.com/hghalebi/eurorust-2026/actions). The receipts
below record checks at specific times and revisions. Use
[the contribution checks](../CONTRIBUTING.md#checks) to reproduce them.

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
