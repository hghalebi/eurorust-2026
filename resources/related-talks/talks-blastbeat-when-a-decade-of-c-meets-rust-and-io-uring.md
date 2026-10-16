# Blastbeat: When a Decade of C++ Meets Rust and io\_uring

[Official EuroRust session](https://eurorust.eu/talks/blastbeat-when-a-decade-of-c-meets-rust-and-io-uring/) · [All session resources](README.md)

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below. Additions checked 2026-10-17; earlier entries retain their individual source dates.

Speakers and their work: [Daniel Rodriguez](../speakers/daniel-rodriguez.md), [Giacomo Bagnoli](../speakers/giacomo-bagnoli.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Review a network-observability rewrite using batched I/O, with kernel compatibility and fleet rollout as important parts of the case study.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Announcing tokio-uring: io-uring support for Tokio](https://tokio.rs/blog/2021-07-tokio-uring). Understand why completion-based I/O owns buffers differently from readiness-based async I/O.
- **Bring a question:** What confirms the intended configuration became the actual running state?
- **Other sessions starting at this time:** [Designing State Machines for Streaming Proxies in Rust](talks-designing-state-machines-for-streaming-proxies-in-rust.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

### [Lessons learned from monitoring a large production network](https://devopsdays.org/events/2020-madrid/program/giacomo-bagnoli/)

Talk · Giacomo Bagnoli · public source page

**Preparation.** Compare the speaker's earlier network-health monitoring problems with the later Blastbeat rewrite.

[devopsdays Madrid 2020](https://devopsdays.org/events/2020-madrid/program/giacomo-bagnoli/) · Delivery unverified.

The archived programme describes monitoring practices, not the Rust implementation; recording availability was not verified.

Listing checked: 2026-10-15. Evidence: [devopsdays.org](https://devopsdays.org/events/2020-madrid/program/giacomo-bagnoli/).

### [Blastbeat: Replacing 10 Years of C++ with Rust and io\_uring](https://rustlab.it/talks/blastbeat-replacing-10-years-of-c-with-rust-and-iouring)

Talk · Giacomo Bagnoli · registration

**Repeat appearance.** Follow the announced RustLab appearance on the same network-observability rewrite.

[RustLab 2026](https://rustlab.it/talks/blastbeat-replacing-10-years-of-c-with-rust-and-iouring) · 2026-11-03 · Announced.

The same project and one EuroRust speaker; presentation details may differ and no future recording is promised.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/blastbeat-replacing-10-years-of-c-with-rust-and-iouring).

## Read, watch, and try

### [Announcing tokio-uring: io-uring support for Tokio](https://tokio.rs/blog/2021-07-tokio-uring)

Article · Tokio contributors · open access

**Preparation.** Understand why completion-based I/O owns buffers differently from readiness-based async I/O.

Publication date: 2021-07-19.

Historical design introduction; its initial release API is not current setup guidance.

Source page checked: 2026-10-15. Evidence: [tokio.rs](https://tokio.rs/blog/2021-07-tokio-uring).

### [Canarying Releases](https://sre.google/workbook/canarying-releases/)

Book · Google SRE contributors · open access

**Suggested next step.** Plan rollout signals and failure checks for replacing an agent across a server fleet.

Source page checked: 2026-10-15. Evidence: [sre.google](https://sre.google/workbook/canarying-releases/).

### [How We Moved Discord Voice to the Edge](https://discord.com/blog/how-we-moved-discord-voice-to-the-edge)

Article · David Chen, Discord · open access

**Suggested next step.** Compare another network-service rollout, including deployment reversals and investigations of async and network behavior.

Publication date: 2026-06-09.

2026 company report; distinct from Discord's 2020 Read States migration.

Source page checked: 2026-10-17. Evidence: [discord.com](https://discord.com/blog/how-we-moved-discord-voice-to-the-edge).

Source review: **2026-10-17**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-blastbeat-when-a-decade-of-c-meets-rust-and-io-uring.json)
