# Small Rust experiments

Experiments demonstrate a bounded engineering lesson with commands, expected
behavior, negative cases, and limits. They are independent learning examples,
not production-agent frameworks or proof of an external system's recovery.

Read [checked tool inputs](checked-tool-input.md) and
[an owned retry budget](owned-retry-budget.md). Both are implemented in
`agent-contract-lab`, with valid use, invalid input, compile-fail examples, and
observable success/failure commands. Run `cargo test --workspace --locked` and
the documented commands. No real external tool or robotic system is exercised.
