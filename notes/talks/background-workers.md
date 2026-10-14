# What happens around the worker loop?

**Status: source-based preparation.** Session:
`talks-lessons-from-building-a-background-task-processing-library`.
[Read the official description](https://eurorust.eu/talks/lessons-from-building-a-background-task-processing-library/).

Production agent work needs a place to live between model calls: a task API,
dependencies, shutdown behavior, and a recovery path. The session's description
covers an evolving task library, dependency injection, middleware, shutdown,
and rolling restarts.

## Questions to bring

- How are duplicate work, cancellation, and retryable failures represented?
- What survives a crash, and when is an acknowledgment durable?
- Which capabilities belong behind traits, and which bounds are actually needed?

## Try one bounded contract

The [retry-budget experiment](../../experiments/owned-retry-budget.md) models a
local attempt limit with simulated failures. It tests ownership and exhaustion;
it provides no restart-safe job execution. Use the session to examine the
persistence and recovery requirements around that smaller contract.

Add actual lessons and measured outcomes after observing them.
