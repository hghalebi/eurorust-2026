# Catch a bad tool request at the boundary

**Learning goal:** turn raw CLI input into values the rest of the API can trust.

This example requires a syntactically valid tool name and a positive timeout.
Checked construction consumes the input DTO. Distinct private-field types keep
a name and a timeout from being swapped in an API call.

[Read the implementation](agent-contract-lab/src/input.rs) or its
[conference preparation context](../notes/talks/compile-time-contracts.md).

## Run it

```sh
cargo build --release --locked --package agent-contract-lab
target/release/agent-contract-lab input --tool index-code --timeout-ms 500
target/release/agent-contract-lab input --tool index-code --timeout-ms 0
```

| Request | Expected result |
| --- | --- |
| `index-code` with a 500 ms timeout | Prints `tool=index-code timeout=500ms`; exits successfully |
| `index-code` with a zero timeout | Rejects the request; exits 1 |

Tests cover exact-name preservation, invalid inputs, and a valid/compile-fail
pair for swapped types.

## What the check means

The values satisfy the local syntax and timeout contract. Tool existence,
authorization, and completion require separate evidence. This experiment invokes
no tool; use that boundary when deciding what the types actually prove.
