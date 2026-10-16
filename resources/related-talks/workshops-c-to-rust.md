# C→Rust, Beyond the Basics

[Official EuroRust session](https://eurorust.eu/workshops/c-to-rust/) · [All session resources](README.md)

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

Speakers and their work: [Magnus Markling](../speakers/magnus-markling.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Practice incremental C-to-Rust migration through interfaces, module replacement, and interoperability pitfalls.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Bring a laptop and follow the emailed software setup instructions.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Foreign Function Interface — The Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html). Explains calling conventions, callbacks and unsafe boundaries when Rust and C share data.
- **Bring a question:** What happens if this work is cancelled or the process stops halfway through?
- **Other sessions starting at this time:** [Beyond p99: Observing Async Rust](workshops-beyond-p99-observing-async-rust.md) · [Building a virtual machine monitor in Rust](workshops-building-a-virtual-machine-monitor-in-rust.md) · [Containers Are Dead: Long Live WebAssembly](workshops-containers-are-dead.md) · [Hands-On Embedded Rust](workshops-hands-on-embedded-rust.md) · [Programming a Line Follower Robot](workshops-programming-a-line-follower-robot.md) · [Rust in Action - Applications of Computer Vision and Control](workshops-rust-in-action.md) · [Rust on the Field: Integrating Real Autonomous Systems](workshops-rust-on-the-field-integrating-real-autonomous-systems.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

No specific related conference appearance was verified in this pass. The sources below provide topic background.

## Read, watch, and try

### [The C to Rust Migration Book](https://mainmatter.com/c-to-rust-migration-book/)

Book · Jonas Kruckenberg · open access

**Same topic.** Covers mixed-language APIs, ownership across FFI and incremental migration exercises.

Overview and contents checked; exercise solutions were not run.

Listing checked: 2026-10-15. Evidence: [mainmatter.com](https://mainmatter.com/c-to-rust-migration-book/).

### [Foreign Function Interface — The Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html)

Documentation · Rust Project contributors · open access

**Preparation.** Explains calling conventions, callbacks and unsafe boundaries when Rust and C share data.

Source page checked: 2026-10-15. Evidence: [doc.rust-lang.org](https://doc.rust-lang.org/nomicon/ffi.html).

### [C2Rust](https://github.com/immunant/c2rust)

Project · Immunant and C2Rust contributors · open access

**Alternative approach.** Shows automated C translation as a starting point for comparison with manual migration.

The project states that translated output remains unsafe and needs further migration work.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/immunant/c2rust).

### [Rust and ThreadX](https://ferrous-systems.com/blog/rust-and-threadx/)

Article · Jonathan Pallant · open access

**Suggested next step.** Provides an embedded example of combining Rust with an existing C real-time operating system.

Publication date: 2023-12-07.

Historical integration example; certification statements are tied to its publication date.

Source page checked: 2026-10-15. Evidence: [ferrous-systems.com](https://ferrous-systems.com/blog/rust-and-threadx/).

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/workshops-c-to-rust.json)
