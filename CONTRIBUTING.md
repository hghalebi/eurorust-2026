# Contributing

Small corrections are useful: identify the session ID, link the official source,
and explain what changed. Notes, workshop setup discoveries, original summaries,
translations, learning paths, and focused code changes are welcome.

## Where to contribute

| Change | Owning files | Evidence |
| --- | --- | --- |
| Official schedule extraction | [collector](tools/conference-data/src/collect.rs) | Synthetic fixtures and parser tests |
| Summary, topics, setup, prerequisites | [annotations](community/annotations/sessions.json) | Session ID, source URL, original paraphrase, author, method |
| Source discrepancy | [corrections](community/corrections/README.md) | Source link, observation time, affected field |
| Personal learning path | [paths](community/paths/README.md) | Fictional/shared profile, snapshot, schedule check |
| Session notes | [note template](templates/session.md) | Attendance context, attribution, explicit uncertainties |
| Rust example | [experiments](experiments/README.md) | Goal, commands, valid and invalid behavior |

Generated views are reproduced from checked data. Change annotations or extraction
logic, then refresh or export as described in [the data guide](docs/data-guide.md).
An annotation is a community interpretation, even when it paraphrases an official
prerequisite. Leave unsupported fields null. Do not upload private participant
profiles, source-page caches, attendee contact lists, or unlicensed slide decks.

## Checks

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --release --locked
target/release/conference-data validate
target/release/conference-data check-docs
```

Run `check-itinerary` for any changed example path. Add a focused fixture for a
parser change and test malformed input as well as valid input. Type guarantees
need compiling valid use and meaningful compile-fail cases. Public docs link to
concrete owning files; update [the inventory](docs/document-map.json) when moving,
adding, or reclassifying documents. Label links to historical material clearly.
Keep release receipts and snapshot history; update pointers rather than rewriting
old observations.

## Pull requests

State the problem, resulting behavior, supporting source, and checks actually
run. Explain unknowns and compatibility changes. Keep PRs focused. The refresh
workflow proposes changes for review rather than pushing generated changes
directly to the default branch.

## Runtime troubleshooting

Start with a small read-only command such as `git status --short`. If the tool
fails before shell execution, inspect the harness installation and running
version/process owner using that harness's supported recovery procedure. Run a
fresh command before declaring recovery. When commands run but parsing fails,
follow the collector fixture/data-validation route. Record machine-specific
diagnostics locally; portable instructions belong here.
