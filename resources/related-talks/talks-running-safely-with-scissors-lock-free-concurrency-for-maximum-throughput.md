# Running Safely with Scissors: Lock-free Concurrency for Maximum Throughput

[Official EuroRust session](https://eurorust.eu/talks/running-safely-with-scissors-lock-free-concurrency-for-maximum-throughput/) · [All session resources](README.md)

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed.

Speakers and their work: [Filip Petkovski](../speakers/filip-petkovski.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Study append-oriented metrics storage and type-constrained concurrency, including the boundary between safe APIs and carefully isolated unsafe implementation.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Rust Atomics and Locks](https://mara.nl/atomics/). Builds the memory-ordering and synchronization knowledge needed to assess lock-free code.
- **Bring a question:** Which rule can the type system enforce, and which still needs a runtime check?
- **Other sessions starting at this time:** [Lessons from building a background task processing library](talks-lessons-from-building-a-background-task-processing-library.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

### [Talking to ghosts](https://rustlab.it/talks/talking-to-ghosts)

Talk · Arya Dradjica · registration

**Same topic.** Offers another treatment of concurrent memory reclamation and its API tradeoffs.

[RustLab 2026](https://rustlab.it/talks/talking-to-ghosts) · 2026-11-03 · Announced.

Announced session on November 3, 2026; no recording is claimed.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/talking-to-ghosts).

## Read, watch, and try

### [Rust Atomics and Locks](https://mara.nl/atomics/)

Book · Mara Bos · open access

**Preparation.** Builds the memory-ordering and synchronization knowledge needed to assess lock-free code.

Free online edition; the book’s introduction and contents were checked.

Listing checked: 2026-10-15. Evidence: [mara.nl](https://mara.nl/atomics/).

### [Lock-freedom without garbage collection](https://aturon.github.io/blog/2015/08/27/epoch/)

Article · Aaron Turon · open access

**Same topic.** Explains epoch-based reclamation and how Rust types can protect concurrent data-structure APIs.

Publication date: 2015-08-27.

Historical Crossbeam design article; current library APIs and benchmark results may differ.

Source page checked: 2026-10-15. Evidence: [aturon.github.io](https://aturon.github.io/blog/2015/08/27/epoch/).

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-running-safely-with-scissors-lock-free-concurrency-for-maximum-throughput.json)
