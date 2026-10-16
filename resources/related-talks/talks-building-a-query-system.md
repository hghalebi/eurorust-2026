# Building a Query System

[Official EuroRust session](https://eurorust.eu/talks/building-a-query-system/) · [All session resources](README.md)

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed.

Speakers and their work: [Arya Dradjica](../speakers/arya-dradjica.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Compare compiler query architectures and incremental-result caching through a Cargo dependency-resolution example.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Queries: demand-driven compilation](https://rustc-dev-guide.rust-lang.org/query.html). Explains memoized queries, providers and dependency tracking in rustc.
- **Bring a question:** Which assumption would you test before using this approach in your own project?
- **Other sessions starting at this time:** [Tales from the Embedded Rust Frontier: Bootstrapping a brand new ecosystem](talks-tales-from-the-embedded-rust-frontier-bootstrapping-a-brand-new-ecosystem.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

### [Talking to ghosts](https://rustlab.it/talks/talking-to-ghosts)

Talk · Arya Dradjica · registration

**Suggested next step.** Continues the speaker’s compiler work through concurrent data structures and memory reclamation.

[RustLab 2026](https://rustlab.it/talks/talking-to-ghosts) · 2026-11-03 · Announced.

An announced follow-up topic, not a repeat of the query-system talk.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/talking-to-ghosts).

## Read, watch, and try

### [Queries: demand-driven compilation](https://rustc-dev-guide.rust-lang.org/query.html)

Documentation · Rust compiler contributors · open access

**Preparation.** Explains memoized queries, providers and dependency tracking in rustc.

Source page checked: 2026-10-15. Evidence: [rustc-dev-guide.rust-lang.org](https://rustc-dev-guide.rust-lang.org/query.html).

### [The Salsa Book](https://salsa-rs.github.io/salsa/)

Book · Salsa contributors · open access

**Same topic.** Shows an incremental computation model and a small language tutorial for comparison.

Listing checked: 2026-10-15. Evidence: [salsa-rs.github.io](https://salsa-rs.github.io/salsa/).

### [Build systems à la carte](https://www.microsoft.com/en-us/research/publication/build-systems-la-carte/)

Paper · Andrey Mokhov, Neil Mitchell, Simon Peyton Jones · open access

**Preparation.** Provides a framework for reasoning about dependencies, cached results and rebuild decisions.

2018 ICFP paper with Haskell examples; concepts transfer to Rust query systems.

Listing checked: 2026-10-15. Evidence: [www.microsoft.com](https://www.microsoft.com/en-us/research/publication/build-systems-la-carte/).

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-building-a-query-system.json)
