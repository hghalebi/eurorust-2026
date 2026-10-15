# Share something useful

A correction, preparation tip, question, or session note can help another
attendee. [Open an issue](https://github.com/hghalebi/eurorust-2026/issues/new/choose)
with a source, or edit a page and propose a pull request. No local tools are needed.

## Where to contribute

| Contribution | Start here | Include |
| --- | --- | --- |
| Incorrect detail | [Corrections](community/corrections/README.md) | Page or session, source, correction, checked date |
| Time, venue, or room | [Venue guide](docs/time-and-place.md) | Official assignment, session ID, source date; preserve unknowns |
| Learning path | [Example paths](community/paths/README.md) | Audience, goals, availability, choices, alternatives |
| Preparation or session note | [Note template](templates/session.md) | Sources, participation context, observation or question, next step |
| Translation or clearer wording | [Planning guide](docs/planning-guide.md) or [agent prompt](prompts/choose-my-path.md) | Language, translator, original page, working links |
| Summary, topic, or prerequisite | [Session annotations](community/annotations/sessions.json) | Original wording and official source |
| Speaker resource or account | [Speaker directory](resources/speakers/README.md) | Resource, identity evidence, checked date |
| Related talk or learning resource | [Session resources](resources/related-talks/README.md) | Session ID, original source, specific connection, format, access, checked date |
| Conference or missing archive | [Conference directory](resources/conferences/README.md) | Series, edition, official programme, material links, known gaps |
| Code example or tool change | [Maintenance guide](docs/maintenance.md) | Goal or failure, reproduction steps, checks, limits |

Unsure where to edit? Include the official URL and the page you were reading
in an issue. Maintainers can route it.

## Make the evidence clear

Use your own words. Separate source facts, interpretation, and observations.
State whether you prepared, attended, or watched a recording. Mark unknowns;
use `null` in structured data.

Annotations need an author, method, source URL, checked date, and source hash.
Maintainers can help and apply changes through a data refresh. Correct the
owning source: manual edits to generated exports will be replaced.

Publish only material you may share. Keep personal participant profiles,
contact lists, caches, credentials, and unlicensed slide decks local. Follow
the [source policy](docs/source-and-reuse-policy.md) and
[code of conduct](CODE_OF_CONDUCT.md).

## Speaker profile corrections

Link a useful work or public professional account. Connect it to the speaker
through their site, an organizer, publisher, institution, or credited project
contribution. A matching name is insufficient.

State the role: author, coauthor, maintainer, contributor, presenter, or
instructor. Label preprints, future talks, archived projects, and restricted
access. Link original work; do not copy biographies, papers, slides, or private
contact details.

For direct edits, keep the profile page and
[speaker-profiles.json](community/speaker-profiles.json) consistent. Preserve
speaker IDs. Change a research date only after rechecking the sources, and state
what you reviewed. Omit uncertain claims. The
[data guide](docs/data-guide.md#speaker-research-companion) explains the format.

## Learning resources and archives

An issue with a source is enough. Explain what a reader can learn and which
session or topic it supports. Prefer the author, organizer, project, publisher,
or research institution. Do not force a book or paper into an unrelated topic.

For direct edits, update `community/session-resources/<session-id>.json` and
regenerate the views using the [learning guide](docs/learning-resources.md).
Keep evidence, access limits, review dates, and the distinction between event
dates and publication dates. Mark future appearances as announced; a shared
title alone does not prove identical content.

For archives, update the records under `community/conference-archive/` and their
conference pages. Report programme coverage separately from recordings. Keep
missing, blocked, and unknown material visible. Link to original content rather
than copying slides, transcripts, books, or full abstracts.

## Events and ongoing sources

Suggest an event, podcast, newsletter, blog, or regional source through an issue.
Link the organizer or publisher, explain who it helps, and give your check date.

For direct edits, update [rust-discovery.json](community/rust-discovery.json),
the matching [reading page](resources/stay-current/README.md), and
[feeds.opml](resources/stay-current/feeds.opml) when a checked feed changes.
Use stable IDs. Keep tentative dates and unknown venues explicit. A page for an
older edition does not confirm next year's event. A feed date does not establish
an event date, future publishing schedule, or recording playback.

Fetch and parse a feed before marking it checked. Mark drafts, ended series,
unavailable sources, and wider engineering coverage clearly. Change review dates
only after inspecting evidence. Link to original work and preserve its terms.

## Timing and venue corrections

Keep [logistics JSON](community/session-logistics.json) and the venue guide
consistent. Match the catalog snapshot and session IDs, retain source evidence,
and change dates only after rechecking. Stage labels do not establish physical
rooms. Follow the [format reference](docs/data-guide.md#session-logistics-check).

## Checks

Check names, links, session references, sources, and participation context.
Can a reader see what is useful and what remains unknown?

Update the [document inventory](docs/document-map.json) and incoming links for
new, moved, or reclassified guides. Use the [maintenance checks](docs/maintenance.md#checks-for-maintainers)
for Rust, dataset, or JSON itinerary changes. Preserve dated observations and
immutable snapshots.

## Pull requests

State who benefits, what changed, sources, affected sessions or snapshot,
checks performed, and open questions. Maintainers and CI handle technical checks
where needed.

## Runtime troubleshooting

For tool failures, use [maintenance troubleshooting](docs/maintenance.md#runtime-troubleshooting).
You can still submit content corrections through an issue.
