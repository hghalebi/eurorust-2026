# Example proposal: stopping a Rust worker

**Fictional planning exercise.** No submission, acceptance, attendance, or
experimental result is claimed. Replace proposed evidence with your own work.

| Field | Draft |
| --- | --- |
| Title | What happens to accepted work when a Rust worker stops? |
| Audience | Developers who have used async tasks and are adding background jobs |
| Problem | Shutdown, cancellation, and retry decisions can leave a job's outcome unclear |
| Takeaway | Identify what was accepted, what completed, and what evidence is needed before retrying |
| Format | Proposed 25-minute talk; adapt to the selected call |
| Evidence to collect | An isolated worker experiment, versions, failure timing, and observed job state |

## Draft abstract

A worker receives a job, starts an external operation, and stops before saving
the result. What can the next worker safely conclude? This proposed talk follows
that question through shutdown, restart, and retry. It compares cancellation
coordination with persisted job state, then uses a small experiment to show which
outcomes the system can establish. The goal is a checklist for reviewing worker
recovery, including the cases where the outcome remains unknown.

## Research before claiming a result

Use the [background-worker reading list](../related-talks/talks-lessons-from-building-a-background-task-processing-library.md):

| Source | Question to investigate |
| --- | --- |
| Tokio shutdown guide | How are tasks asked to stop, and how is completion awaited? |
| Apalis | Which job and shutdown behavior does the documented version provide? |
| Amazon's retry article | When can retries repeat an effect or amplify a failure? |

These sources support preparation. They do not establish durability or an
experimental result for the proposed worker.

## Proposed outline and experiment

Allow three minutes for the failure, five for shutdown coordination, eight for
the experiment, six for recovery choices, and three for limits and questions.

Test stops before external work, after its result, and before the local completion
record. Record actual outcomes and explain which conclusions need a real storage
or external-service test. Produce a captured run before promising a demo fallback.

The [TokioConf call](../stay-current/opportunities.md#tokioconf-2027-call-for-talks) is a research
lead in the dated directory. Recheck its deadline, scope, and submission terms.
Topic relevance does not imply acceptance.

Next: copy the [proposal template](../../templates/talk-proposal.md) and replace
the hypothetical evidence with your observations.
