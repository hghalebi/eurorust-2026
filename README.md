# EuroRust 2026: a conference guide for curious Rust engineers

Choose sessions around what you want to learn, and arrive with questions worth
asking.

I'm using EuroRust to explore how Rust can help me build better autonomous
agents in production. This independent attendee collection shares my proposed
path, preparation notes, speaker reading profiles, and reusable conference data
so you can build a path of your own.

Find me on [X](https://x.com/Hamzeml), [GitHub](https://github.com/hghalebi),
and [LinkedIn](https://www.linkedin.com/in/hamze/).

[EuroRust takes place October 14–17, 2026, in Barcelona](https://eurorust.eu/).
Times in this collection use **Europe/Madrid**.

## Choose your starting point

| You want to… | Start here |
| --- | --- |
| Follow my path | Read [my learning goals](my-conference/README.md) and [proposed itinerary](my-conference/itinerary.md), with choices, reasons, and alternatives |
| Plan your own conference | [Compare the sessions](exports/schedule.md), then use the [planning guide](docs/planning-guide.md) to choose around your goals |
| Plan with your agent | Follow the [agent guide](docs/agent-guide.md) to supply conference context and review its suggestions |

For a first step, copy the [itinerary template](templates/itinerary.md) into your
notes. Pick one session, explain why it matters to you, and keep an alternative.
The [backend, embedded, and Rust foundations examples](community/paths/README.md)
offer other starting points. These reading and planning routes work directly in
your browser.

## Follow the questions behind my path

My interests run from Rust's type system to reliable workers, infrastructure
configuration, and robotics. The preparation notes connect those interests to
published session descriptions:

- **Types:** [Which tool contracts can the compiler check?](notes/talks/compile-time-contracts.md)
- **Workers:** [What happens around the worker loop?](notes/talks/background-workers.md)
- **Streaming:** [What survives when a stream stops halfway?](notes/talks/streaming-state-machines.md)
- **Infrastructure:** [Which integration assumptions can compilation catch?](notes/talks/compiled-infrastructure.md)
- **Robotics:** [How should typed software handle unreliable transport?](notes/workshops/robotics-integration.md)

Each note offers questions to investigate; the connections to agent engineering
are my interpretation. Follow them into [further reading](resources/README.md)
or [two small Rust experiments on checked inputs and retry budgets](experiments/README.md).

My itinerary is provisional: availability, bookings, attendance mode, and
readiness remain unconfirmed. The [journal](journal/README.md) records preparation
so far; the [retrospective](retrospective.md) awaits observations after participation.

## Meet the work behind the talks

The [speaker directory](resources/speakers/README.md) connects conference sessions
to public projects, blogs, books, earlier talks, research, and professional
accounts. Browse by name or start with a technical interest:

- [Njuguna Mureithi and Apalis](resources/speakers/njuguna-mureithi.md) for background jobs.
- [lcnr's trait-system writing](resources/speakers/lcnr.md) for the compiler questions behind an API.
- [Philipp Schuster's virtualization and Nix work](resources/speakers/philipp-schuster.md) for infrastructure.
- [Timo Bachmann's robotics research](resources/speakers/timo-bachmann.md) for task planning and skill adaptation.

These independently researched profiles cover the recorded speaker roster and
two speakers named on the official meetup page. Each records sources, a research
date, and gaps; the drafts await community and speaker review. Open one earlier
work and bring a question it raises to the session.

## Plan with your agent

Provide the [conference pack](exports/conference-pack.md),
[dataset manifest](data/manifest.json), and [planning prompt](prompts/choose-my-path.md)
from the same checkout or context bundle. Add your goals, available time, and
confirmed workshop bookings in your own workspace.

Ask for choices with reasons, alternatives, preparation needs, and official
links. Review the result against your actual availability, prerequisites, and
session access before deciding. For structured input, use the
[catalog JSON](data/catalog.json) or [session JSONL](exports/sessions.jsonl).
The [data guide](docs/data-guide.md) explains formats, schemas, and recorded versions.
Add the [speaker profiles JSON](community/speaker-profiles.json) when you want
a reading list connected to the speakers in your plan.

## Know the sources behind your plan

The exported data records official URLs, retrieval dates, and a snapshot ID that
identifies the recorded dataset version. Community summaries are attributed,
Codex-assisted drafts awaiting community review. Check each record's source date
and review status; a recent validation does not mean the source was recently
retrieved.

Use the [official schedule](https://eurorust.eu/schedule/) and linked session pages
to confirm current times, registration, prerequisites, and access. The
[source and reuse policy](docs/source-and-reuse-policy.md) explains how facts,
interpretation, and original contributions are kept distinct.

## Share what helps the next attendee

A corrected detail, a useful preparation tip, or a different learning path can
help someone make a better choice. [Contribute a note, correction, or path](CONTRIBUTING.md);
a GitHub issue with a supporting source is enough to start. Use the
[session note template](templates/session.md) to distinguish preparation from
what you observed. When sharing the collection, link the specific guide or note
that helped you.

The [guide index](docs/README.md) also leads to optional itinerary tools and
dataset maintenance instructions.

Original prose and annotations use [CC BY 4.0](LICENSES/README.md); supporting
code uses [MIT](LICENSE). Third-party material retains its own terms.
