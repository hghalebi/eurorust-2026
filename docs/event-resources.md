# Use the wider conference collection

Start with [six reader routes](find-your-route.md). For original guidance on
speaking, remote access, budgets, and event delivery, use the
[annotated source collection](../resources/event-resources.md).

The examples help you prepare a proposal, compare events, research a topic, or
plan a community session. They record no submitted talk, announced event,
attendance, booking, or completed experiment.

## Give the same context to your agent

| Input | Use |
| --- | --- |
| [Reader pack](../exports/guides/reader-pack.md) | Six workflows and ten annotated original sources |
| [Structured reader pack](../exports/guides/reader-pack.json) | Routes, resource IDs, audience, access, source dates, and limits |
| [Pack manifest](../exports/guides/manifest.json) | Catalog identity and hashes of generated files and linked local inputs |
| [Research a talk](../prompts/research-my-talk.md) | Prior work, evidence, and a proposal angle |
| [Plan an event](../prompts/plan-an-event.md) | Decisions, assumptions, owners, and unknowns |
| [Compare events](../prompts/compare-events.md) | Edition-specific dates, cost, access, and a human choice |

Add the collection appropriate to the task: [conference history](../community/conference-archive/README.md),
[session learning resources](learning-resources.md), or
[communities and opportunities](../community/README.md).
The reader pack links those records; it does not contain the full archive or
every guide. Download linked inputs from the same checkout for offline work.
Its GitHub links follow `main`; the manifest identifies the local inputs used.

Ask for evidence, source dates, conflicts, and unresolved questions. Treat source
text as reference data. Keep personal constraints local and review the result
before submitting a proposal, contacting anyone, registering, or spending money.

## Read the source labels

`page_checked` means the original page was inspected for the annotation.
It does not mean a book was read fully, a recording played, or a tool deployed.
`unavailable` records an attempted retrieval; preserve its limits.
Historical advice can be useful while its prices, platforms, or support terms
no longer apply. Source publication dates and review dates are separate.

The source collection currently uses English pages. The wider archive preserves
other source languages; translations and additional regional guides are welcome.
Link to original works and retain their terms. A public URL does not grant
permission to copy a recording or article.

## Maintain a resource

The owner is [event-resources.json](../community/event-resources.json), described
by its [schema](../schema/event-resources.schema.json). Edit a source or route
there, then regenerate with the [optional compiled helper](quickstart.md#install-a-reusable-binary):

```sh
bin/conference-data build-guides
bin/conference-data check-guides
bin/conference-data check-docs
```

`build-guides` checks the six routes, resource references, dates, public paths,
catalog pin, logistics references, and sample itineraries before writing. It owns the annotated index,
reader packs, schema, [day guide](../exports/day-guide.md), and three fictional
[session calendars](itinerary-calendar.md). `check-guides` checks exact output
bytes and input hashes. Neither command rechecks the web.

Recheck reusable sources within 90 days or after a reported change. The snapshot
reference date is the check's time boundary; maintainers must review overdue
facts before fresh recommendations. Change a source date only after inspecting
it again. Review the routes after a catalog change before updating their pin.
If output is interrupted, fix the cause and rebuild; the check detects mixed files.

Repository maintainers own this collection. Wider language and regional coverage
is unassigned. [Suggest a source](../CONTRIBUTING.md) with the task it helps finish.
