# Preparing for compile-time contracts

Status: source-based preparation, not attendance. Session ID:
`talks-move-your-problems`. Source:
[official session](https://eurorust.eu/talks/move-your-problems/).

The abstract discusses const evaluation, diagnostic quality, reflection, and
possible future capabilities. Some material is nightly or still being designed;
do not assume every proposal is available in stable Rust.

For production tools, look for ways to make invalid inputs and combinations
harder to express while retaining runtime checks for permissions, freshness, and
external effects. Ask which checks are stable today, how failures are explained,
and what remains dynamic. Compare with
[the checked-input experiment](../../experiments/checked-tool-input.md), which
uses ordinary validated nominal types and makes no claim to implement the talk's
future proposals.

After attending or watching a published recording, add attributed observations,
questions answered, and code tested on a stated toolchain.
