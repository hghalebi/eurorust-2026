# Supply Chain Attacks, and how to protect ourselves

[Official EuroRust session](https://eurorust.eu/talks/supply-chain-attacks-and-how-to-protect-ourselves/) · [All session resources](README.md)

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below. Additions checked 2026-10-17; earlier entries retain their individual source dates.

Speakers and their work: [Francois Mockers](../speakers/francois-mockers.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Map dependency and build-time exposure to available auditing, review, and publishing practices in the Rust ecosystem.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [RustSec Advisory Database](https://rustsec.org/). Find published crate advisories and understand the inputs used by dependency audit tools.
- **Bring a question:** What confirms the intended configuration became the actual running state?
- **Other sessions starting at this time:** [Qt Bridges: Bringing a mature C++ UI framework to Rust](talks-qt-bridges-bringing-a-mature-c-ui-framework-to-rust.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

No specific related conference appearance was verified in this pass. The sources below provide topic background.

## Read, watch, and try

### [RustSec Advisory Database](https://rustsec.org/)

Documentation · Rust Secure Code Working Group · open access

**Suggested next step.** Find published crate advisories and understand the inputs used by dependency audit tools.

Source page checked: 2026-10-15. Evidence: [rustsec.org](https://rustsec.org/).

### [Cargo Vet guide](https://mozilla.github.io/cargo-vet/)

Documentation · Cargo Vet contributors · open access

**Suggested next step.** Learn how reviewed versions and audit records can support dependency policy.

Review evidence and advisory checks cover different risks; neither proves a dependency harmless.

Source page checked: 2026-10-15. Evidence: [mozilla.github.io](https://mozilla.github.io/cargo-vet/).

### [Be alert: targeted attacks on prominent Rustaceans](https://blog.rust-lang.org/2026/09/17/targeted-attacks/)

Article · Adam Harvey, crates.io team, Rust Security Response Working Group · open access

**Same topic.** Connect account compromise and social engineering to the path malicious releases can take.

Publication date: 2026-09-17.

Source page checked: 2026-10-15. Evidence: [blog.rust-lang.org](https://blog.rust-lang.org/2026/09/17/targeted-attacks/).

### [Auditing Rust Crates Effectively](https://arxiv.org/abs/2602.06466)

Paper · Lydia Zoghbi, David Thien, Ranjit Jhala, Deian Stefan, Caleb Stanford · open access

**Suggested next step.** Explore effect analysis that narrows the code requiring manual dependency review.

Publication date: 2026-02-06.

Research preprint; metadata and abstract checked, with no independent validation of its evaluation.

Source page checked: 2026-10-15. Evidence: [arxiv.org](https://arxiv.org/abs/2602.06466).

### [OSTIF audit catalog](https://ostif.org/audits/)

Documentation · Open Source Technology Improvement Fund · open access

**Suggested next step.** Find original audit reports and compare their dates and scope with the dependencies you want to assess.

Catalog review only; audit findings apply to the stated scope and date. Index reviewed; linked works need separate review.

Source page checked: 2026-10-17. Evidence: [ostif.org](https://ostif.org/audits/).

### [rustls security advisories](https://github.com/rustls/rustls/security/advisories)

Documentation · rustls maintainers · open access

**Suggested next step.** Use maintainers’ advisories as an example of tracing an affected dependency version to a specific report.

Index reviewed; resolve a specific version against the individual advisory.

Source page checked: 2026-10-17. Evidence: [github.com](https://github.com/rustls/rustls/security/advisories).

Source review: **2026-10-17**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-supply-chain-attacks-and-how-to-protect-ourselves.json)
