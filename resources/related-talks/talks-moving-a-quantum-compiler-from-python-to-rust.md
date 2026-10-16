# Moving a quantum compiler from Python to Rust

[Official EuroRust session](https://eurorust.eu/talks/moving-a-quantum-compiler-from-python-to-rust/) · [All session resources](README.md)

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below. Additions checked 2026-10-17; earlier entries retain their individual source dates.

Speakers and their work: [Jake Lishman](../speakers/jake-lishman.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Follow an incremental compiler migration from research-oriented Python toward a Rust-centered implementation and changing hardware requirements.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Introduction to transpilation](https://quantum.cloud.ibm.com/docs/en/guides/transpile). Learn why quantum circuits need routing and transformation for the target hardware.
- **Bring a question:** Which assumption would you test before using this approach in your own project?
- **Other sessions starting at this time:** [Atomic Design Patterns in Rust: From Orderings to Code](talks-atomic-design-patterns-in-rust-from-orderings-to-code.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

No specific related conference appearance was verified in this pass. The sources below provide topic background.

## Read, watch, and try

### [LightSABRE: A Lightweight and Enhanced SABRE Algorithm](https://arxiv.org/abs/2409.08368)

Paper · Henry Zou, Matthew Treinish, Kevin Hartman, Alexander Ivrii, Jake Lishman · open access

**Same topic.** Study a concrete Qiskit routing algorithm developed mainly in Rust during the compiler migration.

Publication date: 2024-09-12.

Preprint abstract and metadata checked; performance figures belong to the paper's specific comparisons.

Source page checked: 2026-10-15. Evidence: [arxiv.org](https://arxiv.org/abs/2409.08368).

### [Release News: Qiskit SDK v2.0 is here!](https://www.ibm.com/quantum/blog/qiskit-2-0-release-summary)

Article · Qiskit Team · open access

**Project evolution.** Follow the Rust data-model changes and C interface introduced during Qiskit's migration.

Publication date: 2025-04-03.

A historical release report; consult current documentation for supported APIs.

Source page checked: 2026-10-15. Evidence: [www.ibm.com](https://www.ibm.com/quantum/blog/qiskit-2-0-release-summary).

### [PyO3 user guide](https://pyo3.rs/)

Documentation · PyO3 contributors · open access

**Suggested next step.** Understand the Python/Rust ownership boundary used when accelerating an existing Python library.

The root URL follows the current version; match documentation to your dependency version.

Source page checked: 2026-10-15. Evidence: [pyo3.rs](https://pyo3.rs/).

### [Introduction to transpilation](https://quantum.cloud.ibm.com/docs/en/guides/transpile)

Documentation · IBM Quantum contributors · open access

**Preparation.** Learn why quantum circuits need routing and transformation for the target hardware.

Source page checked: 2026-10-15. Evidence: [quantum.cloud.ibm.com](https://quantum.cloud.ibm.com/docs/en/guides/transpile).

### [Rust in production at Figma](https://www.figma.com/blog/rust-in-production-at-figma/)

Article · Evan Wallace, Figma · open access

**Alternative approach.** Compare a targeted service rewrite with a compiler migration, including the role of architectural changes in reported results.

Publication date: 2018-05-02.

Historical 2018 account; not a description of the whole current stack. Article sections reviewed; code and reported measurements were not reproduced.

Source page checked: 2026-10-17. Evidence: [www.figma.com](https://www.figma.com/blog/rust-in-production-at-figma/).

Source review: **2026-10-17**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-moving-a-quantum-compiler-from-python-to-rust.json)
