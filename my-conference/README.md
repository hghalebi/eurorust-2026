# My path: Rust for better production agents

I want to build autonomous agents that do useful work in production. At EuroRust,
I'm looking for the engineering ideas underneath that goal: clearer contracts,
reliable workers, infrastructure I can reason about, and software that interacts
with the physical world.

| What I'm exploring | What I want to take away |
| --- | --- |
| Rust's type system | Make invalid tool inputs and illegal state transitions harder to express |
| Production reliability | Understand failure, retries, cancellation, shutdown, and recovery |
| Infrastructure in Rust | Check configuration and integration assumptions before delivery |
| Robotics | Connect typed software to unreliable transport and changing physical state |

[Open my proposed itinerary](itinerary.md) for exact slots, reasons, alternatives,
source dates, and preparation details. The [shareable profile](profile.json) and
[checked itinerary JSON](itinerary.json) make the same path reusable by tools.

This is a preparation plan. My availability and attendance mode remain unknown;
no workshop booking is recorded. The talks provide foundations to apply to agent
engineering. Any specific connection to my own work is my interpretation.

## Make the ideas concrete

The [preparation notes](../notes/README.md) collect questions to bring. Two
[small Rust experiments](../experiments/README.md) already explore checked tool
inputs and owned retry budgets. October 17 impl Day is a conditional opportunity
to extend that work, subject to confirmed participation and a prepared environment.

After participating, I'll connect observed lessons to tested follow-ups in the
[journal](../journal/README.md) and [retrospective](../retrospective.md).

## Reproduce or adapt the path

After [building the tool](../README.md#build-and-check-the-tools), run:

```sh
bin/conference-data recommend --profile my-conference/profile.json --output my-conference/itinerary.json --markdown my-conference/itinerary.md
bin/conference-data check-itinerary --itinerary my-conference/itinerary.json
```

Use the [official schedule](https://eurorust.eu/schedule/) to confirm times and
access. To make your own path, keep a profile in your own files and follow the
[planning guide](../docs/planning-guide.md). Your goals should decide your route.
