# Task and evidence index

Choose a route, inspect its headings, then read its owning section and source.
Expand only when the evidence requires it.

| Task | Guide | Owning source/data | Check |
| --- | --- | --- | --- |
| Follow the owner's example | [path](../my-conference/README.md) | [itinerary](../my-conference/itinerary.md) | Official session links and source dates |
| Choose sessions in a browser | [planning](planning-guide.md) | [schedule](../exports/schedule.md) | Availability, prerequisites, conflicts |
| Supply a participant's agent | [agent guide](agent-guide.md) | [manifest](../data/manifest.json), [pack](../exports/conference-pack.md) | `check-itinerary` |
| Refresh official facts | [data guide](data-guide.md#source-discovery-and-refresh) | [collector](../tools/conference-data/src/collect.rs) and [fixture](../tools/conference-data/tests/fixtures/schedule.html) | Collector tests, `validate` |
| Change schema or domain checks | [contract](data-guide.md#contract) | [domain](../tools/conference-data/src/domain.rs) | Constructor and compile-fail tests |
| Change publication | [recovery](data-guide.md#snapshots-and-recovery) | [export](../tools/conference-data/src/export.rs) | Snapshot/digest checks |
| Correct or annotate a session | [contributing](../CONTRIBUTING.md#where-to-contribute) | [annotations](../community/annotations/sessions.json) | Source evidence and refresh |
| Contribute a note or path | [contributing](../CONTRIBUTING.md) | [templates](../templates/session.md), [paths](../community/paths/README.md) | Session references and itinerary check |
| Change documentation routes | [checks](../CONTRIBUTING.md#checks) | [inventory](document-map.json), [checker](../tools/conference-data/src/navigation.rs) | `check-docs` and negative fixtures |
| Command cannot start | [runtime route](../CONTRIBUTING.md#runtime-troubleshooting) | Harness installation/version/process | Fresh read-only execution probe |

## Current evidence pointers

| Boundary | Identity/time/result owner | Evidence and limits |
| --- | --- | --- |
| Official retrieval and checked dataset | [manifest](../data/manifest.json) and per-record provenance | Retrieval is not attendance, live access, or booking |
| Immutable snapshot and exports | Manifest snapshot path and file digests | `validate` checks active and immutable views |
| Local Rust and navigation checks | [validation receipt](validation.md) | Local checks are separate from remote CI |
| Remote CI | [GitHub Actions](https://github.com/hghalebi/eurorust-2026/actions) | Inspect the exact commit and completed jobs |
| Published release | [releases](https://github.com/hghalebi/eurorust-2026/releases) | Match tag/commit and download assets |
| Conference observations | [journal](../journal/README.md) and [retrospective](../retrospective.md) | Preparation remains separate from actual participation |

The document inventory owns classification. Historical or experimental material
must be visibly labeled and active routes updated when it is superseded.
