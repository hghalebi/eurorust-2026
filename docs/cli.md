# Optional CLI reference

For setup and an example, use the [walkthrough](quickstart.md).
Run commands from the repository root after
[installing `bin/conference-data`](quickstart.md#install-a-reusable-binary).
For reading and agent-assisted planning, start with the [guides](README.md).

## Commands and paths

| Command | Action | Result |
| --- | --- | --- |
| `validate` | Check the active bundle's rules, identity, coverage, references, and hashes | Reject inconsistent active or immutable files |
| `recommend --profile PATH --output PATH` | Validate the bundle and profile; select a deterministic draft | Write itinerary JSON; `--markdown PATH` adds a readable copy |
| `check-itinerary --itinerary PATH` | Check a plan against the verified active bundle | Reject identity, reference, availability, and overlap errors; warn about external facts |
| `export` | Validate the active catalog and regenerate offline | Update views and manifest; possibly add a snapshot |
| `export --catalog PATH` | Use a chosen valid catalog | Same publication process; useful for recovery |
| `refresh` | Fetch official sources, apply annotations, validate, and publish | Update facts and generated files |
| `check-docs` | Check public Markdown, links, anchors, classification, and private boundaries | Report navigation failures |
| `build-learning` | Check curated resource inputs, then render session/topic pages, schema, and portable exports offline | Preserve official catalog; rerun after interrupted output |
| `check-learning` | Check exact talk/workshop coverage, references, evidence fields, dates, snapshot join, and generated views | Reject missing resources or stale views; does not refresh sources |
| `rebind-learning` | Compare the reviewed session-content fingerprint with the active catalog, then update its snapshot join | Allows retrieval/schedule-only refreshes; content changes require review; research dates stay intact |
| `check-archive` | Check series/edition/session references, date bounds, and programme coverage counts | Reject inconsistent inventory; does not prove global completeness or playable recordings |
| `build-participation` | Validate pinned discovery/catalog references and community records; generate guides, schema, and calendar offline | Rewrite generated participation views; does not refresh sources |
| `check-participation` | Check pins, IDs, evidence fields, review dates, deadline offsets, and generated views | Reject inconsistent inputs or stale views; does not verify current access or import into an app |

Global `--root PATH` selects the dataset root. Profile, itinerary, output, and
explicit `--catalog` paths remain relative to the working directory unless
absolute. The cache defaults to `.cache/http` under the root; override it with
`--cache PATH`. Create output directories first.

Use `--help` on the binary or a subcommand for flags.
The audited tool version is `0.1.0`.

## Profile fields

Start with an [example](../community/paths/backend-profile.json).
The [schema](../schema/profile.schema.json) defines structure; runtime checks
enforce further constraints.

| Field | Meaning |
| --- | --- |
| `name` | Nonempty profile label |
| `goals` | Prose for people and agents; the CLI does not interpret it |
| `topics` | Nonempty ranking strings, such as `async`, `types`, `robotics`, or `infrastructure` |
| `available_windows` | RFC 3339 start/end pairs, with end after start; `[]` means unknown |
| `registered_workshops` | Distinct known workshop IDs reported as booked; `[]` selects no workshops |
| `attendance_mode` | `in_person`, `remote`, or `null`; unknown mode and unconfirmed remote access produce warnings |

Workshop IDs record your assertion; the tool cannot verify purchases or reserve
seats. Use explicit offsets: Europe/Madrid is `+02:00` during the event.
Keep real profiles local.

## How the baseline chooses

[itinerary.rs](../tools/conference-data/src/itinerary.rs) follows four steps:

1. Consider talks and registered workshops wholly inside an availability window.
   Unknown availability remains conditional.
2. Award one point per topic matching an annotation tag, except
   `source_changed` tags, or appearing in the title. Tag matching is
   case-sensitive; title matching ignores case.
3. Group by start time. Choose the highest score, breaking ties by session ID.
   Keep other candidates as same-slot alternatives.
4. Skip a group if its best candidate overlaps the previous choice or lacks an
   end time. Validate the result.

This greedy method does not find a global optimum. Slots without topic matches
may still get a choice and fallback reason. Scoring ignores prose goals,
experience, readiness, seats, and travel. Lunch, breaks, side activities, and
impl Day are not selected automatically. Markdown may list conditional workshops
without selecting them.

An agent can reason more broadly and submit JSON to `check-itinerary`.
See the [agent guide](agent-guide.md).

## Interpret success, warnings, and errors

`check-itinerary` checks snapshot identity, session references, distinct
selections, same-start alternatives, availability, end times, and overlaps.
It warns about unknown availability or mode, unconfirmed bookings or remote
access, changed annotations, and venue transfers.

Exit codes: **0** success, **1** validation/parsing/filesystem failure, **2** CLI
usage error from Clap. Exit 0 can include warnings; resolve them before attending.

```text
Schedule feasibility checked; no booking or attendance claim
```

For historical plans, use a complete retained checkout or release context with
a matching active manifest. The checker always uses the root's active bundle;
there is no snapshot-ID flag. An immutable folder has a root-level manifest,
while active contexts expect `data/manifest.json`. Pointing `--root` directly
at that folder will not work.

## Recover after a refresh or merge

Views, manifest, and example paths must share a snapshot. Mixed generations can
fail digest checks even when files parse. Preserve immutable history and choose
the catalog with the reviewed facts.

These commands rewrite generated files. Use a working copy and review the diff.
If `data/catalog.json` is the intended valid catalog:

```sh
bin/conference-data export --catalog data/catalog.json
for profile in community/paths/*-profile.json; do
  bin/conference-data recommend \
    --profile "$profile" \
    --output "${profile%-profile.json}-itinerary.json"
done
bin/conference-data recommend \
  --profile my-conference/profile.json \
  --output my-conference/itinerary.json \
  --markdown my-conference/itinerary.md
bin/conference-data validate
for itinerary in community/paths/*-itinerary.json my-conference/itinerary.json; do
  bin/conference-data check-itinerary --itinerary "$itinerary"
done
bin/conference-data check-docs
```

For a damaged active catalog, use the immutable catalog named by the last intact
manifest with `export --catalog PATH`. A newer generator may create a new
snapshot ID from the same facts; regenerate paths afterward. Offline export
neither applies annotation-file edits nor fetches newer facts.

For new sources, `refresh` fetches official pages and applies
`community/annotations/sessions.json`. Then regenerate and check paths as above.
Removals stop refresh unless reviewed and accepted with `--accept-removals`.
Review orphaned annotations. Read the
[refresh procedure](maintenance.md#refresh-official-data) first.

## Implementation and evidence

- [CLI dispatch](../tools/conference-data/src/main.rs): flags, paths, output, exit codes.
- [Domain validation](../tools/conference-data/src/domain.rs): values, references, intervals, annotation evidence.
- [Bundle verification](../tools/conference-data/src/export.rs): active and immutable hashes and identity. Hashes do not authenticate external announcements.
- [Integration tests](../tools/conference-data/tests/end_to_end.rs): distinct profiles, stale identity, overlaps, unavailable windows, corruption, recovery, stale annotations, and failed publication preserving the manifest.

See [executed checks](validation.md) and the
[maintainer task map](maintenance.md#find-the-owning-file).
