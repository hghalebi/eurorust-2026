# From Deref Into Borrow, a Primer on Standard Library Traits

[Official EuroRust session](https://eurorust.eu/talks/from-deref-into-borrow-a-primer-on-standard-library-traits/) · [All session resources](README.md)

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below. Additions checked 2026-10-17; earlier entries retain their individual source dates.

Speakers and their work: [Andre Bogus](../speakers/andre-bogus.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Clarify the different responsibilities of standard conversion and reference traits when designing Rust APIs.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [The Rust Programming Language: Treating Smart Pointers Like Regular References](https://doc.rust-lang.org/book/ch15-02-deref.html). Work through Deref and deref coercion before comparing reference and conversion traits.
- **Bring a question:** Which rule can the type system enforce, and which still needs a runtime check?
- **Other sessions starting at this time:** [Fearless Concurrency on the GPU](talks-fearless-concurrency-on-the-gpu.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

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

### [Brown's experimental Rust Book](https://rust-book.cs.brown.edu/)

Book · Brown University Cognitive Engineering Lab · open access

**Preparation.** Review ownership and borrowing with visual explanations before comparing reference and conversion traits.

Experimental adaptation of the Rust Book; quiz behavior not tested. Introduction reviewed; the full work and exercises were not reviewed or executed.

Source page checked: 2026-10-17. Evidence: [rust-book.cs.brown.edu](https://rust-book.cs.brown.edu/).

### [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

Documentation · Rust library team and contributors · open access

**Suggested next step.** Use the full API checklist after reading the interoperability chapter already linked for this session.

Guidance rather than a mandate; a chapter is already linked in this repository.

Source page checked: 2026-10-17. Evidence: [rust-lang.github.io](https://rust-lang.github.io/api-guidelines/).

### [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)

Book · Rust Design Patterns contributors · open access

**Suggested next step.** Compare API choices with community explanations of Rust idioms, patterns, and anti-patterns.

Community guide; individual examples not evaluated. Introduction reviewed; the full work and exercises were not reviewed or executed.

Source page checked: 2026-10-17. Evidence: [rust-unofficial.github.io](https://rust-unofficial.github.io/patterns/).

### [Oxide: The Essence of Rust](https://arxiv.org/abs/1903.00982)

Paper · Aaron Weiss, Olek Gierczak, Daniel Patterson, Amal Ahmed · open access

**Suggested next step.** Follow ownership and borrowing explanations into a formal model of a Rust-like language.

Abstract and publication metadata reviewed; the full paper was not reviewed. This research language is separate from Oxide Computer Company.

Source page checked: 2026-10-17. Evidence: [arxiv.org](https://arxiv.org/abs/1903.00982).

Source review: **2026-10-17**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-from-deref-into-borrow-a-primer-on-standard-library-traits.json)
