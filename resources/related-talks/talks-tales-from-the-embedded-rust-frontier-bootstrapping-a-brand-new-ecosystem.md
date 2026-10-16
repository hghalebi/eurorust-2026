# Tales from the Embedded Rust Frontier: Bootstrapping a brand new ecosystem

[Official EuroRust session](https://eurorust.eu/talks/tales-from-the-embedded-rust-frontier-bootstrapping-a-brand-new-ecosystem/) · [All session resources](README.md)

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass. Additions checked 2026-10-17; earlier entries retain their individual source dates.

Speakers and their work: [Jonathan Pallant](../speakers/jonathan-pallant.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Survey the crates and testing work needed to build an ecosystem for intermediate-sized Arm processors used in real-time and embedded systems.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Rust and ThreadX](https://ferrous-systems.com/blog/rust-and-threadx/). Explains why integrating Rust with an existing real-time operating system can be useful.
- **Bring a question:** Which assumptions need evidence from the real hardware, and which can you test in simulation?
- **Other sessions starting at this time:** [Building a Query System](talks-building-a-query-system.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

No specific related conference appearance was verified in this pass. The sources below provide topic background.

## Read, watch, and try

### [Rust on Cortex-R52](https://ferrous-systems.com/blog/rust-cortex-r52/)

Article · Ferrous Systems · open access

**Project evolution.** Documents the libraries and examples released for the intermediate Arm processors discussed in the talk.

Publication date: 2025-03-11.

Source page checked: 2026-10-15. Evidence: [ferrous-systems.com](https://ferrous-systems.com/blog/rust-cortex-r52/).

### [aarch32: Rust support for Arm Cortex-R and Cortex-A](https://github.com/rust-embedded/aarch32)

Project · Rust Embedded Devices Working Group · open access

**Same topic.** Provides low-level processor access and runtime support to inspect after the ecosystem overview.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/rust-embedded/aarch32), [ferrous-systems.com](https://ferrous-systems.com/blog/rust-cortex-r52/).

### [Rust and ThreadX](https://ferrous-systems.com/blog/rust-and-threadx/)

Article · Jonathan Pallant · open access

**Preparation.** Explains why integrating Rust with an existing real-time operating system can be useful.

Publication date: 2023-12-07.

Historical integration example; certification statements are tied to its publication date.

Source page checked: 2026-10-15. Evidence: [ferrous-systems.com](https://ferrous-systems.com/blog/rust-and-threadx/).

### [The Embedded Rust Book](https://docs.rust-embedded.org/book/)

Book · Rust Embedded Devices Working Group · open access

**Preparation.** Builds the no\_std and peripheral vocabulary needed to compare embedded ecosystems.

Introductory examples focus on microcontrollers rather than covering every Cortex-A or Cortex-R target.

Listing checked: 2026-10-15. Evidence: [docs.rust-embedded.org](https://docs.rust-embedded.org/book/).

### [Engineering Determination](https://rfd.shared.oxide.computer/rfd/0113)

Article · Oxide · open access

**Suggested next step.** Compare one company’s embedded-system choices and rejected approaches with the work of building a new Rust platform.

Organizational perspective, including its own judgments. Article sections reviewed; code and reported measurements were not reproduced.

Source page checked: 2026-10-17. Evidence: [rfd.shared.oxide.computer](https://rfd.shared.oxide.computer/rfd/0113).

Source review: **2026-10-17**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-tales-from-the-embedded-rust-frontier-bootstrapping-a-brand-new-ecosystem.json)
