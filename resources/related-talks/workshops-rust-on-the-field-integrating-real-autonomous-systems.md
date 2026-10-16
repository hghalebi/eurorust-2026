# Rust on the Field: Integrating Real Autonomous Systems

[Official EuroRust session](https://eurorust.eu/workshops/rust-on-the-field-integrating-real-autonomous-systems/) · [All session resources](README.md)

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

No individual speaker is named in this catalog.

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Integrate vehicle and field-data protocols into a Rust orchestration platform while handling unreliable transport and heterogeneous system models.
- **Prerequisites:** Comfort with async Rust and Tokio; previous robotics or protocol experience is not required.
- **Setup:** Bring a stable-Rust development laptop; hardware is provided and setup instructions are sent before the workshop.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [MAVLink Developer Guide](https://mavlink.io/en/). Explains the message, heartbeat and routing concepts behind the workshop’s MAVLink connector.
- **Bring a question:** Which assumptions need evidence from the real hardware, and which can you test in simulation?
- **Other sessions starting at this time:** [Beyond p99: Observing Async Rust](workshops-beyond-p99-observing-async-rust.md) · [Building a virtual machine monitor in Rust](workshops-building-a-virtual-machine-monitor-in-rust.md) · [C→Rust, Beyond the Basics](workshops-c-to-rust.md) · [Containers Are Dead: Long Live WebAssembly](workshops-containers-are-dead.md) · [Hands-On Embedded Rust](workshops-hands-on-embedded-rust.md) · [Programming a Line Follower Robot](workshops-programming-a-line-follower-robot.md) · [Rust in Action - Applications of Computer Vision and Control](workshops-rust-in-action.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

No specific related conference appearance was verified in this pass. The sources below provide topic background.

## Read, watch, and try

### [MAVLink Developer Guide](https://mavlink.io/en/)

Documentation · MAVLink contributors · open access

**Preparation.** Explains the message, heartbeat and routing concepts behind the workshop’s MAVLink connector.

Source page checked: 2026-10-15. Evidence: [mavlink.io](https://mavlink.io/en/).

### [rust-mavlink](https://github.com/mavlink/rust-mavlink)

Project · rust-mavlink contributors · open access

**Same topic.** Provides Rust bindings and examples for exchanging MAVLink messages.

A learning reference; no vehicle connection or hardware command was executed.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/mavlink/rust-mavlink).

### [Timeouts, retries, and backoff with jitter](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/)

Article · Amazon Builders’ Library · open access

**Preparation.** Explains timeout budgets, retry amplification and jitter before handling unreliable transport.

Source page checked: 2026-10-15. Evidence: [aws.amazon.com](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/).

### [Graceful Shutdown](https://tokio.rs/tokio/topics/shutdown)

Documentation · Tokio contributors · open access

**Suggested next step.** Shows how to cancel connector tasks and wait for cleanup when a service stops.

Source page checked: 2026-10-15. Evidence: [tokio.rs](https://tokio.rs/tokio/topics/shutdown).

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/workshops-rust-on-the-field-integrating-real-autonomous-systems.json)
