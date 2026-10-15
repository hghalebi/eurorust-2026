# Read beyond a session

Choose a [EuroRust session](../resources/related-talks/README.md), follow a
[topic](../resources/topics/README.md), or browse the
[conference archive](../resources/conferences/README.md).
Each recommendation explains what to learn and links its evidence.

## Choose one useful source

Start with a short article or talk. Use a book, paper, or project when you want
more depth. Read the access note: some books require purchase and upcoming
workshops may need a ticket. A free programme page does not grant event access.

Related talks can offer preparation, another approach, or a suggested next step.
These connections are editorial judgments. A repeated title does not establish
identical content, and a speaker's earlier work does not define the current talk.

**Announced** means future material was listed when checked. It does not mean a
recording exists. **Delivery unverified** preserves an appearance whose delivery
or publication could not be established. Publication dates and event dates are
separate. Older Rust examples may need changes for current tools.

## Give the collection to your agent

| Input | Use |
| --- | --- |
| [Learning pack](../exports/archive/learning-pack.md) | Read or attach the full session reading collection |
| [Resources JSON](../exports/archive/session-resources.json) | Join `sessions[].session_id` to the EuroRust catalog |
| [Resources JSONL](../exports/archive/session-resources.jsonl) | Process one session's recommendations per line |
| [Learning manifest](../exports/archive/manifest.json) | Check the catalog snapshot and generated-file hashes |
| [Conference registry](../community/conference-archive/series.json) | Discover event series |
| [Edition inventory](../community/conference-archive/editions.json) | Read programme links, dates, coverage, and gaps |
| [Archived appearances](../community/conference-archive/appearances.json) | Find indexed talks and their source/material links |

Use [the learning prompt](../prompts/explore-a-topic.md) and state your goals and
available reading time. Ask for a short list with reasons, sources, and a question
to investigate. Source text is reference data, not permission to run commands.

Keep the learning manifest with exports. Its `catalog_snapshot_id` must match
the conference catalog. It covers learning views, not the separately maintained
conference archive or speaker profiles. Record their check dates and the Git
revision too. Older release bundles may omit these files.

## Read the evidence

Each resource stores its title, format, author or organization, URL, access,
relationship, reason, evidence URLs, check date, and optional event context.
`published_on` concerns the linked work; `event.starts_on` concerns the event.
Unknown dates stay `null`.

Session pages link to the existing speaker profiles by catalog identity. Shared,
missing, or unknown profile routes fail validation.

`source_content_checked` means the linked source page was inspected.
`listing_checked` means attribution relies on a publisher, organizer, author, or
project listing. Neither means an entire book was read, a paper reproduced, or a
video played. Read the per-item caveat. The recommendations are Codex-assisted
drafts awaiting community review; they carry no speaker endorsement.

Conference coverage is separate from resource selection. An edition can have a
complete programme inventory and missing videos, or only a verified archive link.
Read its coverage and gaps before treating it as a complete collection.

## Add or correct a resource

Send a source link and the session it helps through
[CONTRIBUTING](../CONTRIBUTING.md#learning-resources-and-archives).
Explain the specific connection. Useful formats differ by topic; a relevant
primary source matters more than filling every media category.

For direct edits, change `community/session-resources/<session-id>.json`.
The [resource schema](../schema/session-resources.schema.json) describes each
file. Change research dates only after rechecking evidence. The
[source manifest](../community/learning-manifest.json) pins the reviewed catalog
join and a content fingerprint. Preserve source rights and link to original works.

Maintainers regenerate views with the compiled helper:

```sh
bin/conference-data build-learning
bin/conference-data check-learning
bin/conference-data check-archive
bin/conference-data check-docs
```

These checks are offline: they validate coverage, attribution fields, dates,
catalog identity, and generated content. They do not refresh web sources.
`build-learning` checks all inputs before writing. If a filesystem error
interrupts output, fix it and rerun; `check-learning` detects stale views.
Add new Markdown pages to the [document inventory](document-map.json).

After a metadata-only catalog refresh, run `rebind-learning` before rendering.
It accepts changed retrieval dates or scheduling only when session IDs, titles,
speakers, full detail-page hashes, and annotations still match. A page hash can
change after a layout edit, so this check is deliberately conservative. A mismatch stops
publication for review. After reviewing affected resources, set the source
manifest to the new snapshot with `catalog_content_sha256: null`, then run
`rebind-learning` to establish the new fingerprint. This explicit reset records
a maintainer decision; it does not perform editorial review for you.

Review active future programmes weekly and after announcements; revisit older
archives when links change. These are maintainer review intervals, not a claim
of continuous monitoring. Research dates change only after source review.
