# Help someone make a better conference choice

A contribution can be one corrected fact, one useful setup note, or one clear
explanation of why you chose a talk. Start with the smallest improvement you can
support with evidence. [Starter issues](https://github.com/hghalebi/eurorust-2026/issues)
are open for annotation reviews, learning paths, and translations.

## Where to contribute

| You can help with… | Start here | Include this evidence |
| --- | --- | --- |
| An incorrect or conflicting fact | [Correction guide](community/corrections/README.md) | Session ID, snapshot, official link, affected field, and time checked |
| A summary, topic, prerequisite, or setup note | [Session annotations](community/annotations/sessions.json) | Original wording, author, method, source URL, checked date and source hash |
| A useful route through the conference | [Learning paths](community/paths/README.md) | Audience, shareable assumptions, snapshot, session IDs, reasons, and alternatives |
| A lesson or workshop discovery | [Session note template](templates/session.md) | Participation context, source attribution, observations, and open questions |
| A translation | [Planning guide](docs/planning-guide.md) or [agent prompt](prompts/choose-my-path.md) | Language, translator, original document, and working links |
| A small Rust example | [Experiments](experiments/README.md) | Learning goal, commands, expected behavior, failure cases, and limits |
| Better schedule extraction | [Collector](tools/conference-data/src/collect.rs) | A focused synthetic fixture, the failure it exposes, and parser checks |

For a prose contribution, start with the linked file or template. You can propose
an improvement through GitHub without setting up the scraper.

## Make your change easy to trust

Write summaries in your own words and link the official session. Separate source
facts, community interpretation, and your own observations. Record preparation,
attendance, and watching a recording as different contexts. Leave unsupported
fields `null`.

Annotation changes are applied by `refresh`, which retrieves official sources.
`export` rebuilds views from an existing checked catalog; it does not apply edits
to the annotation file. Use the [data guide](docs/data-guide.md) for the right
regeneration route. Edit the owning inputs so your improvement survives refresh.

Share only material you have authority to publish. Keep real participant profiles,
source caches, credentials, attendee contact lists, and unlicensed slide decks
out of public changes. Follow the [source policy](docs/source-and-reuse-policy.md)
and [community conduct](CODE_OF_CONDUCT.md).

## Checks

From the repository root, with a current stable Rust toolchain:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --release --locked
target/release/conference-data validate
target/release/conference-data check-docs
```

Run `check-itinerary` for each changed JSON path. Parser changes need a focused
fixture with valid and malformed input. Type guarantees need both valid use and
a compile-fail example that fails for the intended reason.

For a new, moved, or reclassified document, update the [document inventory](docs/document-map.json)
and the links that lead to it. Label historical references clearly. Keep recorded
validation receipts and immutable snapshot history; update current pointers when
new evidence arrives.

## Pull requests

Lead with the reader's problem and what your change improves. Include the source,
affected session IDs and snapshot, checks actually run, and any remaining unknowns
or compatibility effects. A reviewer should be able to follow the evidence from
your description to the owning files.

The refresh workflow opens a checked proposal for review. It does not merge
changes automatically.

## Runtime troubleshooting

If commands cannot start, try a small read-only probe such as `git status --short`.
A failure before shell execution belongs to the harness: check its installation,
running version, and process owner using its supported recovery procedure. Prove
recovery with a fresh command before drawing conclusions about repository code.

If commands run and parsing fails, follow the [collector and fixture route](docs/data-guide.md#code-and-proof-routes).
Keep machine-specific diagnostics local; contribute portable reproduction steps.
