# Optimizing rustc's Next Trait Solver

[Official EuroRust session](https://eurorust.eu/talks/optimizing-the-new-rust-trait-solver/) · [All session resources](README.md)

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass. Additions checked 2026-10-17; earlier entries retain their individual source dates.

Speakers and their work: [Jana Dönszelmann](../speakers/jana-donszelmann.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Study algorithmic improvements and sound caching decisions in the next trait solver, with performance claims understood in their benchmark context.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Next-gen trait solving](https://rustc-dev-guide.rust-lang.org/solve/trait-solving.html). Introduces goals, responses and the structure of rustc’s next trait solver.
- **Bring a question:** Which rule can the type system enforce, and which still needs a runtime check?
- **Other sessions starting at this time:** [Shipping Temporal: How the Rust Ecosystem Built Chrome’s Newest Web API](talks-shipping-temporal-how-the-rust-ecosystem-built-chrome-s-newest-web-api.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

### [-Znext-solver: what, why, and when](https://eurorust.eu/2025/talks/znext-solver/)

Talk · lcnr · public source page

**Project evolution.** Explains the correctness and performance problems that motivated replacing the previous solver.

[EuroRust 2025](https://eurorust.eu/2025/talks/znext-solver/) · 2025-10-09 · Delivery unverified.

Official session description checked; delivery and recording are unverified.

Listing checked: 2026-10-15. Evidence: [eurorust.eu](https://eurorust.eu/2025/talks/znext-solver/).

## Read, watch, and try

### [Next-gen trait solving](https://rustc-dev-guide.rust-lang.org/solve/trait-solving.html)

Documentation · Rust compiler contributors · open access

**Preparation.** Introduces goals, responses and the structure of rustc’s next trait solver.

Source page checked: 2026-10-15. Evidence: [rustc-dev-guide.rust-lang.org](https://rustc-dev-guide.rust-lang.org/solve/trait-solving.html).

### [The Chalk Book](https://rust-lang.github.io/chalk/book/)

Book · Chalk contributors · open access

**Preparation.** Explains logical goals, unification and canonical queries behind Rust trait reasoning.

Chalk is useful background; it is not the implementation of rustc’s next trait solver.

Listing checked: 2026-10-15. Evidence: [rust-lang.github.io](https://rust-lang.github.io/chalk/book/).

### [Rust compiler performance dashboard](https://perf.rust-lang.org/)

Project · Rust Project rustc-perf contributors · open access

**Suggested next step.** Explore the compiler performance dashboard when asking how a compiler optimization is measured across workloads.

Dashboard shell, help, and project description checked; no interactive query or independent benchmark run.

Source page checked: 2026-10-17. Evidence: [perf.rust-lang.org](https://perf.rust-lang.org/).

Source review: **2026-10-17**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-optimizing-the-new-rust-trait-solver.json)
