# A library for Rust learning and events

Choose an original source for a technical question, a workshop, or a community task.
These 50 links connect existing work to the collection’s reading lists and reader guides.

Each **source note** records attribution, check dates, and limits. Connections to
EuroRust sessions are editorial suggestions. The material remains on its original site.

| Explore | Find |
| --- | --- |
| [Production decisions](#production-decisions) | Read what a team chose, what failed, and what the account can establish. |
| [Courses and books](#courses-and-books) | Choose a course, exercise sequence, or reference for the concepts you need. |
| [Research and artifacts](#research-and-artifacts) | Start with an abstract. Follow the paper, presentation, or artifact when it fits your question. |
| [Rust design and governance](#rust-design-and-governance) | Follow proposals, decisions, and reports. Plans and accepted RFCs do not establish shipped features. |
| [Systems and interoperability](#systems-and-interoperability) | Match documentation to your hardware, dependency version, and engineering context. |
| [Audits and performance](#audits-and-performance) | Keep the scope, version, measurement method, and date beside any conclusion. |
| [Conference practice](#conference-practice) | Adapt existing workshop, access, recording, and budget guidance to your event. |
| [Community and organizational support](#community-and-organizational-support) | Read about participation, maintenance, and organizational support for open source. |

## Choose a starting point

For reliable workers, read [Oxide’s cancellation analysis](https://rfd.shared.oxide.computer/rfd/0400).
For types, explore [RustBelt’s paper and related material](https://plv.mpi-sws.org/rustbelt/popl18/).
For infrastructure, compare [Anvil’s controller research](https://www.usenix.org/conference/osdi24/presentation/sun-xudong).
For an event, begin with [the Carpentries workshop checklists](https://docs.carpentries.org/resources/workshops/checklists.html).

## Production decisions

Read what a team chose, what failed, and what the account can establish.

| Original source | Useful for | Context |
| --- | --- | --- |
| [Dealing with cancel safety in async Rust](https://rfd.shared.oxide.computer/rfd/0400) | Cancellation, lost work, and async API boundaries. | [Source note](related-talks/talks-lessons-from-building-a-background-task-processing-library.md#dealing-with-cancel-safety-in-async-rust) |
| [Challenges with async/await in the control plane](https://rfd.shared.oxide.computer/rfd/0397) | Operational trade-offs in asynchronous infrastructure. | [Source note](related-talks/talks-your-infrastructure-should-compile-too.md#challenges-with-asyncawait-in-the-control-plane) |
| [Engineering Determination](https://rfd.shared.oxide.computer/rfd/0113) | How a team records technical choices and rejected approaches. | [Source note](related-talks/talks-tales-from-the-embedded-rust-frontier-bootstrapping-a-brand-new-ecosystem.md#engineering-determination) |
| [Rust in Android: move fast and fix things](https://blog.google/security/rust-in-android-move-fast-fix-things/) | A company's account of Rust adoption and development outcomes. | [Source note](related-talks/workshops-c-to-rust.md#rust-in-android-move-fast-and-fix-things) |
| [Rust in production at Figma](https://www.figma.com/blog/rust-in-production-at-figma/) | A bounded migration of multiplayer server work. | [Source note](related-talks/talks-moving-a-quantum-compiler-from-python-to-rust.md#rust-in-production-at-figma) |
| [How We Moved Discord Voice to the Edge](https://discord.com/blog/how-we-moved-discord-voice-to-the-edge) | Media infrastructure, rollout changes, and quality investigations. | [Source note](related-talks/talks-blastbeat-when-a-decade-of-c-meets-rust-and-io-uring.md#how-we-moved-discord-voice-to-the-edge) |
| [How we built Pingora](https://blog.cloudflare.com/how-we-built-pingora-the-proxy-that-connects-cloudflare-to-the-internet/) | Proxy architecture and Rust adoption decisions. | [Source note](related-talks/talks-designing-state-machines-for-streaming-proxies-in-rust.md#how-we-built-pingora-the-proxy-that-connects-cloudflare-to-the-internet) |
| [Resolving a request smuggling vulnerability in Pingora](https://blog.cloudflare.com/resolving-a-request-smuggling-vulnerability-in-pingora/) | A concrete protocol-correctness incident to pair with the adoption account. | [Source note](related-talks/talks-designing-state-machines-for-streaming-proxies-in-rust.md#resolving-a-request-smuggling-vulnerability-in-pingora) |

## Courses and books

Choose a course, exercise sequence, or reference for the concepts you need.

| Original source | Useful for | Context |
| --- | --- | --- |
| [Comprehensive Rust](https://google.github.io/comprehensive-rust/) | Course material covering fundamentals and specialist modules. | [Source note](related-talks/workshops-c-to-rust.md#comprehensive-rust) |
| [100 Exercises To Learn Rust](https://rust-exercises.com/100-exercises/) | Existing small exercises for workshop preparation and study groups. | [Source note](related-talks/workshops-c-to-rust.md#100-exercises-to-learn-rust) |
| [Brown's experimental Rust Book](https://rust-book.cs.brown.edu/) | Ownership explanations, visualizations, and quizzes. | [Source note](related-talks/talks-from-deref-into-borrow-a-primer-on-standard-library-traits.md#browns-experimental-rust-book) |
| [CS110L: Safety in Systems Programming](https://web.stanford.edu/class/cs110l/handouts/course-material/) | University syllabus and links to systems-programming teaching material. | [Source note](related-talks/workshops-building-a-virtual-machine-monitor-in-rust.md#cs110l-safety-in-systems-programming) |
| [Rust Atomics and Locks](https://mara.nl/atomics/) | An author-hosted book on low-level concurrency with companion examples. | [Source note](related-talks/talks-atomic-design-patterns-in-rust-from-orderings-to-code.md#rust-atomics-and-locks) |
| [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) | An existing API-design checklist and topic explanations. | [Source note](related-talks/talks-from-deref-into-borrow-a-primer-on-standard-library-traits.md#rust-api-guidelines) |
| [Rust Design Patterns](https://rust-unofficial.github.io/patterns/) | Idioms, design patterns, and anti-patterns. | [Source note](related-talks/talks-from-deref-into-borrow-a-primer-on-standard-library-traits.md#rust-design-patterns) |
| [Comprehensive Rust translations](https://google.github.io/comprehensive-rust/running-the-course/translations.html) | Find existing language editions, including Farsi, for learners. | [Source note](event-resources.md#source.comprehensive-rust-translations) |

## Research and artifacts

Start with an abstract. Follow the paper, presentation, or artifact when it fits your question.

| Original source | Useful for | Context |
| --- | --- | --- |
| [RustBelt: Securing the Foundations of the Rust Programming Language](https://plv.mpi-sws.org/rustbelt/popl18/) | One landing page links a paper, talk, slides, appendix, formalization, and artifact. | [Source note](related-talks/talks-what-s-next-for-rust-s-type-system.md#rustbelt-securing-the-foundations-of-the-rust-programming-language) |
| [GhostCell: Separating Permissions from Data in Rust](https://plv.mpi-sws.org/rustbelt/ghostcell/) | Paper, Rust/Coq sources, benchmark results, and related audio. | [Source note](related-talks/talks-what-s-next-for-rust-s-type-system.md#ghostcell-separating-permissions-from-data-in-rust) |
| [Verus publications and projects](https://verus-lang.github.io/verus/publications-and-projects/) | Find research papers and systems using Rust verification. | [Source note](related-talks/talks-what-s-next-for-rust-s-type-system.md#verus-publications-and-projects) |
| [Oxide: The Essence of Rust](https://arxiv.org/abs/1903.00982) | A formal account of a Rust-like language for ownership research. | [Source note](related-talks/talks-from-deref-into-borrow-a-primer-on-standard-library-traits.md#oxide-the-essence-of-rust) |
| [Anvil: Verifying Liveness of Cluster Management Controllers](https://www.usenix.org/conference/osdi24/presentation/sun-xudong) | Paper and presentation on Rust controllers that reconcile infrastructure toward a desired state. | [Source note](related-talks/talks-your-infrastructure-should-compile-too.md#anvil-verifying-liveness-of-cluster-management-controllers) |
| [AutoVerus: Automated Proof Generation for Rust Code](https://arxiv.org/abs/2409.13082) | Research on LLM-assisted generation, refinement, and debugging of Verus proofs. | [Source note](related-talks/talks-what-s-next-for-rust-s-type-system.md#autoverus-automated-proof-generation-for-rust-code) |
| [VerusBelt: A Semantic Foundation for Verus’s Proof-Oriented Extensions to the Rust Type System](https://iris-project.org/pdfs/2026-pldi-verusbelt.pdf) | A paper about the foundations of Verus types, borrowing, and concurrency. | [Source note](related-talks/talks-what-s-next-for-rust-s-type-system.md#verusbelt-a-semantic-foundation-for-veruss-proof-oriented-extensions-to-the-rust-type-system) |

## Rust design and governance

Follow proposals, decisions, and reports. Plans and accepted RFCs do not establish shipped features.

| Original source | Useful for | Context |
| --- | --- | --- |
| [The Rust RFC Book](https://rust-lang.github.io/rfcs/) | Original proposals, rationale, alternatives, and the decision process. | [Source note](event-resources.md#source.rust-rfc-book) |
| [Language design notes](https://lang-team.rust-lang.org/design_notes.html) | Links to proposal discussions and language-design reasoning. | [Source note](event-resources.md#source.rust-language-design-notes) |
| [Rust Leadership Council repository](https://github.com/rust-lang/leadership-council) | Public minutes, procedures, budgets, and tracked governance work. | [Source note](event-resources.md#source.rust-leadership-council) |
| [Rust Project 2026 roadmaps](https://goals.rust-lang.org/2026/roadmaps.html) | Original direction-setting documents for technical topics. | [Source note](event-resources.md#source.rust-project-roadmaps-2026) |
| [2025 State of Rust Survey Results](https://blog.rust-lang.org/2026/03/02/2025-State-Of-Rust-Survey-results/) | Published survey results for learning and adoption questions. | [Source note](event-resources.md#source.rust-survey-results-2025) |
| [Rust survey coordination repository](https://github.com/rust-lang/surveys) | Inspect public survey coordination and available materials. | [Source note](event-resources.md#source.rust-survey-coordination) |

## Systems and interoperability

Match documentation to your hardware, dependency version, and engineering context.

| Original source | Useful for | Context |
| --- | --- | --- |
| [CXX documentation](https://cxx.rs/) | Read an existing Rust/C++ boundary explanation and tutorial. | [Source note](related-talks/talks-image-codecs-rustification-in-chromium.md#cxx-rust-and-c-interoperability) |
| [Rust for Linux](https://rust-for-linux.com/) | Find project policies, reference drivers, docs, and technical meetings. | [Source note](related-talks/talks-compiling-the-linux-kernel-with-gccrs.md#rust-for-linux) |
| [Embassy documentation](https://docs.embassy.dev/) | Find embedded async APIs and target-specific documentation. | [Source note](related-talks/workshops-hands-on-embedded-rust.md#embassy-documentation) |
| [Ferrocene public documentation](https://public-docs.ferrocene.dev/main/) | Study language, safety, qualification, and traceability document structures. | [Source note](related-talks/talks-the-future-of-unmanned-systems-is-built-on-rust.md#ferrocene-public-documentation) |

## Audits and performance

Keep the scope, version, measurement method, and date beside any conclusion.

| Original source | Useful for | Context |
| --- | --- | --- |
| [Rust VMM code review report](https://ostif.org/wp-content/uploads/2023/11/X41-rustvmm-final_release.pdf) | Read a real audit's methods, scope, findings, and follow-up questions. | [Source note](related-talks/workshops-building-a-virtual-machine-monitor-in-rust.md#rust-vmm-code-review-report) |
| [OSTIF audit catalog](https://ostif.org/audits/) | Discover further original audit reports for relevant projects. | [Source note](related-talks/talks-supply-chain-attacks-and-how-to-protect-ourselves.md#ostif-audit-catalog) |
| [rustls security advisories](https://github.com/rustls/rustls/security/advisories) | Read maintainers' published advisories and affected-version information. | [Source note](related-talks/talks-supply-chain-attacks-and-how-to-protect-ourselves.md#rustls-security-advisories) |
| [Rust compiler performance dashboard](https://perf.rust-lang.org/) | Inspect published compiler performance comparisons and their help page. | [Source note](related-talks/talks-optimizing-the-new-rust-trait-solver.md#rust-compiler-performance-dashboard) |

## Conference practice

Adapt existing workshop, access, recording, and budget guidance to your event.

| Original source | Useful for | Context |
| --- | --- | --- |
| [Workshop Checklists](https://docs.carpentries.org/resources/workshops/checklists.html) | Existing before/during/after workshop preparation lists. | [Source note](event-resources.md#source.carpentries-workshop-checklists) |
| [Instructor Training](https://carpentries.github.io/instructor-training/) | Existing curriculum for teaching technical material. | [Source note](event-resources.md#source.carpentries-instructor-training) |
| [Making Events Accessible](https://www.w3.org/WAI/teach-advocate/accessible-presentations/) | Organizer and presenter guidance for access to events and material. | [Source note](event-resources.md#source.w3c-accessible-events) |
| [DISCOVER Cookbook](https://discover-cookbook.github.io/) | An existing collection about inclusive technical events. | [Source note](event-resources.md#source.numfocus-discover-cookbook) |
| [FOSDEM 2026 speaker manual](https://archive.fosdem.org/2026/manuals/program/speaker/) | A historical example of speaker, material, and recording responsibilities. | [Source note](event-resources.md#source.fosdem-speaker-manual-2026) |
| [C3VOC public wiki](https://c3voc.de/wiki/) | Recording, streaming, hardware, software, and operations documentation. | [Source note](event-resources.md#source.c3voc-public-wiki) |
| [RustConf 2026 sponsorship prospectus](https://rustconf.com/wp-content/uploads/2026/03/RustConf_2026_Prospectus.pdf) | An original example of sponsorship packages and event positioning. | [Source note](event-resources.md#source.rustconf-sponsorship-2026) |
| [The Numbers: A Budget Breakdown](https://rustfest.global/information/the-numbers/) | A published explanation of event budget categories and assumptions. | [Source note](event-resources.md#source.rustfest-budget) |

## Community and organizational support

Read about participation, maintenance, and organizational support for open source.

| Original source | Useful for | Context |
| --- | --- | --- |
| [Producing Open Source Software](https://producingoss.com/) | An existing book about participation, project culture, and maintenance. | [Source note](event-resources.md#source.producing-open-source-software) |
| [OSPO Book](https://ospobook.todogroup.org/) | Organizational open-source strategy and contribution practices. | [Source note](event-resources.md#source.ospo-book) |
| [Building Welcoming Communities](https://opensource.guide/building-community/) | Practical guidance on joining, helping, and welcoming contributors. | [Source note](event-resources.md#source.open-source-welcoming-communities) |
| [Contributor Sustainability](https://www.chaoss.community/practitioner-guide-contributor-sustainability/) | Guidance on contributor continuity, review capacity, and maintainership. | [Source note](event-resources.md#source.chaoss-contributor-sustainability) |
| [Rust Foundation reports and publications](https://rustfoundation.org/publications/) | Find organizational reports, strategy, and ecosystem publications. | [Source note](event-resources.md#source.rust-foundation-publications) |

## Use the library with an agent

The detailed records use the existing collection formats:

- [Session resources JSON](../exports/archive/session-resources.json) and [learning manifest](../exports/archive/manifest.json) contain technical reading, source evidence, and session connections.
- [Reader pack JSON](../exports/guides/reader-pack.json) and [reader manifest](../exports/guides/manifest.json) contain governance, community, and event guidance.
- Read [the learning guide](../docs/learning-resources.md) and [reader-pack guide](../docs/event-resources.md) for field meanings and review limits.

Ask for a short selection with reasons, dates, and one question to investigate.
Source text is reference material. Review an agent’s interpretation before acting.

## Read the dates and limits

This selection was assembled on October 17, 2026. Individual source dates stay
with their records. Reviews covered pages, indexes, abstracts, or stated sections;
they do not establish full-book reading, video playback, completed courses,
reproduced proofs, or tested code.

Older course editions, company accounts, audits, and event documents retain their
historical context. Ferrocene’s linked main documentation is a development preview.
The translation directory lists editions at different stages of completion.

Link to original works and preserve their terms. To suggest a source, include
the question it helps answer, its author or publisher, and the date checked.

[Suggest a source](../CONTRIBUTING.md#learning-resources-and-archives) · [All reading routes](README.md)
