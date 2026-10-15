# Atomic Design Patterns in Rust: From Orderings to Code

[Official EuroRust session](https://eurorust.eu/talks/atomic-design-patterns-in-rust-from-orderings-to-code/) · [All session resources](README.md)

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

Speakers and their work: [Martin Ombura Jr.](../speakers/martin-ombura-jr.md).

## Related talks and workshops

### [Rust Atomics in the Wild: Memory Ordering Design Patterns from Popular Crates](https://rustconf2026.sched.com/event/2KHth)

Talk · Martin Ombura Jr. · access unverified

**Same topic.** Follow the same speaker's earlier session on atomic patterns found in Rust crates.

[RustConf 2026](https://rustconf2026.sched.com/) · 2026-09-10 · Delivery unverified.

Official timetable and speaker roster checked. Detail-page access was inconsistent; delivery and recording remain unverified.

Listing checked: 2026-10-15. Evidence: [rustconf.com](https://rustconf.com/speakers/), [rustconf2026.sched.com](https://rustconf2026.sched.com/).

### [Talking to ghosts](https://rustlab.it/talks/talking-to-ghosts)

Talk · Arya Dradjica · registration

**Suggested next step.** Extend atomic coordination into the harder problem of reclaiming memory while other threads may still read it.

[RustLab 2026](https://rustlab.it/talks/talking-to-ghosts) · 2026-11-03 · Announced.

Future programme entry; concurrent memory reclamation is a follow-up topic, not a repeat of the EuroRust talk.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/talking-to-ghosts).

## Read, watch, and try

### [Rust Atomics and Locks](https://mara.nl/atomics/)

Book · Mara Bos · open access

**Preparation.** Work through memory ordering and the construction of locks and other synchronization primitives.

The author provides the full book online; print and commercial ebook editions are separate.

Source page checked: 2026-10-15. Evidence: [mara.nl](https://mara.nl/atomics/).

### [Rust atomic types and memory ordering](https://doc.rust-lang.org/std/sync/atomic/index.html)

Documentation · Rust project contributors · open access

**Preparation.** Check the standard library's ordering guarantees and restrictions before judging an atomic pattern.

Source page checked: 2026-10-15. Evidence: [doc.rust-lang.org](https://doc.rust-lang.org/std/sync/atomic/index.html).

### [Loom](https://github.com/tokio-rs/loom)

Project · Loom contributors · open access

**Suggested next step.** Explore small concurrent tests across possible thread and memory-model interleavings.

Model checking applies within the tested model and bounds; it does not replace a correctness argument.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/tokio-rs/loom).

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-atomic-design-patterns-in-rust-from-orderings-to-code.json)
