# Beyond p99: Observing Async Rust

[Official EuroRust session](https://eurorust.eu/workshops/beyond-p99-observing-async-rust/) · [All session resources](README.md)

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

Speakers and their work: [Julián Montes de Oca](../speakers/julian-montes-de-oca.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Instrument async workloads to understand task progress, scheduling delays, slow requests, and the cost of production diagnostics.
- **Prerequisites:** Comfort with Rust, preferably including some async programming; the named tools need no prior experience.
- **Setup:** Bring a Rust-ready laptop and follow the emailed preparation instructions.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Announcing Tokio Console 0.1](https://tokio.rs/blog/2021-12-announcing-tokio-console). Explains task states and why aggregate logs hide async runtime behavior.
- **Bring a question:** How much overhead does the measurement add, and which delays does it reveal or miss?
- **Other sessions starting at this time:** [Building a virtual machine monitor in Rust](workshops-building-a-virtual-machine-monitor-in-rust.md) · [C→Rust, Beyond the Basics](workshops-c-to-rust.md) · [Containers Are Dead: Long Live WebAssembly](workshops-containers-are-dead.md) · [Hands-On Embedded Rust](workshops-hands-on-embedded-rust.md) · [Programming a Line Follower Robot](workshops-programming-a-line-follower-robot.md) · [Rust in Action - Applications of Computer Vision and Control](workshops-rust-in-action.md) · [Rust on the Field: Integrating Real Autonomous Systems](workshops-rust-on-the-field-integrating-real-autonomous-systems.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

No specific related conference appearance was verified in this pass. The sources below provide topic background.

## Read, watch, and try

### [Announcing Tokio Console 0.1](https://tokio.rs/blog/2021-12-announcing-tokio-console)

Article · Tokio team · open access

**Preparation.** Explains task states and why aggregate logs hide async runtime behavior.

Publication date: 2021-12-17.

Historical introduction; use current project documentation for setup.

Source page checked: 2026-10-15. Evidence: [tokio.rs](https://tokio.rs/blog/2021-12-announcing-tokio-console).

### [Getting started with Tracing](https://tokio.rs/tokio/topics/tracing)

Documentation · Tokio contributors · open access

**Preparation.** Introduces spans and events for following one request across async tasks.

Source page checked: 2026-10-15. Evidence: [tokio.rs](https://tokio.rs/tokio/topics/tracing).

### [dial9](https://github.com/dial9-rs/dial9)

Project · dial9 contributors · open access

**Same topic.** Provides the workshop speaker’s profiling project for inspecting task scheduling and CPU activity.

Project documentation checked; no profiling run was performed.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/dial9-rs/dial9).

### [Metrique](https://github.com/awslabs/metrique)

Project · Metrique contributors · open access

**Suggested next step.** Shows how structured records retain the context behind individual slow requests.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/awslabs/metrique).

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/workshops-beyond-p99-observing-async-rust.json)
