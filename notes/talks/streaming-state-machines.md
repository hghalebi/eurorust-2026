# Preparing for streaming state machines

Status: source-based preparation, not attendance. Session ID:
`talks-designing-state-machines-for-streaming-proxies-in-rust`. Source:
[official session](https://eurorust.eu/talks/designing-state-machines-for-streaming-proxies-in-rust/).

The session's stated scope includes partial bodies, concurrent progress,
cancellation, buffering, and connection reuse. Those are useful analogies for
streamed tool output and long-running agent operations.

Ask which state transitions consume ownership, what must be retained when a
future is cancelled, and how connection reuse is justified after failure. A
typed state can restrict an operation without forcing completion or proving an
external resource is still healthy. Keep such runtime evidence explicit.

After observing the session, connect one concrete pattern to a tested follow-up
rather than assuming proxy-specific code transfers unchanged to agent workers.
