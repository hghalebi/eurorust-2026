# Optional CLI reference: inputs, choices, checks, and recovery

This reference is for the local helper that supports the conference collection.
Use the [optional walkthrough](quickstart.md) for its setup and a complete example.
For reading the collection or planning with your own agent, use the
[guide index](README.md). The sections below explain what each command proves
and which decisions remain with the participant.
Run examples from the repository root after [installing `bin/conference-data`](quickstart.md#install-a-reusable-binary).

## Commands and paths

| Command | Reads or does | Result |
| --- | --- | --- |
| `validate` | Loads the active manifest and catalog; checks domain rules, identity, coverage, references, and hashes | Rejects inconsistent active or immutable files |
| `recommend --profile PATH --output PATH` | Validates the active bundle and profile, then selects a deterministic starting path | Writes itinerary JSON; optional `--markdown PATH` adds the readable view |
| `check-itinerary --itinerary PATH` | Checks a supplied plan against the active verified bundle | Rejects identity, reference, availability, and overlap errors; prints external-fact warnings |
| `export` | Loads the active catalog, validates it, and regenerates a bundle offline | Updates generated views and manifest; may add an immutable snapshot |
| `export --catalog PATH` | Uses an explicitly selected checked catalog | Same publication path; useful for recovery |
| `refresh` | Fetches official sources, applies annotations, validates, and publishes | Retrieves newer facts and updates generated files |
| `check-docs` | Checks public Markdown, local links, anchors, classification, and private-path boundaries | Reports structural navigation failures |

`--root PATH` is global and selects the dataset/repository root. Profile, itinerary,
output, and explicit `--catalog` paths remain relative to the shell's working
directory unless absolute. The refresh cache defaults to `.cache/http` under the
selected root; `--cache PATH` changes that location. Output directories must exist.

Use `bin/conference-data --help` or append `--help` to a subcommand for exact flags.
The tool reports version `0.1.0` at the audited revision.

## Profile fields

The [profile schema](../schema/profile.schema.json) defines JSON shape; runtime
checks enforce additional constraints. Start with a complete
[example profile](../community/paths/backend-profile.json).

| Field | Meaning and behavior |
| --- | --- |
| `name` | A nonempty label for the profile |
| `goals` | Descriptive strings for people and agents; the deterministic CLI does not interpret them |
| `topics` | Nonempty strings used for ranking. Use existing annotation tags such as `async`, `types`, `robotics`, or `infrastructure` |
| `available_windows` | RFC 3339 start/end pairs. Each window needs a known end after its start; `[]` means availability is unknown |
| `registered_workshops` | Distinct known workshop IDs the participant reports as registered. `[]` causes the baseline to select no workshops |
| `attendance_mode` | `in_person`, `remote`, or `null`. Unknown mode and unconfirmed remote access produce warnings |

An entered workshop ID records the participant's assertion; the tool cannot check
a purchase or reserve a seat. Use explicit timezone offsets. These guides use
Europe/Madrid, `+02:00` during the event. Real profiles belong in local files.

## How the baseline chooses

The implementation is in [itinerary.rs](../tools/conference-data/src/itinerary.rs).
It performs these steps:

1. Consider talks and workshop IDs listed as registered. Keep sessions wholly
   inside a supplied availability window; unknown availability remains conditional.
2. Give one point per requested topic when it exactly matches an annotation tag
   whose status is not `source_changed`, or appears in the title ignoring case.
   Annotation tag matching itself is case-sensitive.
3. Group candidates by start time. Choose the highest score; break ties by
   session ID. Keep the other candidates as same-slot alternatives.
4. Move forward through slots. Skip a group if its best candidate overlaps the
   previous choice or has no known end. Validate the resulting itinerary.

This is a greedy baseline, not a global optimizer. Unmatched slots can still get
a selection and a fallback rationale. Prose goals, skill level, prerequisite
readiness, seat availability, and travel durations do not influence scoring.
Lunch, breaks, side activities, and impl Day are not automatically selected.
Markdown can list conditional workshop candidates without selecting them.

An agent can use the same source data to reason more broadly, then submit its
JSON to `check-itinerary`. The [agent guide](agent-guide.md) defines that contract.

## Interpret success, warnings, and errors

`check-itinerary` checks the plan's snapshot, session references, distinct
selections, same-start alternatives, availability, known end times, and overlap.
It warns about unknown availability or mode, unconfirmed workshop registration,
remote access, changed annotations, and venue transfers as applicable.

A successful command exits 0. Validation, parsing, and filesystem errors exit 1;
CLI argument/usage errors are handled by Clap and exit 2. Warnings can accompany
exit 0. They are unresolved participant decisions, not successful bookings.

The final message is deliberately narrow:

```text
Schedule feasibility checked; no booking or attendance claim
```

For a historical itinerary, use a complete retained checkout or release context
whose active manifest matches its snapshot. `check-itinerary` always checks the
selected root's active bundle; it has no flag to choose a snapshot ID directly.
An immutable folder has its manifest at its own root, while an active context
expects `data/manifest.json`, so `--root` cannot simply point at that folder.

## Recover after a refresh or merge

Generated views, their manifest, and example paths must describe the same
snapshot. A merge that combines generations can fail digest checks even when the
individual files parse. Preserve immutable history and decide which catalog
contains the reviewed facts before regenerating.

The following local commands rewrite generated files. Use a working copy and
review the diff. If `data/catalog.json` is the intended valid catalog:

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

If the active catalog is damaged, use the immutable catalog path from the last
intact manifest with `export --catalog PATH`. Current generator code can produce
a new snapshot identity from the same facts, so regenerate the paths afterward.
Offline export applies no annotation-file edits and retrieves no newer facts.

For a source update, `refresh` retrieves official pages and applies
`community/annotations/sessions.json`; then regenerate and check the paths as
above. A removal stops refresh unless reviewed and explicitly accepted with
`--accept-removals`. Review orphaned annotations too. Read the
[maintenance guide](maintenance.md#refresh-official-data) before live collection.

## Implementation and evidence

- [CLI dispatch](../tools/conference-data/src/main.rs) owns flags, path handling,
  output, and exit behavior.
- [Domain validation](../tools/conference-data/src/domain.rs) checks constructed
  values, references, intervals, and annotation evidence.
- [Bundle verification](../tools/conference-data/src/export.rs) checks active and
  immutable file digests and manifest identity. Hash consistency authenticates
  no external conference announcement.
- [Integration tests](../tools/conference-data/tests/end_to_end.rs) exercise
  distinct profiles, stale identity, overlaps, unavailable windows, corrupt views,
  recovery, stale annotations, and rejected publication preserving the manifest.

Use the [validation receipt](validation.md) for executed checks and the
[maintainer task map](maintenance.md#find-the-owning-file) for the next owning source.
