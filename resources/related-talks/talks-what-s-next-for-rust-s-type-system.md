# What's next for Rust's type system

[Official EuroRust session](https://eurorust.eu/talks/what-s-next-for-rust-s-type-system/) · [All session resources](README.md)

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below. Additions checked 2026-10-17; earlier entries retain their individual source dates.

Speakers and their work: [lcnr](../speakers/lcnr.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Review ongoing type-system work and prospective language changes following the next-generation trait solver. Future features are not assumed stable.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Diving Deep: implied bounds and variance](https://lcnr.de/blog/2022/02/05/diving-deep-implied-bounds-and-variance.html). Work through a concrete soundness problem involving lifetime bounds and variance.
- **Bring a question:** Which rule can the type system enforce, and which still needs a runtime check?
- **Other sessions starting at this time:** [Inside Cloud Hypervisor: From KVM to a Running VM](talks-inside-cloud-hypervisor-from-kvm-to-a-running-vm.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

### [-Znext-solver: what, why, and when](https://eurorust.eu/2025/talks/znext-solver/)

Talk · lcnr · public source page

**Preparation.** Read the earlier motivation and design challenges behind the new trait solver.

[EuroRust 2025](https://eurorust.eu/2025/talks/znext-solver/) · 2025-10-09 · Delivery unverified.

Official session description checked; no recording is linked here.

Listing checked: 2026-10-15. Evidence: [eurorust.eu](https://eurorust.eu/2025/talks/znext-solver/).

## Read, watch, and try

### [Enabling the next-generation trait solver on nightly](https://blog.rust-lang.org/2026/08/21/enabling-next-solver-on-nightly/)

Article · lcnr, Rustc Trait System Refactor Initiative · open access

**Project evolution.** Connect the solver redesign to inference changes, compatibility tests, and future language work.

Publication date: 2026-08-21.

A dated nightly rollout report; planned stabilization is not a stable-release guarantee.

Source page checked: 2026-10-15. Evidence: [blog.rust-lang.org](https://blog.rust-lang.org/2026/08/21/enabling-next-solver-on-nightly/).

### [Next-gen trait solving](https://rustc-dev-guide.rust-lang.org/solve/trait-solving.html)

Documentation · Rust compiler contributors · open access

**Suggested next step.** Learn how goals, candidates, and constraints turn trait bounds into compiler decisions.

Source page checked: 2026-10-15. Evidence: [rustc-dev-guide.rust-lang.org](https://rustc-dev-guide.rust-lang.org/solve/trait-solving.html).

### [Diving Deep: implied bounds and variance](https://lcnr.de/blog/2022/02/05/diving-deep-implied-bounds-and-variance.html)

Article · lcnr · open access

**Preparation.** Work through a concrete soundness problem involving lifetime bounds and variance.

Publication date: 2022-02-05.

Historical analysis with an author correction; check the linked issue for current status.

Source page checked: 2026-10-15. Evidence: [lcnr.de](https://lcnr.de/blog/2022/02/05/diving-deep-implied-bounds-and-variance.html).

### [RustBelt: Securing the Foundations of the Rust Programming Language](https://plv.mpi-sws.org/rustbelt/popl18/)

Paper · Ralf Jung, Jacques-Henri Jourdan, Robbert Krebbers, Derek Dreyer · open access

**Suggested next step.** Explore formal foundations behind Rust’s safety argument through the paper, presentation, and linked proof artifact.

POPL 2018; abstract and asset listing reviewed, not proofs or executable artifact.

Source page checked: 2026-10-17. Evidence: [plv.mpi-sws.org](https://plv.mpi-sws.org/rustbelt/popl18/).

### [GhostCell: Separating Permissions from Data in Rust](https://plv.mpi-sws.org/rustbelt/ghostcell/)

Paper · Joshua Yanovski, Hoang-Hai Dang, Ralf Jung, Derek Dreyer · open access

**Alternative approach.** Explore a research design that separates permission from data when reasoning about shared mutable structures.

ICFP 2021; abstract and asset listing reviewed; results not reproduced.

Source page checked: 2026-10-17. Evidence: [plv.mpi-sws.org](https://plv.mpi-sws.org/rustbelt/ghostcell/).

### [Verus publications and projects](https://verus-lang.github.io/verus/publications-and-projects/)

Documentation · Verus project and cited authors · open access

**Suggested next step.** Find work on Rust verification and distinguish a tool’s proved properties from guarantees supplied by Rust’s type system.

Publication index reviewed; each paper and tool needs separate evaluation.

Source page checked: 2026-10-17. Evidence: [verus-lang.github.io](https://verus-lang.github.io/verus/publications-and-projects/).

### [AutoVerus: Automated Proof Generation for Rust Code](https://arxiv.org/abs/2409.13082)

Paper · Chenyuan Yang and coauthors · open access

**Suggested next step.** Explore LLM-assisted proof construction with verification feedback, beyond the properties checked by ordinary Rust types.

OOPSLA 2025, arXiv revision 22 August 2025; abstract and introduction reviewed. Reported results are bounded to the paper's benchmark and were not reproduced here.

Source page checked: 2026-10-17. Evidence: [arxiv.org](https://arxiv.org/abs/2409.13082).

### [VerusBelt: A Semantic Foundation for Verus’s Proof-Oriented Extensions to the Rust Type System](https://iris-project.org/pdfs/2026-pldi-verusbelt.pdf)

Paper · Travis Hance, Laila Elbeheiry, Yusuke Matsushita, Derek Dreyer · open access

**Suggested next step.** Read research on the semantic foundations of Verus’s proof-oriented types, borrowing, and concurrency.

PLDI 2026; abstract and introduction reviewed. The paper studies a subset and does not verify the Verus implementation or all Rust libraries.

Source page checked: 2026-10-17. Evidence: [iris-project.org](https://iris-project.org/pdfs/2026-pldi-verusbelt.pdf).

Source review: **2026-10-17**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-what-s-next-for-rust-s-type-system.json)
