# Preparing for background-worker design

Status: source-based preparation, not attendance. Session ID:
`talks-lessons-from-building-a-background-task-processing-library`. Source:
[official session](https://eurorust.eu/talks/lessons-from-building-a-background-task-processing-library/).

The abstract describes a task library's evolving APIs, dependency injection,
middleware, shutdown, and rolling restarts. These concerns are relevant to
production agent workers independently of the choice of reasoning model.

Bring questions about duplicate work, cancellation, crash recovery, retry
classification, and durable acknowledgments. Distinguish a local API ownership
guarantee from persistence or remote-effect guarantees. Compare trait-based
capabilities with concrete dependencies and ask which bounds are truly required.

The [retry experiment](../../experiments/owned-retry-budget.md) shows a bounded
local attempt loop with simulated failures. It does not demonstrate restart-safe
job execution. Record actual lessons and results only after observation.
