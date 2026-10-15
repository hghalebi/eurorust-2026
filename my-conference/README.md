# My path: Rust for production agents

I want to build reliable autonomous agents. At EuroRust, I'm exploring types,
workers, infrastructure, and robotics.

Find me on [X](https://x.com/Hamzeml), [GitHub](https://github.com/hghalebi),
and [LinkedIn](https://www.linkedin.com/in/hamze/).

| Interest | What I want to learn |
| --- | --- |
| Types | Reject invalid tool inputs and state transitions |
| Reliability | Handle retries, cancellation, shutdown, and recovery |
| Infrastructure | Check configuration and integration assumptions |
| Robotics | Work with unreliable transport and changing physical state |

[My proposed itinerary](itinerary.md) is a draft selected by topic matches. It
lists times, alternatives, sources, and known setup requirements. Availability
and attendance mode are unknown; no workshop booking is recorded.

## The choices I want to examine

Start with [four questions and first sources](../resources/learning-paths.md).
For my production-agent goal, background workers raise the most immediate
question: what happens to accepted work after a crash? Types and streaming state
machines help examine the contracts around that work. Infrastructure and robotics
bring in facts that code must check outside the process.

One slot needs a deliberate choice. The captured schedule puts
[infrastructure compilation](../resources/related-talks/talks-your-infrastructure-should-compile-too.md)
and [the OpenTelemetry engine](../resources/related-talks/talks-building-a-high-performance-thread-per-core-zero-copy-opentelemetry-engine-in-rust.md)
at **15:15 on October 16** (Europe/Madrid). For my configuration goal, I would
favor infrastructure; for telemetry pipeline design, the engine talk is the
closer fit. The generated draft selects the engine by topic score, so this is a
choice to review. Confirm the [official schedule](https://eurorust.eu/schedule/).
These priorities are my interpretation, not a record of attendance.

## Prepare and test

Use the [preparation questions](../notes/README.md) and
[two Rust experiments](../experiments/README.md) on checked inputs and retry budgets.
I may extend them at October 17 impl Day if participation and setup are confirmed.

The [journal](../journal/README.md) records preparation and later observations.
The [retrospective](../retrospective.md) awaits lessons after participation.

## Make your own path

Adapt the reasons that fit your goals. Use the [planning guide](../docs/planning-guide.md)
and [template](../templates/itinerary.md), then confirm your time, bookings, and
the [official schedule](https://eurorust.eu/schedule/).

The [profile](profile.json) and [itinerary JSON](itinerary.json) support structured
reuse. Follow the [agent guide](../docs/agent-guide.md) or the
[optional local walkthrough](../docs/quickstart.md#reuse-the-owner-example).
