# EuroRust 2026, with a plan

**Choose your talks. Bring better questions. Build something useful.**

I'm planning my EuroRust around one question: what can Rust teach me about
building better autonomous agents in production? My route connects type systems,
reliable workers, infrastructure, and robotics. This is where I turn those
interests into session choices, preparation notes, and small experiments.

Use my path as a starting point, make a plan of your own, or give the conference
data to your agent. Everything you need to get started is readable on GitHub.

| Your next step | Open this |
| --- | --- |
| **Follow my path** | [What I'm looking for](my-conference/README.md) → [my proposed itinerary](my-conference/itinerary.md) |
| **Plan your own EuroRust** | [Browse the schedule](exports/schedule.md) → [choose your sessions](docs/planning-guide.md) → [save your plan](templates/itinerary.md) |
| **Plan with your agent** | [Give it the conference pack](exports/conference-pack.md) → [copy the planning prompt](prompts/choose-my-path.md) → [check its itinerary](docs/agent-guide.md) |

EuroRust 2026 runs **October 14–17 in Barcelona**, with online participation
announced on the [official event site](https://eurorust.eu/). Schedule times here
use **Europe/Madrid**. Check the linked official pages for changes, registration,
prerequisites, and access to each session.

## Find a path worth bringing home

Start with a question you want the conference to help answer. Then compare
sessions, choose between conflicting talks, and prepare one question for each.
The [planning guide](docs/planning-guide.md) walks you through it without a build
step. [Example paths](community/paths/README.md) show how different goals lead to
different choices.

My itinerary is a proposal. Availability, bookings, attendance mode, and readiness
still need confirmation. [Preparation notes](notes/README.md),
[experiments](experiments/README.md), and the [journal](journal/README.md) keep
planned learning separate from what has actually been observed.

## Bring the data to your own harness

Use the [Markdown pack](exports/conference-pack.md) for a quick start, or take
[JSON](data/catalog.json), [JSONL](exports/sessions.jsonl), and
[schemas](schema/catalog.schema.json) into your own tools. The
[agent guide](docs/agent-guide.md) explains how to build a plan around a
participant's goals and constraints. No provider account or custom MCP server is
required by this repository.

Every published bundle has a [manifest](data/manifest.json) with its snapshot
identity, file hashes, coverage, and validation time. Official facts retain their
sources; community summaries carry attribution and review status. Missing
information stays explicit. Read the [data guide](docs/data-guide.md) when you
need the exact contract.

[Download a complete context pack and macOS arm64 tools](https://github.com/hghalebi/eurorust-2026/releases).

## Make the next person's plan better

A corrected time, a clearer prerequisite, or a useful question can help another
attendee decide where to go. [Contribute one improvement](CONTRIBUTING.md),
[pick a starter issue](https://github.com/hghalebi/eurorust-2026/issues), or share a
path for someone with different goals. Small, well-sourced contributions welcome.

If this helps you plan, share the guide or path that helped—and tell us what
would make the next choice easier.

## Build and check the tools

Use a current stable Rust toolchain. From the repository root:

```sh
cargo build --release --locked
mkdir -p bin
install -m 755 target/release/conference-data bin/conference-data
bin/conference-data validate
bin/conference-data check-docs
```

The compiled tool refreshes official facts, exports checked snapshots, and
proposes or checks itineraries. See the [task index](docs/README.md) for the owning
source and relevant check, and the [validation receipt](docs/validation.md) for
recorded evidence.

This is an independent attendee project. Code uses [MIT](LICENSE); original prose
and annotations use [CC BY 4.0](LICENSES/README.md). Third-party material retains
its own terms. See the [source and reuse policy](docs/source-and-reuse-policy.md).
