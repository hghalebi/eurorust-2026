# Find the right file and the check that matters

Choose your task below. Read the linked guide's relevant section, then the owning
source or data and its check. Expand the search when that evidence leaves a
question open.

| I want to… | Start here | Owning source or data | Check before claiming success |
| --- | --- | --- | --- |
| Follow the owner's path | [Learning goals](../my-conference/README.md) | [Proposed itinerary](../my-conference/itinerary.md) | Official session links, source dates, and unresolved access |
| Choose sessions in a browser | [Planning guide](planning-guide.md) | [Readable schedule](../exports/schedule.md) | Availability, prerequisites, and conflicts |
| Plan with a participant's agent | [Agent guide](agent-guide.md) | [Manifest](../data/manifest.json), [conference pack](../exports/conference-pack.md) | Same-snapshot `check-itinerary` |
| Refresh official facts | [Refresh guide](data-guide.md#source-discovery-and-refresh) | [Collector](../tools/conference-data/src/collect.rs), [parser fixture](../tools/conference-data/tests/fixtures/schedule.html) | Parser failures, then `validate` |
| Change a schema or domain check | [Dataset contract](data-guide.md#contract) | [Domain types](../tools/conference-data/src/domain.rs) | Constructors and compile-fail cases |
| Change export or recovery | [Publication protocol](data-guide.md#snapshots-and-recovery) | [Exporter](../tools/conference-data/src/export.rs) | Digest checks and failed-publication recovery |
| Correct or annotate a session | [Contribution routes](../CONTRIBUTING.md#where-to-contribute) | [Annotations](../community/annotations/sessions.json), official source | Source evidence, then refresh |
| Share a note or learning path | [Contribution guide](../CONTRIBUTING.md) | [Note template](../templates/session.md), [example paths](../community/paths/README.md) | Attribution, session references, and itinerary feasibility |
| Change a documentation route | [Documentation checks](../CONTRIBUTING.md#checks) | [Document inventory](document-map.json), [navigation checker](../tools/conference-data/src/navigation.rs) | `check-docs` and historical/private-route fixtures |
| Recover a command that cannot start | [Runtime troubleshooting](../CONTRIBUTING.md#runtime-troubleshooting) | Harness installation, running version, and process owner | A fresh successful read-only command |

## Current evidence pointers

Use the source that can establish the particular result you need.

| You need evidence of… | Open this | Read it with this limit |
| --- | --- | --- |
| Source retrieval and dataset coverage | [Manifest](../data/manifest.json) and per-record provenance | Retrieval time does not establish attendance, live access, or a booking |
| Snapshot and export integrity | The manifest's immutable path and file hashes | `validate` checks both active and immutable views |
| Recorded verification | [Validation receipt](validation.md) | Results apply to the stated time, revision, and environment |
| Remote CI | [GitHub Actions](https://github.com/hghalebi/eurorust-2026/actions) | Match the exact commit and completed jobs |
| A published release | [Releases](https://github.com/hghalebi/eurorust-2026/releases) | Match the tag, commit, build identity, and downloaded hashes |
| Conference observations | [Journal](../journal/README.md), [retrospective](../retrospective.md) | Preparation and actual participation carry different evidence |

The [document inventory](document-map.json) owns classification. When a guide is
superseded, update its current routes and label retained historical references.
