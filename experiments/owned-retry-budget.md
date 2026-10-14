# Experiment: an owned local retry budget

Status: learning example with a simulated workload. A positive budget issues
non-cloneable attempt tokens, each consumed by one modeled call. The trait asks
only for the operation required by the loop. Runtime failures consume attempts,
and exhaustion is explicit.

Source: [retry.rs](agent-contract-lab/src/retry.rs). Preparation context:
[background workers](../notes/talks/background-workers.md).

```sh
cargo build --release --locked --package agent-contract-lab
target/release/agent-contract-lab retry --budget 3 --failures 2
target/release/agent-contract-lab retry --budget 2 --failures 2
```

The first completes on attempt 3. The second reports exhaustion after 2 attempts
and exits 2. Unit tests check both outcomes; rustdoc checks that reusing a consumed
token fails to compile and that valid use succeeds.

The token proves only a local ownership restriction. It is not authentication,
cross-process uniqueness, durable exactly-once execution, or a guarantee that an
external effect stopped after cancellation. Real remote work needs appropriate
idempotency/reconciliation, persistence, and observable recovery behavior.
