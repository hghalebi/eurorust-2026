# Containers Are Dead: Long Live WebAssembly

[Official EuroRust session](https://eurorust.eu/workshops/containers-are-dead/) · [All session resources](README.md)

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

Speakers and their work: [Jonas Kruckenberg](../speakers/jonas-kruckenberg.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Build a Rust WebAssembly service and compare its deployment and isolation tradeoffs with container-based approaches.
- **Prerequisites:** Rust familiarity and introductory awareness of WebAssembly.
- **Setup:** Confirm the organizer’s preparation instructions; specific equipment requirements are not captured here.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [The WebAssembly Component Model](https://component-model.bytecodealliance.org/). Introduces components, interfaces and WASI before comparing service deployment models.
- **Bring a question:** What confirms the intended configuration became the actual running state?
- **Other sessions starting at this time:** [Beyond p99: Observing Async Rust](workshops-beyond-p99-observing-async-rust.md) · [Building a virtual machine monitor in Rust](workshops-building-a-virtual-machine-monitor-in-rust.md) · [C→Rust, Beyond the Basics](workshops-c-to-rust.md) · [Hands-On Embedded Rust](workshops-hands-on-embedded-rust.md) · [Programming a Line Follower Robot](workshops-programming-a-line-follower-robot.md) · [Rust in Action - Applications of Computer Vision and Control](workshops-rust-in-action.md) · [Rust on the Field: Integrating Real Autonomous Systems](workshops-rust-on-the-field-integrating-real-autonomous-systems.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

### [Containers Are Dead: Long Live WebAssembly](https://eurorust.eu/2025/workshops/long-live-webassembly/)

Workshop · Jonas Kruckenberg · public source page

**Repeat appearance.** Lets readers compare the previous syllabus for Rust HTTP services, WASM deployment and diagnostics.

[EuroRust 2025](https://eurorust.eu/2025/workshops/long-live-webassembly/) · 2025-10-08 · Delivery unverified.

Earlier programme with the same title and mentor; no recording or identical 2026 syllabus is claimed.

Listing checked: 2026-10-15. Evidence: [eurorust.eu](https://eurorust.eu/2025/workshops/long-live-webassembly/).

## Read, watch, and try

### [The WebAssembly Component Model](https://component-model.bytecodealliance.org/)

Book · Bytecode Alliance contributors · open access

**Preparation.** Introduces components, interfaces and WASI before comparing service deployment models.

Listing checked: 2026-10-15. Evidence: [component-model.bytecodealliance.org](https://component-model.bytecodealliance.org/).

### [Wasmtime security model](https://docs.wasmtime.dev/security.html)

Documentation · Wasmtime contributors · open access

**Same topic.** Explains sandboxing and capability-based filesystem access so isolation claims can be assessed.

Source page checked: 2026-10-15. Evidence: [docs.wasmtime.dev](https://docs.wasmtime.dev/security.html).

### [Bringing the Web up to Speed with WebAssembly](https://css.csail.mit.edu/6.5660/2024/readings/wasm.pdf)

Paper · Andreas Haas, Andreas Rossberg, Derek L. Schuff, Ben L. Titzer, Michael Holman, Dan Gohman, Luke Wagner, Alon Zakai, JF Bastien · open access

**Preparation.** Explains WebAssembly’s portable execution model, validation and original design constraints.

2017 research paper; its original runtime results do not describe current WASI deployments.

Listing checked: 2026-10-15. Evidence: [css.csail.mit.edu](https://css.csail.mit.edu/6.5660/2024/readings/wasm.pdf), [webassembly.org](https://webassembly.org/community/events/).

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/workshops-containers-are-dead.json)
