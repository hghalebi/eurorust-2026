# Explore a question in Rust

These optional examples follow the [preparation notes](../notes/README.md).
Read the explanation or run the code. Each shows one contract and its failure cases.

| Question | Experiment | What it checks |
| --- | --- | --- |
| Can a tool reject invalid or mixed-up inputs? | [Checked tool inputs](checked-tool-input.md) | Valid construction, invalid input, nominal type separation |
| Can a retry attempt be used twice? | [An owned retry budget](owned-retry-budget.md) | Consumed tokens, simulated recovery, exhaustion |

Both live in `agent-contract-lab`. Use the documented compiled binaries and
`cargo test --workspace --locked` for unit and rustdoc checks.

These local examples call no external tools or robots and prove no production
recovery guarantee. Contributions should explain the contract, failure case,
and limits.
