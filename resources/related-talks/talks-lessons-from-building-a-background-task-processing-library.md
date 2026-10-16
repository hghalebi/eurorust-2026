# Lessons from building a background task processing library

[Official EuroRust session](https://eurorust.eu/talks/lessons-from-building-a-background-task-processing-library/) · [All session resources](README.md)

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass. Additions checked 2026-10-17; earlier entries retain their individual source dates.

Speakers and their work: [Njuguna Mureithi](../speakers/njuguna-mureithi.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Explore worker API evolution, extensibility, dependency injection, and production shutdown/restart concerns in a Rust task-processing library.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Graceful Shutdown](https://tokio.rs/tokio/topics/shutdown). Explains cancellation and task completion when workers stop or restart.
- **Bring a question:** What happens if this work is cancelled or the process stops halfway through?
- **Other sessions starting at this time:** [Running Safely with Scissors: Lock-free Concurrency for Maximum Throughput](talks-running-safely-with-scissors-lock-free-concurrency-for-maximum-throughput.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

No specific related conference appearance was verified in this pass. The sources below provide topic background.

## Read, watch, and try

### [Apalis](https://apalis.dev/)

Project · Njuguna Mureithi, Apalis contributors · open access

**Same topic.** Provides the worker API, storage backends and shutdown behavior behind the speaker’s library.

Source page checked: 2026-10-15. Evidence: [apalis.dev](https://apalis.dev/).

### [Background job processing with Rust using Actix and Redis](https://mureithi.me/blog/background-job-processing-with-rust-actix-redis)

Article · Njuguna Mureithi · open access

**Project evolution.** Shows an earlier design for separating request handling from background job execution.

Historical article; examples may use older APIs than current Apalis.

Source page checked: 2026-10-15. Evidence: [mureithi.me](https://mureithi.me/blog/background-job-processing-with-rust-actix-redis).

### [Graceful Shutdown](https://tokio.rs/tokio/topics/shutdown)

Documentation · Tokio contributors · open access

**Preparation.** Explains cancellation and task completion when workers stop or restart.

Source page checked: 2026-10-15. Evidence: [tokio.rs](https://tokio.rs/tokio/topics/shutdown).

### [Timeouts, retries, and backoff with jitter](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/)

Article · Amazon Builders’ Library · open access

**Suggested next step.** Explains retry budgets and backoff choices that keep background workers from multiplying failures.

Source page checked: 2026-10-15. Evidence: [aws.amazon.com](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/).

### [Dealing with cancel safety in async Rust](https://rfd.shared.oxide.computer/rfd/0400)

Article · Rain Paharia, Oxide · open access

**Suggested next step.** Examine cancellation and partial progress when a worker stops; compare task completion with durable recovery.

Living engineering document; dated examples need version checks. Article sections reviewed; code and reported measurements were not reproduced.

Source page checked: 2026-10-17. Evidence: [rfd.shared.oxide.computer](https://rfd.shared.oxide.computer/rfd/0400).

Source review: **2026-10-17**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-lessons-from-building-a-background-task-processing-library.json)
