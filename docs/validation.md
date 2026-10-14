# Validation receipt

Local verification on 2026-10-14, macOS arm64, Rust/Cargo 1.98.0:

- Formatting and Clippy with warnings denied passed for the workspace.
- 14 unit/integration tests and 7 rustdoc cases passed, including 4 compile-fail
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
- Snapshot integrity, 36 public Markdown files, and three distinct fictional
  participant paths with 18 selections each passed their checks.
- Workflow YAML and embedded Bash syntax checks passed locally.

The manifest owns the exact dataset identity and validation time. Remote CI,
clean-clone verification, and release readback are recorded separately when
completed; a local result alone establishes none of those outcomes.

Use [the checks](../CONTRIBUTING.md#checks) to reproduce verification. Do not infer
remote publication, tickets, or attendance from a local check.
