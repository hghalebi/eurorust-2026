# Which tool contracts can the compiler check?

**Status: source-based preparation.** Session: `talks-move-your-problems`.
[Read the official description](https://eurorust.eu/talks/move-your-problems/).

For production agents, tool boundaries are a useful place to ask what Rust can
make harder to misuse. This session's description covers const evaluation,
diagnostics, reflection, and possible future capabilities. Some material is
nightly or still in design; check the status of a feature before using it.

## Questions to bring

- Which checks can stable Rust express today?
- How can a type-driven API explain an invalid input clearly?
- Which facts still need runtime evidence, such as permissions or source freshness?

## Try the foundation

The [checked-input experiment](../../experiments/checked-tool-input.md) uses
validated nominal types to separate a tool name from its timeout. It is a small
existing-Rust example; it does not implement the session's future proposals.

After attending or watching a published recording, add attributed answers and
code checked on a named toolchain.

[Explore const evaluation, reflection, and related work](../../resources/related-talks/talks-move-your-problems.md).
