# Building a high performance, thread-per-core, zero-copy OpenTelemetry engine in Rust

[Official EuroRust session](https://eurorust.eu/talks/building-a-high-performance-thread-per-core-zero-copy-opentelemetry-engine-in-rust/) · [All session resources](README.md)

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

Speakers and their work: [Albert Lockett](../speakers/albert-lockett.md).

## Related talks and workshops

No specific related conference appearance was verified in this pass. The sources below provide topic background.

## Read, watch, and try

### [OTel-Arrow Phase 2: From Efficient Transport to Efficient Telemetry Pipelines](https://opentelemetry.io/blog/2026/otel-arrow-phase-2/)

Article · Laurent Querel, Joshua MacDonald, Reiley Yang, Albert Lockett, Cijo Thomas, Drew Relmas, Jake Dern · open access

**Same topic.** Read the coauthored account of bounded pipelines, thread-per-core execution, and columnar telemetry.

Publication date: 2026-06-12.

Reported benchmark results apply to the documented hardware and configurations.

Source page checked: 2026-10-15. Evidence: [opentelemetry.io](https://opentelemetry.io/blog/2026/otel-arrow-phase-2/).

### [OpenTelemetry Arrow](https://github.com/open-telemetry/otel-arrow)

Project · OpenTelemetry Arrow contributors · open access

**Suggested next step.** Inspect the protocol, dataflow engine, and benchmark material behind the architecture.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/open-telemetry/otel-arrow).

### [Arrow Columnar Format](https://arrow.apache.org/docs/format/Columnar.html)

Documentation · Apache Arrow contributors · open access

**Preparation.** Understand the columnar buffers that make direct telemetry processing possible.

Source page checked: 2026-10-15. Evidence: [arrow.apache.org](https://arrow.apache.org/docs/format/Columnar.html).

### [Shared-nothing Design](https://seastar.io/shared-nothing/)

Article · Seastar contributors · open access

**Preparation.** Compare per-core ownership and explicit message passing with shared-state concurrency.

Seastar's examples are C++; this is architectural background, not an OTel Rust API guide.

Source page checked: 2026-10-15. Evidence: [seastar.io](https://seastar.io/shared-nothing/).

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-building-a-high-performance-thread-per-core-zero-copy-opentelemetry-engine-in-rust.json)
