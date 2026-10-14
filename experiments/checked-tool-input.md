# Experiment: checked tool inputs

Status: learning example. The contract requires a syntactically valid tool name
and a positive timeout. A CLI DTO is consumed into checked domain values; the
name and timeout cannot be swapped in an API call. Representation fields remain
private. This validates syntax, not tool existence, authorization, or completion.

Source: [input.rs](agent-contract-lab/src/input.rs). Preparation context:
[compile-time contracts](../notes/talks/compile-time-contracts.md).

```sh
cargo build --release --locked --package agent-contract-lab
target/release/agent-contract-lab input --tool index-code --timeout-ms 500
target/release/agent-contract-lab input --tool index-code --timeout-ms 0
```

The first command prints `tool=index-code timeout=500ms` and succeeds. The second
rejects the contract and exits 1. Tests cover exact-name preservation, invalid
inputs, and a compiling/compile-fail pair for swapped nominal types. No tool is
executed by this experiment.
