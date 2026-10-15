# What survives when a stream stops halfway?

**Status: source-based preparation.** Session:
`talks-designing-state-machines-for-streaming-proxies-in-rust`.
[Read the official description](https://eurorust.eu/talks/designing-state-machines-for-streaming-proxies-in-rust/).

The stated scope includes partial bodies, concurrent progress, cancellation,
buffering, and connection reuse. Those concerns offer useful questions for
streamed tool output and long-running agent operations.

## Questions to bring

- Which transitions consume ownership, and what does each state prove?
- What progress or resource state must survive cancellation?
- What runtime evidence makes connection reuse safe after a failure?

## Apply with care

A typed state can restrict an operation. It cannot force completion or establish
that an external resource is still healthy. After the session, choose one
observed pattern and test its fit for an agent worker, including the failure
path. Record where proxy-specific assumptions stop applying.

[Read about state machines, Pingora, and async cancellation](../../resources/related-talks/talks-designing-state-machines-for-streaming-proxies-in-rust.md).
