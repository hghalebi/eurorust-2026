# Beyond Flamegraphs: Precise Rust Profiling with hotpath-rs

[Official EuroRust session](https://eurorust.eu/talks/beyond-flamegraphs-precise-rust-profiling-with-hotpath-rs/) · [All session resources](README.md)

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

Speakers and their work: [Paweł Urbanek](../speakers/pawel-urbanek.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Compare instrumentation-oriented profiling with sampling approaches and examine measurement overhead in async-heavy programs.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Rust Performance Profiling using hotpath-rs](https://www.youtube.com/watch?v=ir1WC_rO2Fk). Watch the speaker's earlier profiling explanation and Zed instrumentation demonstration.
- **Bring a question:** How much overhead does the measurement add, and which delays does it reveal or miss?
- **Other sessions starting at this time:** [The Lämp](talks-the-lamp.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

### [Rust Performance Profiling using hotpath-rs](https://www.youtube.com/watch?v=ir1WC_rO2Fk)

Video · Paweł Urbanek · open access

**Same topic.** Watch the speaker's earlier profiling explanation and Zed instrumentation demonstration.

[Rustikon 2026](https://www.rustikon.dev/talk/improving-rust-performance-debugging-workflow-with-tuis-and-llms) · 2026-03-20 · Published.

The speaker links the recording; the organizer dates the related hotpath session March 20, 2026. YouTube playback was not checked.

Listing checked: 2026-10-15. Evidence: [hotpath.rs](https://hotpath.rs/blog/rust-hotpath-rustikon), [www.rustikon.dev](https://www.rustikon.dev/talk/improving-rust-performance-debugging-workflow-with-tuis-and-llms).

### [Tango: Precise Performance Measurement through Direct Comparison](https://2025.rustweek.org/talks/denis/)

Talk · Denis Bazhenov · public source page

**Suggested next step.** Follow profiling with paired benchmarks that reduce noise when comparing candidate changes.

[RustWeek 2025](https://2025.rustweek.org/talks/denis/) · Delivery unverified.

The official abstract describes benchmarking, a complementary task rather than profiling; recording not checked.

Listing checked: 2026-10-15. Evidence: [2025.rustweek.org](https://2025.rustweek.org/talks/denis/).

## Read, watch, and try

### [Lessons Learned Building High-Performance Rust Profiler](https://hotpath.rs/blog/rust-performance-profiling)

Article · Paweł Urbanek · open access

**Same topic.** Read the implementation tradeoffs around instrumentation overhead, cache contention, and async futures.

Publication date: 2026-05-12.

Source page checked: 2026-10-15. Evidence: [hotpath.rs](https://hotpath.rs/blog/rust-performance-profiling).

### [The Rust Performance Book: Profiling](https://nnethercote.github.io/perf-book/profiling.html)

Book · Nicholas Nethercote, The Rust Performance Book contributors · open access

**Alternative approach.** Compare sampling and instrumentation tools before choosing what to measure.

Source page checked: 2026-10-15. Evidence: [nnethercote.github.io](https://nnethercote.github.io/perf-book/profiling.html).

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-beyond-flamegraphs-precise-rust-profiling-with-hotpath-rs.json)
