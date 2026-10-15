# From Deref Into Borrow, a Primer on Standard Library Traits

[Official EuroRust session](https://eurorust.eu/talks/from-deref-into-borrow-a-primer-on-standard-library-traits/) · [All session resources](README.md)

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

Speakers and their work: [Andre Bogus](../speakers/andre-bogus.md).

## Related talks and workshops

No specific related conference appearance was verified in this pass. The sources below provide topic background.

## Read, watch, and try

### [The Rust Programming Language: Treating Smart Pointers Like Regular References](https://doc.rust-lang.org/book/ch15-02-deref.html)

Book · The Rust Programming Language contributors · open access

**Preparation.** Work through Deref and deref coercion before comparing reference and conversion traits.

Source page checked: 2026-10-15. Evidence: [doc.rust-lang.org](https://doc.rust-lang.org/book/ch15-02-deref.html).

### [Rust API Guidelines: Interoperability](https://rust-lang.github.io/api-guidelines/interoperability.html)

Documentation · Rust API Guidelines contributors · open access

**Suggested next step.** Apply the conversion and common-trait conventions when reviewing a public API.

Source page checked: 2026-10-15. Evidence: [rust-lang.github.io](https://rust-lang.github.io/api-guidelines/interoperability.html).

### [Borrow trait reference](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)

Documentation · Rust project contributors · open access

**Same topic.** Check the equality, ordering, and hashing obligations that distinguish Borrow from a simple reference conversion.

Source page checked: 2026-10-15. Evidence: [doc.rust-lang.org](https://doc.rust-lang.org/std/borrow/trait.Borrow.html).

### [Standard library conversion traits](https://doc.rust-lang.org/std/convert/index.html)

Documentation · Rust project contributors · open access

**Same topic.** Compare AsRef, From, Into, and fallible conversions alongside Deref and Borrow.

Source page checked: 2026-10-15. Evidence: [doc.rust-lang.org](https://doc.rust-lang.org/std/convert/index.html).

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-from-deref-into-borrow-a-primer-on-standard-library-traits.json)
