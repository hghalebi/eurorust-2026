# Give every retry a budget it must spend

**Learning goal:** make a local attempt limit explicit and exhaustion observable.

A positive budget issues attempt tokens that cannot be cloned. Each modeled call
consumes one token. The loop's trait asks only for the operation it needs, and a
runtime failure still spends an attempt.

[Read the implementation](agent-contract-lab/src/retry.rs) or its
[background-worker preparation context](../notes/talks/background-workers.md).

## Run it

```sh
cargo build --release --locked --package agent-contract-lab
target/release/agent-contract-lab retry --budget 3 --failures 2
target/release/agent-contract-lab retry --budget 2 --failures 2
```

| Simulated workload | Expected result |
| --- | --- |
| Budget 3, first 2 calls fail | Completes on attempt 3; exits successfully |
| Budget 2, first 2 calls fail | Reports exhaustion after 2 attempts; exits 2 |

Unit tests check both outcomes. Rustdoc pairs valid use with a compile-fail
example showing that a consumed token cannot be reused.

## Take the contract as far as the evidence goes

The token restricts ownership inside this process. Authentication, cross-process
uniqueness, durable exactly-once execution, and stopping remote effects after
cancellation each require additional mechanisms. For real remote work, design
persistence, idempotency or reconciliation, and observable recovery around the
local retry loop.
