# What should infrastructure catch before delivery?

**Status: source-based preparation.** Session:
`talks-your-infrastructure-should-compile-too`.
[Read the official description](https://eurorust.eu/talks/your-infrastructure-should-compile-too/).

This Encore-sponsored session explores moving infrastructure integration checks into
development and compilation. That fits my interest in expressing infrastructure
through Rust and making configuration assumptions easier to inspect.

## Questions to bring

- How does the API represent configuration meaning and integration constraints?
- Which mismatches can compilation catch, and which depend on the environment?
- How are secrets handled, and where does actual deployment begin?

## Keep the evidence attached

Compare the approach with build-system and observability ideas. When trying an
example, record the tool, version, checked contract, and deployment result
separately. Compilation can support a configuration guarantee; remote deployment
and recovery still need their own checks.

[Compare infrastructure design articles and related talks](../../resources/related-talks/talks-your-infrastructure-should-compile-too.md).
