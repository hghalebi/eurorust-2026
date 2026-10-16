# Four questions to guide your reading

Pick the problem closest to your work. Read one source, connect it to a session,
and write a question. These are my suggestions for exploring production agents
through Rust; they are not speaker-endorsed paths or attendance records.

## Types: which transitions should be impossible?

Start with Hoverbear's [Pretty State Machine Patterns in Rust](https://hoverbear.org/blog/rust-state-machine-pattern/).
It compares enums, state-specific structs, and ownership-consuming transitions.
The 2016 examples explain a design pattern; check them on your current toolchain.

Then explore [streaming state machines](related-talks/talks-designing-state-machines-for-streaming-proxies-in-rust.md)
for partial progress and cancellation. Bring this question: **what does each
state prove, and what must still be checked at runtime?**

For compile-time evaluation and future language ideas, use the separate
[compile-time contracts note](../notes/talks/compile-time-contracts.md). It keeps
stable Rust examples apart from nightly and proposed capabilities.

Continue with the [types community path](../community/README.md#types-and-contracts):
read a team discussion or inspect a small compiler task.

## Workers: what happens to accepted work when a process stops?

Start with Tokio's [graceful shutdown guide](https://tokio.rs/tokio/topics/shutdown):
detect shutdown, notify tasks, and wait for them. Then read Amazon's
[timeouts, retries, and backoff with jitter](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/)
for failures across a network. A timeout can leave the remote result unknown;
repeating an operation can repeat its effects.

Connect these to [background-task processing](related-talks/talks-lessons-from-building-a-background-task-processing-library.md).
Ask: **when is work durably recorded, and how is it recovered after a crash?**
Graceful shutdown alone does not answer that question. The
[worker note](../notes/talks/background-workers.md) adds questions about duplicates
and cancellation.

Continue with the [production-agent community path](../community/README.md#production-agents):
prepare a small recovery example before asking a project-specific question.

## Infrastructure: what can be checked before deployment?

Read Encore's [infrastructure planning design](https://dev.to/encore/designing-an-algorithmic-cloud-infrastructure-provisioning-system-kde).
It follows application needs through deployment choices to cloud resources.
This is the vendor's account of its design, useful for understanding the model.

Compare it with [Your Infrastructure Should Compile, Too](related-talks/talks-your-infrastructure-should-compile-too.md),
an Encore-sponsored session. Ask: **which configuration errors can be caught
before deployment, and which require evidence from the running environment?**
The [preparation note](../notes/talks/compiled-infrastructure.md) covers this boundary;
the reading list distinguishes Encore's announced Rust preview from general availability.

Continue with the [infrastructure community path](../community/README.md#infrastructure-configuration):
compare the model with Nix and Snix, while keeping their scope and maturity distinct.

## Robotics: what does a successful message tell you?

Start with the [MAVLink guide](https://mavlink.io/en/) to understand message
definitions, telemetry, and protocols between vehicles and other components.
Then use [the autonomous-systems reading list](related-talks/workshops-rust-on-the-field-integrating-real-autonomous-systems.md)
and [preparation questions](../notes/workshops/robotics-integration.md) to examine
unreliable links. Ask: **what evidence confirms the physical state after a command?**

For a simulation-based introduction, explore the
[line-follower workshop resources](related-talks/workshops-programming-a-line-follower-robot.md).
The field-integration workshop asks for comfort with async Rust and Tokio;
compare its requirements with your background. Confirm workshop access separately.

Continue with the [robotics community path](../community/README.md#robotics-and-embedded-systems):
start with a documentation or simulation task, then check hardware requirements.

## Keep one next step

Save the source, session ID, question, and one idea to test in the
[session note template](../templates/session.md). An agent can propose alternatives
with the [learning prompt](../prompts/explore-a-topic.md); review its reasons and sources.

Sources above checked on October 15, 2026. This pass inspected source pages;
it did not reproduce external code or establish production guarantees.

[All session resources](related-talks/README.md) · [Topic indexes](topics/README.md) · [My choices](../my-conference/README.md)
