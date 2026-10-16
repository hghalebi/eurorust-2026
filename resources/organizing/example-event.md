# Example: discuss one Rust worker failure

**Fictional event plan.** Date, venue, host, participants, and budget are unconfirmed.
This is a worked example of the [organizer handbook](README.md).

| Decision | Proposal | Reason or gap |
| --- | --- | --- |
| Audience | Developers familiar with basic Rust async tasks | Focused preparation |
| Outcome | Explain an uncertain job outcome and propose a test | Small enough for discussion |
| Format | 60-minute remote reading and design session | Joining and access arrangements unknown |
| Preparation | Tokio shutdown guide from the [worker reading list](../related-talks/talks-lessons-from-building-a-background-task-processing-library.md) | Text-first option; no recording needed |
| Roles | Facilitator, note taker, access/help contact | Unassigned; people must opt in |
| Spending | None authorized | Confirm platform and access-support costs before promising free participation |

## Proposed run sheet

| Minutes | Activity | Output |
| --- | --- | --- |
| 0–5 | Introductions, participation rules, note-sharing consent | Shared expectations |
| 5–15 | Compare understanding of shutdown | Questions and sources |
| 15–30 | Trace a job through an external effect and interrupted completion | Known and unknown states |
| 30–45 | Design an isolated experiment | Inputs, stop points, evidence, limits |
| 45–55 | Compare alternatives | One smaller next step |
| 55–60 | Agree what may be shared | Attributed notes, if consented |

Before announcing, confirm host, date/timezone, accessible joining route,
conduct/reporting process, costs, and who will test the example. Allow written
questions and asynchronous responses when bandwidth or timing limits participation.

Afterward, complete the [retrospective](../../templates/event-retrospective.md)
from observations. The outputs above remain proposals until they happen.
