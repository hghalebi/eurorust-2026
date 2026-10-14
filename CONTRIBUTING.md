# Help someone make a better conference choice

A contribution can be one corrected detail, a preparation tip, a useful
question, or a clear account of what you learned. Start with the smallest
improvement you can explain and support with a source.

## Where to contribute

| You can help with… | Start here | What to include |
| --- | --- | --- |
| An incorrect or conflicting detail | [Correction guide](community/corrections/README.md) | Session or page, official source, proposed correction, and when you checked |
| A useful route through the conference | [Learning paths](community/paths/README.md) | Who it helps, goals, availability assumptions, session choices, and alternatives |
| A preparation or session note | [Session note template](templates/session.md) | Source attribution, participation context, useful questions or observations, and a next step |
| A translation or clearer explanation | [Planning guide](docs/planning-guide.md) or [agent prompt](prompts/choose-my-path.md) | Language, translator, original page, and working links |
| A summary, topic, or prerequisite | [Session annotations](community/annotations/sessions.json) | Original wording and official source; maintainers can help with the structured fields |
| A speaker's work or public account | [Speaker directory](resources/speakers/README.md) | Profile, resource, evidence connecting it to the speaker, and when you checked |
| An optional code example or tool improvement | [Maintenance guide](docs/maintenance.md) | The learning goal or failure, reproducible steps, relevant checks, and limits |

## Share an issue or edit a page

You can [open an issue](https://github.com/hghalebi/eurorust-2026/issues/new/choose)
with the note or correction in plain language. For a written change, edit the
relevant Markdown page or copy a template and propose a pull request. You do not
need to install the local tools to contribute content.

A learning path can be a Markdown table. JSON is optional for paths intended for
structured reuse. If you are unsure where a correction belongs, include the
session's official URL and the page you were reading; maintainers can route it.

## Make your change easy to trust

Write summaries in your own words and link the official session. Distinguish
source facts, community interpretation, and your own observations. Record
preparation, attendance, and watching a recording as different contexts. Mark
missing information as unknown; use `null` in structured data where appropriate.

For annotations, include the author, method, source URL, checked date, and source
hash. You can propose the wording and source in an issue if you need help with
those fields. Maintainers apply annotation changes through a data refresh.
Generated schedules and exports are rebuilt from their inputs, so report the
underlying correction instead of relying on a manual edit to a generated view.

Share only material you have authority to publish. Keep personal participant
profiles, source caches, credentials, attendee contact lists, and unlicensed
slide decks out of public changes. Follow the
[source policy](docs/source-and-reuse-policy.md) and
[community conduct](CODE_OF_CONDUCT.md).

## Speaker profile corrections

A speaker profile can begin with one useful resource. Link a project, book,
article, talk, publication, or public professional account and explain what an
attendee can learn from it. Establish the connection through the speaker's own
site, an organizer, a publisher, an institution, or a credited contribution.
A matching name alone is insufficient.

Identify the actual relationship: author, coauthor, maintainer, contributor,
presenter, or instructor. Label preprints, upcoming talks, archived projects,
and restricted access accurately. Link to original work and write a short
annotation; do not copy biographies, papers, slides, or personal contact details.

An issue with the source is enough. For a direct edit, keep the relevant
`resources/speakers/` page and its record in
[speaker-profiles.json](community/speaker-profiles.json) consistent. Preserve
the catalog speaker ID, update the checked date only for research actually
rechecked, and explain the extent of your review. Keep uncertain claims out
of the public profile until attribution is established. The
[data guide](docs/data-guide.md#speaker-research-companion) documents the fields.

## Checks

For a note or guide, check the links, names, session references, attribution,
and participation context. Read it as someone choosing a session: can they tell
what it helps with and what remains unknown? Say which checks you performed;
maintainers and repository CI handle the technical checks where needed.

For new, moved, or reclassified guides, maintainers also update the
[document inventory](docs/document-map.json) and incoming links. Rust, dataset,
and JSON itinerary changes follow the [maintenance checks](docs/maintenance.md#checks-for-maintainers).
Historical observations and immutable snapshots retain their original scope.

## Pull requests

Explain who the change helps and what becomes clearer or more useful. Include
the supporting source, affected sessions and dataset version when relevant,
checks actually performed, and any open questions. A reviewer should be able to
follow your explanation back to the material you used.

## Runtime troubleshooting

If you are working on the optional tools and commands fail, use
[maintainer troubleshooting](docs/maintenance.md#runtime-troubleshooting).
Content corrections and notes can still be proposed through an issue.
