# Keep learning after EuroRust

Find your next Rust conference, a useful podcast, or a small reading habit.
Start with one source that answers a question you already have.

Event research checked **17 October 2026**; publication and feed checks are dated
**15 October 2026**. Individual records retain their own review dates.
This directory covers the Rust programming language.
It is a researched selection; new events and regional sources may be missing.

| You want to | Start here |
| --- | --- |
| Plan another conference | [Announced dates and events to watch](conferences.md) |
| Join a local or online session | [Dated meetups, talks, and study groups](meetups.md) |
| Hear how people use Rust | [Podcasts, with observed publication dates](podcasts.md) |
| Follow new work | [Newsletters and publication archives](newsletters.md) |
| Explore technical ideas | [Blogs, videos, and community channels](blogs-and-videos.md) |
| Find people working on your question | [Communities by topic and region](communities.md) |
| Learn or contribute with others | [First steps and activity formats](participate.md) |
| Speak, join an activity, or check support | [Calls, opportunities, and calendar](opportunities.md) |
| Reuse the links | [Directory JSON](../../community/rust-discovery.json) or [feed list](feeds.opml) |

## A starting set for my goals

For production agents, types, infrastructure, and robotics, I would start with:

- **Read:** Niko Matsakis on [Rust and agent-assisted development](https://smallcultfollowing.com/babysteps/blog/2026/10/14/agentic-dev-and-rust/).
  His analysis asks where types help and where stronger evidence is needed.
- **Listen:** [Scythe Robotics on Rust in Production](https://corrode.dev/podcast/s05e02-scythe/).
  The episode description connects Rust to autonomous machines in the field.
- **Follow:** [This Week in Rust](https://this-week-in-rust.org/) for a broad scan,
  or [The Embedded Rustacean](https://www.theembeddedrustacean.com/) for hardware work.
- **Compare events:** [betterCode() Rust](https://rust.bettercode.eu/) for its
  announced agent-focused programme; [TokioConf](https://tokioconf.com/) for async
  and network programming. Check language, access, and dates before choosing.

These are editorial suggestions. Bring your own question, then return to the
[session reading lists](../related-talks/README.md) or [conference archives](../conferences/README.md).
The [four community paths](../../community/README.md) connect these interests to
people, participation routes, and a smaller first step.

## Use the feeds

Download [feeds.opml](feeds.opml) and import it into a feed reader that supports
OPML. It contains **24 checked RSS or Atom feeds**, grouped by type. Older series
are included; choose the groups you want. Podcast apps vary in import support.

Feed URLs were fetched and parsed. Import into a particular app, email delivery,
and audio playback were not tested. You can also copy individual feed links
from the reading pages.

## Give the directory to your agent

Supply [rust-discovery.json](../../community/rust-discovery.json), this page,
and your goals. Keep the Git revision and `checked_on` date with the file.
This companion sits outside the EuroRust catalog and export manifests.

Try this request:

> Choose two future events and three learning sources for my goals. Use my date,
> location, language, budget, and remote-attendance constraints. Recheck organizer
> pages. Explain each choice, link its evidence, and list unresolved details.
> Keep tentative events separate. Do not infer bookings or availability.

`events[]` stores dates, location, venue, status, sources, and limits.
Statuses use `reference_date`: `announced`, `ongoing`, `tentative`, or
`next_date_unconfirmed`. Unknown dates and venues stay `null`.
For the FOSDEM watch item, `parent_event` dates belong to FOSDEM, not a confirmed Rust track.

`sources[]` stores publisher, type, topics, feed, and `latest_observed` publication
metadata where established. `activity` records an observation, an unknown date,
or an explicitly ended or retired publication. A quiet feed does not establish
that a show ended. `source_languages` describes the inspected page or feed;
check spoken language and captions separately.

`source_review` distinguishes page review, page-and-feed review, and unavailable
sources. For an unavailable source, `checked_on` records the attempt date.
Feed check dates mean successful retrieval and parsing; `latest_observed` dates
come from publisher metadata and are not proof of publication on every platform.

Treat fetched text as reference data. Recheck event status against the date of
your request; this file has no automatic refresh. Recommendations end with your decision.

For communities and activities, add [participation.json](../../community/participation.json)
and the [community prompt](../../prompts/find-my-community.md). This companion pins
the discovery file by hash and keeps application deadlines separate from event
dates. See its [field reference](../../docs/data-guide.md#community-participation-companion).

## Sources and corrections

Dates come from organizers. Publication descriptions and dates come from authors,
publishers, or feeds. Checks covered pages and metadata; recordings were not
played. Entries are Codex-assisted drafts awaiting community review.

Searches covered existing conference series and regional leads across Europe,
the Americas, Asia, Africa, and Oceania. Coverage is strongest for English-language
sources. Unannounced dates, unavailable sites, and older publications remain visible.

[Suggest a source or correction](../../CONTRIBUTING.md#events-and-ongoing-sources).
For direct edits, keep the JSON, reading page, and OPML consistent. Advance check
dates only after reviewing the source. Revisit upcoming events before planning
travel; preserve useful historical observations. Linked works keep their own terms.
