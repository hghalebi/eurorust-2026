# EuroRust 2026: a conference plan you can reuse

**My Rust learning path, a schedule you can browse, and a CLI that builds checked starting itineraries for attendees and their agents.**

I'm planning around production autonomous agents, Rust's type system,
infrastructure, and robotics. Start with [my proposed path](my-conference/itinerary.md),
then adapt the same conference context to your own goals.

1. **Follow my plan:** [learning goals and preparation](my-conference/README.md).
2. **Choose your sessions:** [browse the schedule](exports/schedule.md) and use
   the [browser planning guide](docs/planning-guide.md). No installation needed.
3. **Plan with your agent:** give it the [conference pack](exports/conference-pack.md)
   and [copyable prompt](prompts/choose-my-path.md). Use your own harness.

The event dates are **October 14–17, 2026**, in Barcelona. Session times use
**Europe/Madrid**. Check the [official schedule](https://eurorust.eu/schedule/)
for current times, tickets, prerequisites, and access to individual sessions.

## Quick start: generate a plan

**Prerequisites:** Git and a current stable Rust toolchain. Cloning needs network
access, and the first build may download dependencies. The planning commands read the local snapshot
and require no model provider, account, or API key.

```sh
git clone https://github.com/hghalebi/eurorust-2026.git
cd eurorust-2026
cargo build --release --locked --package conference-data
mkdir -p .cache/quickstart
./target/release/conference-data validate
./target/release/conference-data recommend \
  --profile community/paths/backend-profile.json \
  --output .cache/quickstart/itinerary.json \
  --markdown .cache/quickstart/itinerary.md
./target/release/conference-data check-itinerary \
  --itinerary .cache/quickstart/itinerary.json
```

Open `.cache/quickstart/itinerary.md` to see times, reasons, official links,
preparation, and alternatives. The final check prints:

```text
Schedule feasibility checked; no booking or attendance claim
```

Warnings identify decisions that still need confirmation. The fictional backend
profile covers October 15–16. [Make it your own](docs/quickstart.md#use-your-own-profile)
by editing a local profile's topics, availability, and confirmed registrations.
The CLI uses topic matching; an agent can interpret your broader goals.

See the [complete walkthrough](docs/quickstart.md) for verified example output,
platform evidence, downloads, and troubleshooting.

## What makes a plan traceable?

The [manifest](data/manifest.json) identifies the dataset snapshot, generator
fingerprint, coverage, and file hashes. Each session retains its official URL and
retrieval date. Original community summaries carry attribution, a checked source
hash, and a review status. Validation checks internal consistency; it does not
establish live booking, access, or prerequisite readiness.

Use [JSON](data/catalog.json), [JSONL](exports/sessions.jsonl), or
[Markdown](exports/conference-pack.md). The [data guide](docs/data-guide.md)
explains the schema, collection rules, immutable snapshots, and recovery.
The [CLI reference](docs/cli.md) explains exactly how selection and checks work.

## Turn a choice into something you can try

[Preparation notes](notes/README.md) connect session descriptions to questions
about tool contracts, workers, infrastructure, and robotics. Two
[small Rust experiments](experiments/README.md) exercise checked tool inputs and
owned retry budgets. They are local learning examples with explicit limits.

My itinerary is conditional: availability, bookings, attendance mode, and
readiness remain unconfirmed. The [journal](journal/README.md) and
[retrospective](retrospective.md) separate preparation from actual observations.

## Help the next attendee choose

[Report a source discrepancy](community/corrections/README.md),
[share a learning path](community/paths/README.md), or add an original session
note. [CONTRIBUTING](CONTRIBUTING.md) shows the owning files and relevant checks;
[starter issues](https://github.com/hghalebi/eurorust-2026/issues) offer focused ways in.
If you share the project, link the guide, example, or experiment that helped you.

## Build and check the tools

The quick start uses the compiled executable in `target/release/`. For reusable
`bin/` commands, follow [the installation steps](docs/quickstart.md#install-a-reusable-binary).
Use the [task index](docs/README.md) for source and check routes and the
[validation receipt](docs/validation.md) for dated evidence. Native downloads
are available in [releases](https://github.com/hghalebi/eurorust-2026/releases).

This is an independent attendee project. Code uses [MIT](LICENSE); original prose
and annotations use [CC BY 4.0](LICENSES/README.md). Third-party materials retain
their own terms. Read the [source and reuse policy](docs/source-and-reuse-policy.md).
