# Follow Rust conferences and their ideas

Use the [conference directory](README.md) to find an edition, then its program,
speakers, and available material. The [coverage table](editions.md) distinguishes
indexed programs, partial records, source links, and access gaps.

## Early reference points

| Event | Evidence to start with | What remains limited |
| --- | --- | --- |
| RustCamp 2015 | The [Rust Blog retrospective](https://blog.rust-lang.org/2015/08/14/Next-year/) reports the event; [Carol Nichols's dated talk notes](https://carol-nichols.com/2015/08/01/rustcamp-talk-notes/) preserve an original contribution | Full-program recovery is incomplete; an old recording-directory link now leads to unrelated content |
| RustConf 2016 | The [official two-year retrospective](https://blog.rust-lang.org/2017/05/15/rust-at-two-years/) records the event; [Back to the Futures](https://www.youtube.com/watch?v=bcrzfivXpc4) has matched publisher metadata | Original program host failed certificate validation during research; recording playback and captions unchecked |
| RustConf 2017 | The [official schedule](https://2017.rustconf.com/schedule.html) and [program](https://2017.rustconf.com/program.html) describe the conference day | Conference talks and the preceding training day require separate coverage |

These are selected reference points, not a complete origin story. Source and
metadata checks were made on October 16, 2026. A program listing alone does not
establish delivery; a dated retrospective or speaker account can add evidence.

## Follow a technical question

The connections below are editorial reading suggestions. They do not claim that
later work derives from a particular talk or that old examples run unchanged.

| Question | Earlier material | Continue with | Comparison to make |
| --- | --- | --- | --- |
| How do async tasks become reliable workers? | Alex Crichton's [Back to the Futures](https://www.youtube.com/watch?v=bcrzfivXpc4), RustConf 2016 | [Worker recovery and shutdown resources](../related-talks/talks-lessons-from-building-a-background-task-processing-library.md) | Separate an async computation model from persistence, cancellation, and recovery responsibilities |
| How do types guide API and compiler design? | Sean Griffin's [Type System Tips for the Real World](https://2017.rustconf.com/program.html#types), RustConf 2017 | [2025 solver talk, 2026 report, and current compiler guide](../related-talks/talks-what-s-next-for-rust-s-type-system.md) | API constraints and compiler solving are related questions at different levels |
| What can infrastructure checks establish? | The [2022 architecture article and later Rust preview](../related-talks/talks-your-infrastructure-should-compile-too.md) | The same guide links Rustikon and announced RustLab 2026 material | Distinguish checked declarations from external deployment outcomes; preserve preview and future-talk status |
| What must a robot's software know about the outside world? | The [embedded ecosystem session and references](../related-talks/talks-tales-from-the-embedded-rust-frontier-bootstrapping-a-brand-new-ecosystem.md) | [Autonomous-system integration and MAVLink resources](../related-talks/workshops-rust-on-the-field-integrating-real-autonomous-systems.md) | Compare protocol assumptions, hardware dependencies, and evidence from a real or simulated system |

For other subjects, start with the [topic index](../topics/README.md). Language,
library, and hardware versions matter. Record the version described by a source
and a separate date if you actually retest it.

## Example: research a types talk

**Research exercise.** Read the 2017 abstract, then the 2025 solver abstract and
the linked compiler documentation. Write down which question each source answers,
what evidence you inspected, and what remains unresolved. Find the speaker's own
work before merging identities or attributing a technical claim.

Your result can be a short brief: “These sources discuss API-level constraints
and compiler-level solving. I want to explain the boundary between the two.”
That is a proposed angle, not proof of novelty or a report that all talks were watched.

## Cite and help preserve an edition

Record event/edition, original title and speaker credit, session URL, material
publisher, event date, publication date if known, and check date. Preserve original
language and credit translations. Distinguish a repeated appearance from a
confirmed identical talk. Keep original and archived URLs with their capture dates.

[Contribute an archive source](../../CONTRIBUTING.md#learning-resources-and-archives)
· [Prepare a proposal](../speaking/README.md) · [Find your route](../../docs/find-your-route.md)
