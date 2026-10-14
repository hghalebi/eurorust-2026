# Turn a conference question into a small Rust experiment

These optional code examples follow questions from the
[preparation notes](../notes/README.md). Read the explanations for the idea, or
run an example when you want to explore it in Rust. Each isolates one contract,
shows valid and invalid use, and makes the result observable:

| Question | Experiment | What you can check |
| --- | --- | --- |
| Can a tool API reject invalid or mixed-up inputs? | [Checked tool inputs](checked-tool-input.md) | Valid construction, invalid input, and nominal type separation |
| Can one retry attempt be used twice? | [An owned retry budget](owned-retry-budget.md) | Consumed attempt tokens, simulated recovery, and exhaustion |

Both live in `agent-contract-lab`. Run the documented compiled binaries and
`cargo test --workspace --locked` for unit and rustdoc evidence.

These are local learning examples. They execute no external tools or robotic
systems, and they establish no production recovery guarantee. For a contribution,
state the contract, show the failure case, and explain what remains outside it.
