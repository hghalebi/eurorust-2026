# EuroRust 2026: my conference path, shared

I am using this repository to plan a useful EuroRust experience and keep the
result useful for other participants and their agents.

My example path explores reliable production tools and workers, Rust type
systems, infrastructure, and robotics. It is a proposed path, not a claim of
attendance, registration, or expertise. This is an independent attendee project.

| Priority | Start here |
| --- | --- |
| 1. My own conference path | [Overview](my-conference/README.md) and [shareable itinerary](my-conference/itinerary.md) |
| 2. Plan your EuroRust attendance | [Readable schedule](exports/schedule.md), [planning guide](docs/planning-guide.md), and [itinerary template](templates/itinerary.md) |
| 3. Give your agent conference context | [Agent guide](docs/agent-guide.md), [conference pack](exports/conference-pack.md), and [copyable prompt](prompts/choose-my-path.md) |

The [official event](https://eurorust.eu/) runs October 14–17 in Barcelona and
online. Times in this repository use **Europe/Madrid**, with explicit offsets.
Confirm current schedule, tickets, prerequisites, and individual remote access on
the linked official pages. Unknowns stay unknown.

The [manifest](data/manifest.json) identifies the checked snapshot, retrieval and
validation evidence, coverage counts, and immutable data location. Read
[how the data works](docs/data-guide.md) before building an integration. Source
pages are linked; their full copyrighted text and personal profiles are excluded
from published exports.

You can use all public planning files without installing anything. For an agent,
start with the Markdown pack, or use [JSON](data/catalog.json),
[JSONL](exports/sessions.jsonl), and the [schema](schema/catalog.schema.json).
Fictional [profiles and checked paths](community/paths/README.md) demonstrate
different goals. The baseline planner helps compare topics; it does not infer
ticket availability, readiness, or actual attendance.

## Build and check the tools

Use a current stable Rust toolchain. From the repository root:

```sh
cargo build --release --locked
mkdir -p bin
install -m 755 target/release/conference-data bin/conference-data
bin/conference-data validate
bin/conference-data check-docs
```

The installed executable supports `refresh`, `export`, `recommend`, and
`check-itinerary`; see the [data guide](docs/data-guide.md). Recurring jobs invoke
the compiled executable directly.

## Contribute and learn

Corrections, summaries, translations, notes, learning paths, and small Rust
experiments are welcome. Start with [CONTRIBUTING](CONTRIBUTING.md) and the
[task index](docs/README.md). See [preparation notes](notes/README.md),
[experiments](experiments/README.md), [resources](resources/README.md), and the
[journal](journal/README.md). Preparation is labeled separately from observations
made after attending. The [retrospective](retrospective.md) records its status.

Code uses the [MIT license](LICENSE). Original prose and annotations use
[CC BY 4.0](LICENSES/README.md). Third-party materials retain their own terms;
see the [source and reuse policy](docs/source-and-reuse-policy.md).
