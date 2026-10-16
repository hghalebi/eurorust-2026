# Designing State Machines for Streaming Proxies in Rust

[Official EuroRust session](https://eurorust.eu/talks/designing-state-machines-for-streaming-proxies-in-rust/) · [All session resources](README.md)

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below. Additions checked 2026-10-17; earlier entries retain their individual source dates.

Speakers and their work: [Sanskar Jaiswal](../speakers/sanskar-jaiswal.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Model partial streaming progress, cancellation, buffering, and connection reuse as explicit states and transitions in a Rust proxy.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Pretty State Machine Patterns in Rust](https://hoverbear.org/blog/rust-state-machine-pattern/). Compare enums and owned transitions for representing valid protocol states.
- **Bring a question:** Which rule can the type system enforce, and which still needs a runtime check?
- **Other sessions starting at this time:** [Blastbeat: When a Decade of C++ Meets Rust and io\_uring](talks-blastbeat-when-a-decade-of-c-meets-rust-and-io-uring.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

No specific related conference appearance was verified in this pass. The sources below provide topic background.

## Read, watch, and try

### [Pretty State Machine Patterns in Rust](https://hoverbear.org/blog/rust-state-machine-pattern/)

Article · Hoverbear · open access

**Preparation.** Compare enums and owned transitions for representing valid protocol states.

Publication date: 2016-10-12.

A historical design article; examples were not compiled against current Rust.

Source page checked: 2026-10-15. Evidence: [hoverbear.org](https://hoverbear.org/blog/rust-state-machine-pattern/).

### [How we built Pingora, the proxy that connects Cloudflare to the Internet](https://blog.cloudflare.com/how-we-built-pingora-the-proxy-that-connects-cloudflare-to-the-internet/)

Article · Yuchen Wu, Andrew Hauck · open access

**Same topic.** Study a production Rust proxy's architecture and connection reuse decisions.

Publication date: 2022-09-14.

A separate proxy implementation; its design is not attributed to the EuroRust speaker.

Source page checked: 2026-10-15. Evidence: [blog.cloudflare.com](https://blog.cloudflare.com/how-we-built-pingora-the-proxy-that-connects-cloudflare-to-the-internet/).

### [Tokio tutorial: Select](https://tokio.rs/tokio/tutorial/select)

Documentation · Tokio contributors · open access

**Preparation.** Understand what happens to unfinished futures when another stream or cancellation branch wins.

Source page checked: 2026-10-15. Evidence: [tokio.rs](https://tokio.rs/tokio/tutorial/select).

### [Resolving a request smuggling vulnerability in Pingora](https://blog.cloudflare.com/resolving-a-request-smuggling-vulnerability-in-pingora/)

Article · Cloudflare · open access

**Suggested next step.** Study a proxy protocol failure alongside the Pingora design account; memory safety alone does not establish protocol correctness.

Historical 2025 report; old fixed versions are not current upgrade advice. Article sections reviewed; code and reported measurements were not reproduced.

Source page checked: 2026-10-17. Evidence: [blog.cloudflare.com](https://blog.cloudflare.com/resolving-a-request-smuggling-vulnerability-in-pingora/).

Source review: **2026-10-17**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-designing-state-machines-for-streaming-proxies-in-rust.json)
