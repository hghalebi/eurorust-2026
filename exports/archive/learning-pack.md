# EuroRust learning resources

Catalog snapshot: `eurorust-2026-71f2ee4aea59a1589532`. Format: `1.0.0`.

Curated source-checked drafts, with community review pending. Use source dates and
access notes; a listing does not prove attendance or a published recording.

## Beyond p99: Observing Async Rust

Session ID: `workshops-beyond-p99-observing-async-rust`. [Official source](https://eurorust.eu/workshops/beyond-p99-observing-async-rust/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [Announcing Tokio Console 0.1](https://tokio.rs/blog/2021-12-announcing-tokio-console)

Article · Tokio team · open access

**Preparation.** Explains task states and why aggregate logs hide async runtime behavior.

Publication date: 2021-12-17.

Historical introduction; use current project documentation for setup.

Source page checked: 2026-10-15. Evidence: [tokio.rs](https://tokio.rs/blog/2021-12-announcing-tokio-console).

### [Getting started with Tracing](https://tokio.rs/tokio/topics/tracing)

Documentation · Tokio contributors · open access

**Preparation.** Introduces spans and events for following one request across async tasks.

Source page checked: 2026-10-15. Evidence: [tokio.rs](https://tokio.rs/tokio/topics/tracing).

### [dial9](https://github.com/dial9-rs/dial9)

Project · dial9 contributors · open access

**Same topic.** Provides the workshop speaker’s profiling project for inspecting task scheduling and CPU activity.

Project documentation checked; no profiling run was performed.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/dial9-rs/dial9).

### [Metrique](https://github.com/awslabs/metrique)

Project · Metrique contributors · open access

**Suggested next step.** Shows how structured records retain the context behind individual slow requests.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/awslabs/metrique).

## Building a virtual machine monitor in Rust

Session ID: `workshops-building-a-virtual-machine-monitor-in-rust`. [Official source](https://eurorust.eu/workshops/building-a-virtual-machine-monitor-in-rust/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [Firecracker: Lightweight Virtualization for Serverless Applications](https://www.usenix.org/conference/nsdi20/presentation/agache)

Paper · Alexandru Agache, Marc Brooker, Andreea Florescu, Alexandra Iordache, Anthony Liguori, Rolf Neugebauer, Phil Piwonka, Diana-Maria Popa · open access

**Same topic.** Explains the device, isolation and overhead choices behind a production microVM monitor.

[USENIX NSDI 2020](https://www.usenix.org/conference/nsdi20/presentation/agache) · Published.

The publisher page includes the paper, slides and a recording; results describe the 2020 system.

Listing checked: 2026-10-15. Evidence: [www.usenix.org](https://www.usenix.org/conference/nsdi20/presentation/agache).

### [The KVM API](https://docs.kernel.org/virt/kvm/api.html)

Documentation · Linux kernel contributors · open access

**Preparation.** Defines the VM, vCPU and memory interfaces used when building a Linux VMM.

Source page checked: 2026-10-15. Evidence: [docs.kernel.org](https://docs.kernel.org/virt/kvm/api.html).

### [rust-vmm reference VMM](https://github.com/rust-vmm/vmm-reference)

Project · rust-vmm contributors · open access

**Suggested next step.** Shows how reusable Rust virtualization components fit together in a small VMM.

Reference implementation; no build or guest boot was performed.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/rust-vmm/vmm-reference).

## C→Rust, Beyond the Basics

Session ID: `workshops-c-to-rust`. [Official source](https://eurorust.eu/workshops/c-to-rust/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [The C to Rust Migration Book](https://mainmatter.com/c-to-rust-migration-book/)

Book · Jonas Kruckenberg · open access

**Same topic.** Covers mixed-language APIs, ownership across FFI and incremental migration exercises.

Overview and contents checked; exercise solutions were not run.

Listing checked: 2026-10-15. Evidence: [mainmatter.com](https://mainmatter.com/c-to-rust-migration-book/).

### [Foreign Function Interface — The Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html)

Documentation · Rust Project contributors · open access

**Preparation.** Explains calling conventions, callbacks and unsafe boundaries when Rust and C share data.

Source page checked: 2026-10-15. Evidence: [doc.rust-lang.org](https://doc.rust-lang.org/nomicon/ffi.html).

### [C2Rust](https://github.com/immunant/c2rust)

Project · Immunant and C2Rust contributors · open access

**Alternative approach.** Shows automated C translation as a starting point for comparison with manual migration.

The project states that translated output remains unsafe and needs further migration work.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/immunant/c2rust).

### [Rust and ThreadX](https://ferrous-systems.com/blog/rust-and-threadx/)

Article · Jonathan Pallant · open access

**Suggested next step.** Provides an embedded example of combining Rust with an existing C real-time operating system.

Publication date: 2023-12-07.

Historical integration example; certification statements are tied to its publication date.

Source page checked: 2026-10-15. Evidence: [ferrous-systems.com](https://ferrous-systems.com/blog/rust-and-threadx/).

## Containers Are Dead: Long Live WebAssembly

Session ID: `workshops-containers-are-dead`. [Official source](https://eurorust.eu/workshops/containers-are-dead/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [Containers Are Dead: Long Live WebAssembly](https://eurorust.eu/2025/workshops/long-live-webassembly/)

Workshop · Jonas Kruckenberg · public source page

**Repeat appearance.** Lets readers compare the previous syllabus for Rust HTTP services, WASM deployment and diagnostics.

[EuroRust 2025](https://eurorust.eu/2025/workshops/long-live-webassembly/) · 2025-10-08 · Delivery unverified.

Earlier programme with the same title and mentor; no recording or identical 2026 syllabus is claimed.

Listing checked: 2026-10-15. Evidence: [eurorust.eu](https://eurorust.eu/2025/workshops/long-live-webassembly/).

### [The WebAssembly Component Model](https://component-model.bytecodealliance.org/)

Book · Bytecode Alliance contributors · open access

**Preparation.** Introduces components, interfaces and WASI before comparing service deployment models.

Listing checked: 2026-10-15. Evidence: [component-model.bytecodealliance.org](https://component-model.bytecodealliance.org/).

### [Wasmtime security model](https://docs.wasmtime.dev/security.html)

Documentation · Wasmtime contributors · open access

**Same topic.** Explains sandboxing and capability-based filesystem access so isolation claims can be assessed.

Source page checked: 2026-10-15. Evidence: [docs.wasmtime.dev](https://docs.wasmtime.dev/security.html).

### [Bringing the Web up to Speed with WebAssembly](https://css.csail.mit.edu/6.5660/2024/readings/wasm.pdf)

Paper · Andreas Haas, Andreas Rossberg, Derek L. Schuff, Ben L. Titzer, Michael Holman, Dan Gohman, Luke Wagner, Alon Zakai, JF Bastien · open access

**Preparation.** Explains WebAssembly’s portable execution model, validation and original design constraints.

2017 research paper; its original runtime results do not describe current WASI deployments.

Listing checked: 2026-10-15. Evidence: [css.csail.mit.edu](https://css.csail.mit.edu/6.5660/2024/readings/wasm.pdf), [webassembly.org](https://webassembly.org/community/events/).

## Hands-On Embedded Rust

Session ID: `workshops-hands-on-embedded-rust`. [Official source](https://eurorust.eu/workshops/hands-on-embedded-rust/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [The Embedded Rust Book](https://docs.rust-embedded.org/book/)

Book · Rust Embedded Devices Working Group · open access

**Preparation.** Introduces no\_std programs, peripherals and the embedded development workflow.

Listing checked: 2026-10-15. Evidence: [docs.rust-embedded.org](https://docs.rust-embedded.org/book/).

### [Embassy Book](https://embassy.dev/book/)

Book · Embassy contributors · open access

**Same topic.** Explains the async executor and hardware abstractions used in the workshop.

Listing checked: 2026-10-15. Evidence: [embassy.dev](https://embassy.dev/book/).

### [Introductory Presentation for Embedded Rust](https://github.com/domenicquirl/rust-embedded-intro)

Project · Domenic Quirl · open access

**Preparation.** Offers introductory material from one of the workshop mentors.

Repository description checked; exercises and hardware compatibility were not tested.

Listing checked: 2026-10-15. Evidence: [github.com](https://github.com/domenicquirl/rust-embedded-intro).

## Programming a Line Follower Robot

Session ID: `workshops-programming-a-line-follower-robot`. [Official source](https://eurorust.eu/workshops/programming-a-line-follower-robot/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed.

### [Programming a line follower robot](https://rustlab.it/talks/programming-a-line-follower-robot)

Workshop · Massimiliano Mantione, Michele Mantione · public source page

**Repeat appearance.** Shows an earlier workshop by the same mentors on simulated robots, PID control and telemetry.

[RustLab 2025](https://rustlab.it/talks/programming-a-line-follower-robot) · 2025-11-02 · Delivery unverified.

Date comes from the page’s embedded event data; current site branding says 2026. Delivery and recording are unverified.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/programming-a-line-follower-robot).

### [Motion Control with Velocity Inputs, Part 2](https://modernrobotics.northwestern.edu/nu-gm-book-resource/11-3-motion-control-with-velocity-inputs-part-2-of-3/)

Video · Kevin M. Lynch, Frank C. Park · open access

**Preparation.** Explains how integral feedback reduces tracking error before tuning a line follower.

University description and transcript checked; video playback was not tested.

Source page checked: 2026-10-15. Evidence: [modernrobotics.northwestern.edu](https://modernrobotics.northwestern.edu/nu-gm-book-resource/11-3-motion-control-with-velocity-inputs-part-2-of-3/).

### [RustLab 2023 Robotic Sumo workshop code](https://github.com/massimiliano-mantione/rustlab-2023-robotic-sumo)

Project · Massimiliano Mantione · open access

**Preparation.** Provides earlier robot-control exercises from a workshop mentor.

Earlier hardware-based workshop; this EuroRust workshop uses simulated line followers.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/massimiliano-mantione/rustlab-2023-robotic-sumo).

### [Modern Robotics: Mechanics, Planning, and Control](https://hades.mech.northwestern.edu/index.php/Modern_Robotics)

Book · Kevin M. Lynch, Frank C. Park · open access

**Suggested next step.** Develops the motion and feedback-control foundations behind more advanced robot behavior.

The authors provide a free preprint; the printed book is sold separately.

Listing checked: 2026-10-15. Evidence: [hades.mech.northwestern.edu](https://hades.mech.northwestern.edu/index.php/Modern_Robotics).

### [Accurate Robot Simulations with Bevy and WASM](https://rustlab.it/talks/accurate-robot-simulations-with-bevy-and-wasm-workshop)

Workshop · Massimiliano Mantione, Michele Mantione · registration

**Suggested next step.** Extends line-follower control into deterministic simulation, telemetry and separate WASM robot logic.

[RustLab 2026](https://rustlab.it/talks/accurate-robot-simulations-with-bevy-and-wasm-workshop) · 2026-11-01 · Announced.

Announced workshop on November 1, 2026; no recording is claimed.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/accurate-robot-simulations-with-bevy-and-wasm-workshop).

## Rust in Action - Applications of Computer Vision and Control

Session ID: `workshops-rust-in-action`. [Official source](https://eurorust.eu/workshops/rust-in-action/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [Modern Robotics: Mechanics, Planning, and Control](https://hades.mech.northwestern.edu/index.php/Modern_Robotics)

Book · Kevin M. Lynch, Frank C. Park · open access

**Preparation.** Introduces the coordinate frames, motion and feedback control needed to connect perception to movement.

The authors provide a free preprint; examples are not specific to the workshop SDK.

Listing checked: 2026-10-15. Evidence: [hades.mech.northwestern.edu](https://hades.mech.northwestern.edu/index.php/Modern_Robotics).

### [Motion Control with Velocity Inputs, Part 2](https://modernrobotics.northwestern.edu/nu-gm-book-resource/11-3-motion-control-with-velocity-inputs-part-2-of-3/)

Video · Kevin M. Lynch, Frank C. Park · open access

**Preparation.** Gives a focused introduction to tracking a target with feedforward and PI feedback.

University description and transcript checked; video playback was not tested.

Source page checked: 2026-10-15. Evidence: [modernrobotics.northwestern.edu](https://modernrobotics.northwestern.edu/nu-gm-book-resource/11-3-motion-control-with-velocity-inputs-part-2-of-3/).

### [OpenCV tutorials](https://docs.opencv.org/4.x/d9/df8/tutorial_root.html)

Documentation · OpenCV contributors · open access

**Suggested next step.** Provides camera calibration and image-processing background for interpreting vehicle camera data.

Independent background material; the workshop page does not say that its SDK uses OpenCV.

Listing checked: 2026-10-15. Evidence: [docs.opencv.org](https://docs.opencv.org/4.x/d9/df8/tutorial_root.html).

## Rust on the Field: Integrating Real Autonomous Systems

Session ID: `workshops-rust-on-the-field-integrating-real-autonomous-systems`. [Official source](https://eurorust.eu/workshops/rust-on-the-field-integrating-real-autonomous-systems/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [MAVLink Developer Guide](https://mavlink.io/en/)

Documentation · MAVLink contributors · open access

**Preparation.** Explains the message, heartbeat and routing concepts behind the workshop’s MAVLink connector.

Source page checked: 2026-10-15. Evidence: [mavlink.io](https://mavlink.io/en/).

### [rust-mavlink](https://github.com/mavlink/rust-mavlink)

Project · rust-mavlink contributors · open access

**Same topic.** Provides Rust bindings and examples for exchanging MAVLink messages.

A learning reference; no vehicle connection or hardware command was executed.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/mavlink/rust-mavlink).

### [Timeouts, retries, and backoff with jitter](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/)

Article · Amazon Builders’ Library · open access

**Preparation.** Explains timeout budgets, retry amplification and jitter before handling unreliable transport.

Source page checked: 2026-10-15. Evidence: [aws.amazon.com](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/).

### [Graceful Shutdown](https://tokio.rs/tokio/topics/shutdown)

Documentation · Tokio contributors · open access

**Suggested next step.** Shows how to cancel connector tasks and wait for cleanup when a service stops.

Source page checked: 2026-10-15. Evidence: [tokio.rs](https://tokio.rs/tokio/topics/shutdown).

## Move your problems to compile-time and get new capabilities out of it

Session ID: `talks-move-your-problems`. [Official source](https://eurorust.eu/talks/move-your-problems/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [Constant evaluation — The Rust Reference](https://doc.rust-lang.org/reference/const_eval.html)

Documentation · Rust Project contributors · open access

**Preparation.** Defines what Rust can evaluate during compilation and where const contexts apply.

Source page checked: 2026-10-15. Evidence: [doc.rust-lang.org](https://doc.rust-lang.org/reference/const_eval.html).

### [Const generics MVP hits beta!](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta/)

Article · The const generics project group · open access

**Preparation.** Introduces value parameters in types and explains why evaluating generic expressions at compile time needs extra rules.

Publication date: 2021-02-26.

Historical Rust 1.51 announcement. Use the current Rust Reference for present language rules.

Source page checked: 2026-10-15. Evidence: [blog.rust-lang.org](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta/).

### [Rust reflection and comptime project goal](https://goals.rust-lang.org/2026/reflection-and-comptime.html)

Documentation · Oli Scherer, Rust Project · open access

**Project evolution.** Explains the speaker’s proposed reflection work and the design questions still being explored.

A project goal and experiment; acceptance does not mean the feature is stable Rust.

Source page checked: 2026-10-15. Evidence: [goals.rust-lang.org](https://goals.rust-lang.org/2026/reflection-and-comptime.html).

### [Misusing Const for Fn and Profit](https://eurorust.eu/2025/talks/misusing-const/)

Talk · Tris Oaten · public source page

**Same topic.** Offers an earlier conference perspective on moving useful work into Rust’s compile-time evaluation.

[EuroRust 2025](https://eurorust.eu/2025/talks/misusing-const/) · Delivery unverified.

Official session description checked; delivery and recording are unverified.

Listing checked: 2026-10-15. Evidence: [eurorust.eu](https://eurorust.eu/2025/talks/misusing-const/).

## Compiling the Linux kernel with gccrs

Session ID: `talks-compiling-the-linux-kernel-with-gccrs`. [Official source](https://eurorust.eu/talks/compiling-the-linux-kernel-with-gccrs/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [Compiling the Linux Kernel with gccrs](https://rustconf2026.sched.com/event/2KHyD)

Talk · Arthur Cohen, Pierre-Emmanuel Patry · public source page

**Repeat appearance.** Lists an earlier 2026 appearance with the same title and speaker.

[RustConf 2026](https://rustconf2026.sched.com/) · 2026-09-09 · Delivery unverified.

Official timetable and speaker roster checked. Detail-page access was inconsistent; delivery and recording remain unverified.

Listing checked: 2026-10-15. Evidence: [rustconf.com](https://rustconf.com/speakers/), [rustconf2026.sched.com](https://rustconf2026.sched.com/).

### [gccrs: An alternative compiler for Rust](https://blog.rust-lang.org/2024/11/07/gccrs-an-alternative-compiler-for-rust/)

Article · Arthur Cohen · open access

**Preparation.** Explains gccrs goals and its relationship to rustc and the wider Rust Project.

Publication date: 2024-11-07.

Source page checked: 2026-10-15. Evidence: [blog.rust-lang.org](https://blog.rust-lang.org/2024/11/07/gccrs-an-alternative-compiler-for-rust/).

### [Inching ever closer to the Rust standard library with gccrs](https://2025.rustweek.org/talks/arthurcohen/)

Talk · Arthur Cohen, Pierre-Emmanuel Patry · public source page

**Project evolution.** Shows the compiler’s earlier standard-library milestone before the kernel-focused session.

[RustWeek 2025](https://2025.rustweek.org/talks/arthurcohen/) · Delivery unverified.

Official session description checked; delivery and recording are unverified.

Listing checked: 2026-10-15. Evidence: [2025.rustweek.org](https://2025.rustweek.org/talks/arthurcohen/).

### [gccrs](https://github.com/Rust-GCC/gccrs)

Project · gccrs contributors · open access

**Suggested next step.** Provides the compiler source and current development limitations behind the talk.

The project describes gccrs as experimental; the talk title is not proof that it can build the Linux kernel.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/Rust-GCC/gccrs).

## The Future of Unmanned Systems Is Built on Rust

Session ID: `talks-the-future-of-unmanned-systems-is-built-on-rust`. [Official source](https://eurorust.eu/talks/the-future-of-unmanned-systems-is-built-on-rust/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed.

### [Rust as the Foundation for Future Unmanned Systems](https://oxidizeconf.com/sessions/keynote_rust_as_the-foundation_for_future_unmanned-systems)

Talk · Sebastian Rietzscher, Sebastian Haas · public source page

**Same topic.** Provides an earlier account of MOSAIC’s architecture, Rust migration and legacy integration.

[Oxidize 2026](https://oxidizeconf.com/sessions/keynote_rust_as_the-foundation_for_future_unmanned-systems) · 2026-09-15 · Delivery unverified.

Official programme checked; different speakers from EuroRust, and no delivery or recording is verified.

Listing checked: 2026-10-15. Evidence: [oxidizeconf.com](https://oxidizeconf.com/sessions/keynote_rust_as_the-foundation_for_future_unmanned-systems).

### [Comparison of Middlewares in Edge-to-Edge and Edge-to-Cloud Communication for Distributed ROS2 Systems](https://arxiv.org/abs/2309.07496)

Paper · Jiaqiang Zhang, Xianjia Yu, Sier Ha, Jorge Pena Queralta, Tomi Westerlund · open access

**Alternative approach.** Compares communication tradeoffs under different networks for distributed robotic systems.

Publication date: 2023-09-14.

Abstract and publication record checked; experiments are specific to the paper’s ROS 2 setup, not MOSAIC.

Listing checked: 2026-10-15. Evidence: [arxiv.org](https://arxiv.org/abs/2309.07496).

### [What is Zenoh?](https://zenoh.io/docs/overview/what-is-zenoh/)

Documentation · Eclipse Zenoh contributors · open access

**Alternative approach.** Introduces another approach to moving data across distributed and constrained systems.

Comparison material; the EuroRust abstract does not say that MOSAIC uses Zenoh.

Source page checked: 2026-10-15. Evidence: [zenoh.io](https://zenoh.io/docs/overview/what-is-zenoh/).

### [Rust bindings for ROS 2](https://github.com/ros2-rust/ros2_rust)

Project · ROS 2 Rust contributors · open access

**Alternative approach.** Shows how Rust can participate in an existing robotics middleware ecosystem.

Independent alternative; no claim that the speakers use this project.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/ros2-rust/ros2_rust).

### [Bridging robotics and systems programming: why Copper is a game changer](https://rustlab.it/talks/bridging-robotics-and-systems-programming-why-copper-is-a-game-changer)

Talk · Guillaume Binet · registration

**Alternative approach.** Offers a future comparison of Rust runtime design for robot integration and concurrent execution.

[RustLab 2026](https://rustlab.it/talks/bridging-robotics-and-systems-programming-why-copper-is-a-game-changer) · 2026-11-02 · Announced.

Announced session on November 2, 2026; the title is the organizer’s wording, not a repository endorsement.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/bridging-robotics-and-systems-programming-why-copper-is-a-game-changer).

## Optimizing rustc's Next Trait Solver

Session ID: `talks-optimizing-the-new-rust-trait-solver`. [Official source](https://eurorust.eu/talks/optimizing-the-new-rust-trait-solver/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [Next-gen trait solving](https://rustc-dev-guide.rust-lang.org/solve/trait-solving.html)

Documentation · Rust compiler contributors · open access

**Preparation.** Introduces goals, responses and the structure of rustc’s next trait solver.

Source page checked: 2026-10-15. Evidence: [rustc-dev-guide.rust-lang.org](https://rustc-dev-guide.rust-lang.org/solve/trait-solving.html).

### [-Znext-solver: what, why, and when](https://eurorust.eu/2025/talks/znext-solver/)

Talk · lcnr · public source page

**Project evolution.** Explains the correctness and performance problems that motivated replacing the previous solver.

[EuroRust 2025](https://eurorust.eu/2025/talks/znext-solver/) · 2025-10-09 · Delivery unverified.

Official session description checked; delivery and recording are unverified.

Listing checked: 2026-10-15. Evidence: [eurorust.eu](https://eurorust.eu/2025/talks/znext-solver/).

### [The Chalk Book](https://rust-lang.github.io/chalk/book/)

Book · Chalk contributors · open access

**Preparation.** Explains logical goals, unification and canonical queries behind Rust trait reasoning.

Chalk is useful background; it is not the implementation of rustc’s next trait solver.

Listing checked: 2026-10-15. Evidence: [rust-lang.github.io](https://rust-lang.github.io/chalk/book/).

## Shipping Temporal: How the Rust Ecosystem Built Chrome’s Newest Web API

Session ID: `talks-shipping-temporal-how-the-rust-ecosystem-built-chrome-s-newest-web-api`. [Official source](https://eurorust.eu/talks/shipping-temporal-how-the-rust-ecosystem-built-chrome-s-newest-web-api/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. Temporal here is JavaScript’s date-and-time API, not the workflow engine. No verified future conference match is included in this pass.

### [Temporal\_rs is here! The datetime library powering Temporal in Boa, Kiesel, and V8](https://boajs.dev/blog/2025/09/24/temporal-release)

Article · Temporal\_rs developers · open access

**Project evolution.** Explains the library’s design, FFI boundary and adoption by JavaScript engines.

Publication date: 2025-09-24.

Historical release article; its browser and standards forecasts describe September 2025.

Source page checked: 2026-10-15. Evidence: [boajs.dev](https://boajs.dev/blog/2025/09/24/temporal-release).

### [Implementing Temporal, the new date/time API for JavaScript (and Rust!)](https://boajs.dev/blog/2025/06/15/temporal-impl-1)

Article · Boa developers · open access

**Preparation.** Explains implementation choices behind calendar and time-zone behavior in Rust.

Publication date: 2025-06-15.

Source page checked: 2026-10-15. Evidence: [boajs.dev](https://boajs.dev/blog/2025/06/15/temporal-impl-1).

### [Temporal documentation](https://tc39.es/proposal-temporal/docs/)

Documentation · TC39 Temporal contributors · open access

**Preparation.** Defines the date-and-time API that the Rust implementation must expose.

Source page checked: 2026-10-15. Evidence: [tc39.es](https://tc39.es/proposal-temporal/docs/).

### [Beyond ICU4X 2.0: ZeroVec, Diplomat, Yoke, and Unicode’s Contributions to the Rust Ecosystem](https://2025.rustweek.org/talks/shane/)

Talk · Shane Carr · public source page

**Preparation.** Introduces Unicode’s Rust libraries and bindings work behind cross-language integration.

[RustWeek 2025](https://2025.rustweek.org/talks/shane/) · Delivery unverified.

Related library background, not the same Temporal talk. Official description checked; delivery and recording are unverified.

Listing checked: 2026-10-15. Evidence: [2025.rustweek.org](https://2025.rustweek.org/talks/shane/).

## Building a Query System

Session ID: `talks-building-a-query-system`. [Official source](https://eurorust.eu/talks/building-a-query-system/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed.

### [Queries: demand-driven compilation](https://rustc-dev-guide.rust-lang.org/query.html)

Documentation · Rust compiler contributors · open access

**Preparation.** Explains memoized queries, providers and dependency tracking in rustc.

Source page checked: 2026-10-15. Evidence: [rustc-dev-guide.rust-lang.org](https://rustc-dev-guide.rust-lang.org/query.html).

### [The Salsa Book](https://salsa-rs.github.io/salsa/)

Book · Salsa contributors · open access

**Same topic.** Shows an incremental computation model and a small language tutorial for comparison.

Listing checked: 2026-10-15. Evidence: [salsa-rs.github.io](https://salsa-rs.github.io/salsa/).

### [Build systems à la carte](https://www.microsoft.com/en-us/research/publication/build-systems-la-carte/)

Paper · Andrey Mokhov, Neil Mitchell, Simon Peyton Jones · open access

**Preparation.** Provides a framework for reasoning about dependencies, cached results and rebuild decisions.

2018 ICFP paper with Haskell examples; concepts transfer to Rust query systems.

Listing checked: 2026-10-15. Evidence: [www.microsoft.com](https://www.microsoft.com/en-us/research/publication/build-systems-la-carte/).

### [Talking to ghosts](https://rustlab.it/talks/talking-to-ghosts)

Talk · Arya Dradjica · registration

**Suggested next step.** Continues the speaker’s compiler work through concurrent data structures and memory reclamation.

[RustLab 2026](https://rustlab.it/talks/talking-to-ghosts) · 2026-11-03 · Announced.

An announced follow-up topic, not a repeat of the query-system talk.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/talking-to-ghosts).

## Tales from the Embedded Rust Frontier: Bootstrapping a brand new ecosystem

Session ID: `talks-tales-from-the-embedded-rust-frontier-bootstrapping-a-brand-new-ecosystem`. [Official source](https://eurorust.eu/talks/tales-from-the-embedded-rust-frontier-bootstrapping-a-brand-new-ecosystem/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [Rust on Cortex-R52](https://ferrous-systems.com/blog/rust-cortex-r52/)

Article · Ferrous Systems · open access

**Project evolution.** Documents the libraries and examples released for the intermediate Arm processors discussed in the talk.

Publication date: 2025-03-11.

Source page checked: 2026-10-15. Evidence: [ferrous-systems.com](https://ferrous-systems.com/blog/rust-cortex-r52/).

### [aarch32: Rust support for Arm Cortex-R and Cortex-A](https://github.com/rust-embedded/aarch32)

Project · Rust Embedded Devices Working Group · open access

**Same topic.** Provides low-level processor access and runtime support to inspect after the ecosystem overview.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/rust-embedded/aarch32), [ferrous-systems.com](https://ferrous-systems.com/blog/rust-cortex-r52/).

### [Rust and ThreadX](https://ferrous-systems.com/blog/rust-and-threadx/)

Article · Jonathan Pallant · open access

**Preparation.** Explains why integrating Rust with an existing real-time operating system can be useful.

Publication date: 2023-12-07.

Historical integration example; certification statements are tied to its publication date.

Source page checked: 2026-10-15. Evidence: [ferrous-systems.com](https://ferrous-systems.com/blog/rust-and-threadx/).

### [The Embedded Rust Book](https://docs.rust-embedded.org/book/)

Book · Rust Embedded Devices Working Group · open access

**Preparation.** Builds the no\_std and peripheral vocabulary needed to compare embedded ecosystems.

Introductory examples focus on microcontrollers rather than covering every Cortex-A or Cortex-R target.

Listing checked: 2026-10-15. Evidence: [docs.rust-embedded.org](https://docs.rust-embedded.org/book/).

## From Hardware to Human Interface: Building HID Devices in Rust

Session ID: `talks-from-hardware-to-human-interface-building-hid-devices-in-rust`. [Official source](https://eurorust.eu/talks/from-hardware-to-human-interface-building-hid-devices-in-rust/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed.

### [Device Class Definition for HID 1.11](https://usb.org/document-library/device-class-definition-hid-111)

Documentation · USB Implementers Forum · open access

**Preparation.** Defines the descriptors and reports that let input devices work with standard host drivers.

Listing checked: 2026-10-15. Evidence: [usb.org](https://usb.org/document-library/device-class-definition-hid-111).

### [usbd-hid](https://docs.rs/usbd-hid/latest/usbd_hid/)

Documentation · usbd-hid contributors · open access

**Same topic.** Documents Rust support for HID report descriptors, input reports and USB communication.

Source page checked: 2026-10-15. Evidence: [docs.rs](https://docs.rs/usbd-hid/latest/usbd_hid/).

### [USB Complete: The Developer’s Guide, fifth edition](https://www.ipgbook.com/usb-complete-products-9781931448284.php)

Book · Jan Axelson · purchase

**Preparation.** Explains USB device classes, host drivers and debugging beyond a particular Rust library.

Publisher-distributor overview checked; 2015 edition, with examples outside Rust.

Listing checked: 2026-10-15. Evidence: [www.ipgbook.com](https://www.ipgbook.com/usb-complete-products-9781931448284.php).

### [From hardware to Human Interface: buiding HID devices in Rust](https://rustlab.it/talks/from-hardware-to-huma-interface-buiding-hid-devices-in-rust)

Talk · Frank Lyaruu · registration

**Repeat appearance.** Offers another announced appearance of the same speaker, topic and abstract.

[RustLab 2026](https://rustlab.it/talks/from-hardware-to-huma-interface-buiding-hid-devices-in-rust) · 2026-11-02 · Announced.

Announced for November 2, 2026; the organizer’s title contains a typo, and content may change.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/from-hardware-to-huma-interface-buiding-hid-devices-in-rust).

## Rebuilding the Tower of Babel – Write Your Own Language Server with Tree-sitter

Session ID: `talks-rebuilding-the-tower-of-babel-write-your-own-language-server-with-tree-sitter`. [Official source](https://eurorust.eu/talks/rebuilding-the-tower-of-babel-write-your-own-language-server-with-tree-sitter/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [Tree-sitter documentation](https://tree-sitter.github.io/tree-sitter/)

Documentation · Tree-sitter contributors · open access

**Preparation.** Explains incremental parsing and grammar construction for editor tooling.

Source page checked: 2026-10-15. Evidence: [tree-sitter.github.io](https://tree-sitter.github.io/tree-sitter/).

### [Language Server Protocol 3.17 specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)

Documentation · Microsoft and LSP contributors · open access

**Same topic.** Defines document synchronization, diagnostics and completion messages for an editor connection.

Listing checked: 2026-10-15. Evidence: [microsoft.github.io](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/).

### [Crafting Interpreters](https://craftinginterpreters.com/contents.html)

Book · Robert Nystrom · open access

**Preparation.** Builds the syntax-tree and parsing foundations needed when designing a language grammar.

The book uses Java and C; it teaches language implementation rather than Tree-sitter or LSP APIs.

Listing checked: 2026-10-15. Evidence: [craftinginterpreters.com](https://craftinginterpreters.com/contents.html).

## Lessons from building a background task processing library

Session ID: `talks-lessons-from-building-a-background-task-processing-library`. [Official source](https://eurorust.eu/talks/lessons-from-building-a-background-task-processing-library/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

### [Apalis](https://apalis.dev/)

Project · Njuguna Mureithi, Apalis contributors · open access

**Same topic.** Provides the worker API, storage backends and shutdown behavior behind the speaker’s library.

Source page checked: 2026-10-15. Evidence: [apalis.dev](https://apalis.dev/).

### [Background job processing with Rust using Actix and Redis](https://mureithi.me/blog/background-job-processing-with-rust-actix-redis)

Article · Njuguna Mureithi · open access

**Project evolution.** Shows an earlier design for separating request handling from background job execution.

Historical article; examples may use older APIs than current Apalis.

Source page checked: 2026-10-15. Evidence: [mureithi.me](https://mureithi.me/blog/background-job-processing-with-rust-actix-redis).

### [Graceful Shutdown](https://tokio.rs/tokio/topics/shutdown)

Documentation · Tokio contributors · open access

**Preparation.** Explains cancellation and task completion when workers stop or restart.

Source page checked: 2026-10-15. Evidence: [tokio.rs](https://tokio.rs/tokio/topics/shutdown).

### [Timeouts, retries, and backoff with jitter](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/)

Article · Amazon Builders’ Library · open access

**Suggested next step.** Explains retry budgets and backoff choices that keep background workers from multiplying failures.

Source page checked: 2026-10-15. Evidence: [aws.amazon.com](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/).

## Running Safely with Scissors: Lock-free Concurrency for Maximum Throughput

Session ID: `talks-running-safely-with-scissors-lock-free-concurrency-for-maximum-throughput`. [Official source](https://eurorust.eu/talks/running-safely-with-scissors-lock-free-concurrency-for-maximum-throughput/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed.

### [Rust Atomics and Locks](https://mara.nl/atomics/)

Book · Mara Bos · open access

**Preparation.** Builds the memory-ordering and synchronization knowledge needed to assess lock-free code.

Free online edition; the book’s introduction and contents were checked.

Listing checked: 2026-10-15. Evidence: [mara.nl](https://mara.nl/atomics/).

### [Lock-freedom without garbage collection](https://aturon.github.io/blog/2015/08/27/epoch/)

Article · Aaron Turon · open access

**Same topic.** Explains epoch-based reclamation and how Rust types can protect concurrent data-structure APIs.

Publication date: 2015-08-27.

Historical Crossbeam design article; current library APIs and benchmark results may differ.

Source page checked: 2026-10-15. Evidence: [aturon.github.io](https://aturon.github.io/blog/2015/08/27/epoch/).

### [Talking to ghosts](https://rustlab.it/talks/talking-to-ghosts)

Talk · Arya Dradjica · registration

**Same topic.** Offers another treatment of concurrent memory reclamation and its API tradeoffs.

[RustLab 2026](https://rustlab.it/talks/talking-to-ghosts) · 2026-11-03 · Announced.

Announced session on November 3, 2026; no recording is claimed.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/talking-to-ghosts).

## Game Show

Session ID: `talks-game-show`. [Official source](https://eurorust.eu/talks/game-show/).

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. These are optional background links; no game-show syllabus is assumed. No verified future conference match is included in this pass.

### [Engineering a Rust optimization quiz](https://fasterthanli.me/articles/engineering-a-rust-optimization-quiz)

Article · Amos Wenger · open access

**Preparation.** Shows how one host built an earlier Rust quiz and chose questions about compiler behavior.

Publication date: 2025-11-01.

Optional background; it does not reveal or verify the 2026 game-show questions.

Source page checked: 2026-10-15. Evidence: [fasterthanli.me](https://fasterthanli.me/articles/engineering-a-rust-optimization-quiz).

### [Rust Quiz](https://dtolnay.github.io/rust-quiz/)

Course · David Tolnay · open access

**Preparation.** Offers short code puzzles with explanations for practicing careful Rust code reading.

Independent optional practice, not official game-show preparation.

Source page checked: 2026-10-15. Evidence: [dtolnay.github.io](https://dtolnay.github.io/rust-quiz/).

### [Rustlings](https://rustlings.rust-lang.org/)

Course · Rustlings contributors · open access

**Preparation.** Offers small exercises for refreshing syntax and compiler-error reading before a code-based activity.

Broad optional practice; the game show has no published technical syllabus.

Source page checked: 2026-10-15. Evidence: [rustlings.rust-lang.org](https://rustlings.rust-lang.org/).

## Inside Cloud Hypervisor: From KVM to a Running VM

Session ID: `talks-inside-cloud-hypervisor-from-kvm-to-a-running-vm`. [Official source](https://eurorust.eu/talks/inside-cloud-hypervisor-from-kvm-to-a-running-vm/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [Quickly Boot a Minimal Linux Kernel + Initrd in Cloud Hypervisor or QEMU](https://phip1611.de/blog/quickly-boot-a-minimal-linux-kernel-initrd-in-cloud-hypervisor-or-qemu/)

Article · Philipp Schuster · open access

**Preparation.** Build the kernel and initrd inputs needed to follow the session's VM boot sequence.

Publication date: 2025-03-18.

The example uses Nix; commands were not executed during this resource review.

Source page checked: 2026-10-15. Evidence: [phip1611.de](https://phip1611.de/blog/quickly-boot-a-minimal-linux-kernel-initrd-in-cloud-hypervisor-or-qemu/).

### [The KVM API](https://docs.kernel.org/virt/kvm/api.html)

Documentation · Linux kernel contributors · open access

**Suggested next step.** Look up the kernel interfaces behind guest memory, virtual CPUs, and VM execution.

Source page checked: 2026-10-15. Evidence: [docs.kernel.org](https://docs.kernel.org/virt/kvm/api.html).

### [Firecracker: Lightweight Virtualization for Serverless Applications](https://www.usenix.org/conference/nsdi20/presentation/agache)

Paper · Alexandru Agache, Marc Brooker, Andreea Florescu, Alexandra Iordache, Anthony Liguori, Rolf Neugebauer, Phil Piwonka, Diana-Maria Popa · open access

**Alternative approach.** Compare a serverless VMM's isolation and device choices with Cloud Hypervisor.

[USENIX NSDI 2020](https://www.usenix.org/conference/nsdi20/presentation/agache) · Published.

The proceedings page links the paper, slides, and recording; the full paper and video were not reviewed.

Source page checked: 2026-10-15. Evidence: [www.usenix.org](https://www.usenix.org/conference/nsdi20/presentation/agache).

### [Cloud Hypervisor](https://github.com/cloud-hypervisor/cloud-hypervisor)

Project · Cloud Hypervisor contributors · open access

**Suggested next step.** Follow the VM monitor's current architecture, guest boot instructions, and device support.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/cloud-hypervisor/cloud-hypervisor).

## What's next for Rust's type system

Session ID: `talks-what-s-next-for-rust-s-type-system`. [Official source](https://eurorust.eu/talks/what-s-next-for-rust-s-type-system/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [-Znext-solver: what, why, and when](https://eurorust.eu/2025/talks/znext-solver/)

Talk · lcnr · public source page

**Preparation.** Read the earlier motivation and design challenges behind the new trait solver.

[EuroRust 2025](https://eurorust.eu/2025/talks/znext-solver/) · 2025-10-09 · Delivery unverified.

Official session description checked; no recording is linked here.

Listing checked: 2026-10-15. Evidence: [eurorust.eu](https://eurorust.eu/2025/talks/znext-solver/).

### [Enabling the next-generation trait solver on nightly](https://blog.rust-lang.org/2026/08/21/enabling-next-solver-on-nightly/)

Article · lcnr, Rustc Trait System Refactor Initiative · open access

**Project evolution.** Connect the solver redesign to inference changes, compatibility tests, and future language work.

Publication date: 2026-08-21.

A dated nightly rollout report; planned stabilization is not a stable-release guarantee.

Source page checked: 2026-10-15. Evidence: [blog.rust-lang.org](https://blog.rust-lang.org/2026/08/21/enabling-next-solver-on-nightly/).

### [Next-gen trait solving](https://rustc-dev-guide.rust-lang.org/solve/trait-solving.html)

Documentation · Rust compiler contributors · open access

**Suggested next step.** Learn how goals, candidates, and constraints turn trait bounds into compiler decisions.

Source page checked: 2026-10-15. Evidence: [rustc-dev-guide.rust-lang.org](https://rustc-dev-guide.rust-lang.org/solve/trait-solving.html).

### [Diving Deep: implied bounds and variance](https://lcnr.de/blog/2022/02/05/diving-deep-implied-bounds-and-variance.html)

Article · lcnr · open access

**Preparation.** Work through a concrete soundness problem involving lifetime bounds and variance.

Publication date: 2022-02-05.

Historical analysis with an author correction; check the linked issue for current status.

Source page checked: 2026-10-15. Evidence: [lcnr.de](https://lcnr.de/blog/2022/02/05/diving-deep-implied-bounds-and-variance.html).

## Image codecs rustification in Chromium

Session ID: `talks-image-codecs-rustification-in-chromium`. [Official source](https://eurorust.eu/talks/image-codecs-rustification-in-chromium/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [Rustifying Image Codecs in Chromium](https://microsoftedge.github.io/edgevr/posts/Rustifying-Image-Codecs-in-Chromium/)

Article · Sergio Gonzalez Martin · open access

**Same topic.** Read the speaker's account of codec migration, streaming compatibility, upstream work, and staged rollout.

Publication date: 2026-07-17.

Codec status and performance results describe the author's dated evaluation.

Source page checked: 2026-10-15. Evidence: [microsoftedge.github.io](https://microsoftedge.github.io/edgevr/posts/Rustifying-Image-Codecs-in-Chromium/).

### [Rust PNG crate gets even faster, used by GNOME and Chromium](https://blog.image-rs.org/2026/06/18/png-adoption.html)

Article · Shnatsel · open access

**Preparation.** See the upstream PNG work on partial decoding, compatibility, fuzzing, and browser integration.

Publication date: 2026-06-18.

Source page checked: 2026-10-15. Evidence: [blog.image-rs.org](https://blog.image-rs.org/2026/06/18/png-adoption.html).

### [image: encoding and decoding images in Rust](https://github.com/image-rs/image)

Project · image-rs contributors · open access

**Suggested next step.** Inspect the upstream decoder APIs and supported formats used in the browser migration.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/image-rs/image).

### [CXX: Rust and C++ interoperability](https://cxx.rs/)

Documentation · David Tolnay, CXX contributors · open access

**Preparation.** Understand the Rust and C++ boundary needed when replacing codecs inside an existing browser.

Source page checked: 2026-10-15. Evidence: [cxx.rs](https://cxx.rs/).

## Rust's Influence on C++ and Python: Shaping the Future of Three Communities

Session ID: `talks-rust-s-influence-on-c-and-python-shaping-the-future-of-three-communities`. [Official source](https://eurorust.eu/talks/rust-s-influence-on-c-and-python-shaping-the-future-of-three-communities/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [What comes after Rust in the Python ecosystem?](https://www.youtube.com/watch?v=yJPX7n0tMfo)

Video · Cristián Maureira-Fredes · open access

**Preparation.** Compare the speaker's earlier account of Rust and other compiled languages extending Python.

[EuroPython 2025](https://maureira.dev/speaking/) · Published.

The speaker links this July 2025 recording; YouTube retrieval was throttled and playback was not checked.

Listing checked: 2026-10-15. Evidence: [maureira.dev](https://maureira.dev/speaking/).

### [¿Qué viene después de Rust en el ecosistema Python?](https://pretalx.com/pycones-2026/talk/GXFMUV/)

Talk · Cristián Maureira-Fredes · access unverified

**Suggested next step.** Follow the speaker's announced comparison of Rust, C++, and other language extensions for Python.

[PyConES 2026](https://pretalx.com/pycones-2026/talk/GXFMUV/) · 2026-11-07 · Announced.

Spanish session; programme verified, attendance requirements and future recording availability not checked.

Listing checked: 2026-10-15. Evidence: [pretalx.com](https://pretalx.com/pycones-2026/talk/GXFMUV/).

### [PyO3 user guide](https://pyo3.rs/)

Documentation · PyO3 contributors · open access

**Suggested next step.** Study Rust extension modules, Python object ownership, and conversion across the language boundary.

The root URL follows the current version; match documentation to your dependency version.

Source page checked: 2026-10-15. Evidence: [pyo3.rs](https://pyo3.rs/).

### [CXX: Rust and C++ interoperability](https://cxx.rs/)

Documentation · David Tolnay, CXX contributors · open access

**Preparation.** Compare the concrete Rust/C++ bridge model with Python's Rust extension model.

Source page checked: 2026-10-15. Evidence: [cxx.rs](https://cxx.rs/).

## Blastbeat: When a Decade of C++ Meets Rust and io\_uring

Session ID: `talks-blastbeat-when-a-decade-of-c-meets-rust-and-io-uring`. [Official source](https://eurorust.eu/talks/blastbeat-when-a-decade-of-c-meets-rust-and-io-uring/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [Announcing tokio-uring: io-uring support for Tokio](https://tokio.rs/blog/2021-07-tokio-uring)

Article · Tokio contributors · open access

**Preparation.** Understand why completion-based I/O owns buffers differently from readiness-based async I/O.

Publication date: 2021-07-19.

Historical design introduction; its initial release API is not current setup guidance.

Source page checked: 2026-10-15. Evidence: [tokio.rs](https://tokio.rs/blog/2021-07-tokio-uring).

### [Lessons learned from monitoring a large production network](https://devopsdays.org/events/2020-madrid/program/giacomo-bagnoli/)

Talk · Giacomo Bagnoli · public source page

**Preparation.** Compare the speaker's earlier network-health monitoring problems with the later Blastbeat rewrite.

[devopsdays Madrid 2020](https://devopsdays.org/events/2020-madrid/program/giacomo-bagnoli/) · Delivery unverified.

The archived programme describes monitoring practices, not the Rust implementation; recording availability was not verified.

Listing checked: 2026-10-15. Evidence: [devopsdays.org](https://devopsdays.org/events/2020-madrid/program/giacomo-bagnoli/).

### [Canarying Releases](https://sre.google/workbook/canarying-releases/)

Book · Google SRE contributors · open access

**Suggested next step.** Plan rollout signals and failure checks for replacing an agent across a server fleet.

Source page checked: 2026-10-15. Evidence: [sre.google](https://sre.google/workbook/canarying-releases/).

### [Blastbeat: Replacing 10 Years of C++ with Rust and io\_uring](https://rustlab.it/talks/blastbeat-replacing-10-years-of-c-with-rust-and-iouring)

Talk · Giacomo Bagnoli · registration

**Repeat appearance.** Follow the announced RustLab appearance on the same network-observability rewrite.

[RustLab 2026](https://rustlab.it/talks/blastbeat-replacing-10-years-of-c-with-rust-and-iouring) · 2026-11-03 · Announced.

The same project and one EuroRust speaker; presentation details may differ and no future recording is promised.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/blastbeat-replacing-10-years-of-c-with-rust-and-iouring).

## Designing State Machines for Streaming Proxies in Rust

Session ID: `talks-designing-state-machines-for-streaming-proxies-in-rust`. [Official source](https://eurorust.eu/talks/designing-state-machines-for-streaming-proxies-in-rust/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [Pretty State Machine Patterns in Rust](https://hoverbear.org/blog/rust-state-machine-pattern/)

Article · Hoverbear · open access

**Preparation.** Compare enums and owned transitions for representing valid protocol states.

Publication date: 2016-10-12.

A historical design article; examples were not compiled against current Rust.

Source page checked: 2026-10-15. Evidence: [hoverbear.org](https://hoverbear.org/blog/rust-state-machine-pattern/).

### [How we built Pingora, the proxy that connects Cloudflare to the Internet](https://blog.cloudflare.com/how-we-built-pingora-the-proxy-that-connects-cloudflare-to-the-internet/)

Article · Yuchen Wu, Andrew Hauck · open access

**Same topic.** Study a production Rust proxy's architecture and connection reuse decisions.

Publication date: 2022-09-14.

A separate proxy implementation; its design is not attributed to the EuroRust speaker.

Source page checked: 2026-10-15. Evidence: [blog.cloudflare.com](https://blog.cloudflare.com/how-we-built-pingora-the-proxy-that-connects-cloudflare-to-the-internet/).

### [Tokio tutorial: Select](https://tokio.rs/tokio/tutorial/select)

Documentation · Tokio contributors · open access

**Preparation.** Understand what happens to unfinished futures when another stream or cancellation branch wins.

Source page checked: 2026-10-15. Evidence: [tokio.rs](https://tokio.rs/tokio/tutorial/select).

## Qt Bridges: Bringing a mature C++ UI framework to Rust

Session ID: `talks-qt-bridges-bringing-a-mature-c-ui-framework-to-rust`. [Official source](https://eurorust.eu/talks/qt-bridges-bringing-a-mature-c-ui-framework-to-rust/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [Qt Bridge for Rust 0.3 Beta: CXX-Qt Compatibility and Soundness](https://www.qt.io/blog/qt-bridge-for-rust-0.3-cxx-qt-compatibility-and-soundness)

Article · Matthias Rauter · open access

**Same topic.** Read the speaker's explanation of ownership changes and soundness fixes at the QML boundary.

Publication date: 2026-09-23.

This describes a beta version; check current APIs before using its migration advice.

Source page checked: 2026-10-15. Evidence: [www.qt.io](https://www.qt.io/blog/qt-bridge-for-rust-0.3-cxx-qt-compatibility-and-soundness).

### [CXX-Qt documentation](https://kdab.github.io/cxx-qt/book/)

Book · KDAB, CXX-Qt contributors · open access

**Suggested next step.** Follow QObject, QML, type conversion, and build-system integration in a related Qt bridge.

Source page checked: 2026-10-15. Evidence: [kdab.github.io](https://kdab.github.io/cxx-qt/book/).

### [CXX: Rust and C++ interoperability](https://cxx.rs/)

Documentation · David Tolnay, CXX contributors · open access

**Preparation.** Understand the CXX layer that supports Rust and C++ types crossing the Qt boundary.

Source page checked: 2026-10-15. Evidence: [cxx.rs](https://cxx.rs/).

## Supply Chain Attacks, and how to protect ourselves

Session ID: `talks-supply-chain-attacks-and-how-to-protect-ourselves`. [Official source](https://eurorust.eu/talks/supply-chain-attacks-and-how-to-protect-ourselves/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [RustSec Advisory Database](https://rustsec.org/)

Documentation · Rust Secure Code Working Group · open access

**Suggested next step.** Find published crate advisories and understand the inputs used by dependency audit tools.

Source page checked: 2026-10-15. Evidence: [rustsec.org](https://rustsec.org/).

### [Cargo Vet guide](https://mozilla.github.io/cargo-vet/)

Documentation · Cargo Vet contributors · open access

**Suggested next step.** Learn how reviewed versions and audit records can support dependency policy.

Review evidence and advisory checks cover different risks; neither proves a dependency harmless.

Source page checked: 2026-10-15. Evidence: [mozilla.github.io](https://mozilla.github.io/cargo-vet/).

### [Be alert: targeted attacks on prominent Rustaceans](https://blog.rust-lang.org/2026/09/17/targeted-attacks/)

Article · Adam Harvey, crates.io team, Rust Security Response Working Group · open access

**Same topic.** Connect account compromise and social engineering to the path malicious releases can take.

Publication date: 2026-09-17.

Source page checked: 2026-10-15. Evidence: [blog.rust-lang.org](https://blog.rust-lang.org/2026/09/17/targeted-attacks/).

### [Auditing Rust Crates Effectively](https://arxiv.org/abs/2602.06466)

Paper · Lydia Zoghbi, David Thien, Ranjit Jhala, Deian Stefan, Caleb Stanford · open access

**Suggested next step.** Explore effect analysis that narrows the code requiring manual dependency review.

Publication date: 2026-02-06.

Research preprint; metadata and abstract checked, with no independent validation of its evaluation.

Source page checked: 2026-10-15. Evidence: [arxiv.org](https://arxiv.org/abs/2602.06466).

## Build systems: when your systems need systems to build

Session ID: `talks-build-systems-when-your-systems-need-systems-to-build`. [Official source](https://eurorust.eu/talks/build-systems-when-your-systems-need-systems-to-build/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [Build systems à la carte](https://www.microsoft.com/en-us/research/publication/build-systems-la-carte/)

Paper · Andrey Mokhov, Neil Mitchell, Simon Peyton Jones · open access

**Preparation.** Compare dependency discovery, scheduling, and rebuilding as separate build-system design choices.

[ICFP 2018](https://www.microsoft.com/en-us/research/publication/build-systems-la-carte/) · Published.

Research landing page and summary checked; the full paper was not reviewed.

Source page checked: 2026-10-15. Evidence: [www.microsoft.com](https://www.microsoft.com/en-us/research/publication/build-systems-la-carte/).

### [Hermeticity](https://bazel.build/basics/hermeticity)

Documentation · Bazel contributors · open access

**Preparation.** Understand why declared inputs and controlled environments matter for reliable caching.

Source page checked: 2026-10-15. Evidence: [bazel.build](https://bazel.build/basics/hermeticity).

### [Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

Documentation · Cargo contributors · open access

**Suggested next step.** Review Cargo's input tracking and native-build interface before adding another build layer.

Source page checked: 2026-10-15. Evidence: [doc.rust-lang.org](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

### [Tips for Speeding Up Cargo Build in CI](https://rustconf2026.sched.com/event/2KHtV)

Talk · Kaitlin Seng · access unverified

**Same topic.** Compare CI-focused build improvements with broader build-system design.

[RustConf 2026](https://rustconf2026.sched.com/) · 2026-09-09 · Delivery unverified.

Official timetable and speaker roster checked. Detail-page access was inconsistent; delivery and recording remain unverified.

Listing checked: 2026-10-15. Evidence: [rustconf.com](https://rustconf.com/speakers/), [rustconf2026.sched.com](https://rustconf2026.sched.com/).

## Vortex: One Format for Any Shape

Session ID: `talks-vortex-one-format-for-any-shape`. [Official source](https://eurorust.eu/talks/vortex-one-format-for-any-shape/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [Vortex: One Format for Any Shape](https://spiraldb.com/blog/vortex-one-format-for-any-shape)

Article · Connor Tsui · open access

**Same topic.** Read the speaker's longer explanation of extensible logical types, encodings, and storage layouts.

Publication date: 2026-09-16.

Source page checked: 2026-10-15. Evidence: [spiraldb.com](https://spiraldb.com/blog/vortex-one-format-for-any-shape).

### [Vortex](https://github.com/vortex-data/vortex)

Project · Vortex contributors · open access

**Suggested next step.** Inspect the format, compression implementations, and examples behind the session.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/vortex-data/vortex).

### [Arrow Columnar Format](https://arrow.apache.org/docs/format/Columnar.html)

Documentation · Apache Arrow contributors · open access

**Preparation.** Learn the buffer layouts, nested types, and extension types behind columnar data interchange.

Source page checked: 2026-10-15. Evidence: [arrow.apache.org](https://arrow.apache.org/docs/format/Columnar.html).

### [BtrBlocks: Efficient Columnar Compression for Data Lakes](https://github.com/maxi-k/btrblocks)

Project · Maximilian Kuschewski, David Sauerwein, Adnan Alhomssi, Viktor Leis · open access

**Alternative approach.** Compare another columnar compression design through its research code, paper, and SIGMOD presentation links.

[ACM SIGMOD 2023](https://github.com/maxi-k/btrblocks) · Published.

Research artifact and authors checked; the linked paper and recording were not reviewed in full.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/maxi-k/btrblocks), [zenodo.org](https://zenodo.org/records/7936448).

## Atomic Design Patterns in Rust: From Orderings to Code

Session ID: `talks-atomic-design-patterns-in-rust-from-orderings-to-code`. [Official source](https://eurorust.eu/talks/atomic-design-patterns-in-rust-from-orderings-to-code/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [Rust Atomics and Locks](https://mara.nl/atomics/)

Book · Mara Bos · open access

**Preparation.** Work through memory ordering and the construction of locks and other synchronization primitives.

The author provides the full book online; print and commercial ebook editions are separate.

Source page checked: 2026-10-15. Evidence: [mara.nl](https://mara.nl/atomics/).

### [Rust atomic types and memory ordering](https://doc.rust-lang.org/std/sync/atomic/index.html)

Documentation · Rust project contributors · open access

**Preparation.** Check the standard library's ordering guarantees and restrictions before judging an atomic pattern.

Source page checked: 2026-10-15. Evidence: [doc.rust-lang.org](https://doc.rust-lang.org/std/sync/atomic/index.html).

### [Loom](https://github.com/tokio-rs/loom)

Project · Loom contributors · open access

**Suggested next step.** Explore small concurrent tests across possible thread and memory-model interleavings.

Model checking applies within the tested model and bounds; it does not replace a correctness argument.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/tokio-rs/loom).

### [Rust Atomics in the Wild: Memory Ordering Design Patterns from Popular Crates](https://rustconf2026.sched.com/event/2KHth)

Talk · Martin Ombura Jr. · access unverified

**Same topic.** Follow the same speaker's earlier session on atomic patterns found in Rust crates.

[RustConf 2026](https://rustconf2026.sched.com/) · 2026-09-10 · Delivery unverified.

Official timetable and speaker roster checked. Detail-page access was inconsistent; delivery and recording remain unverified.

Listing checked: 2026-10-15. Evidence: [rustconf.com](https://rustconf.com/speakers/), [rustconf2026.sched.com](https://rustconf2026.sched.com/).

### [Talking to ghosts](https://rustlab.it/talks/talking-to-ghosts)

Talk · Arya Dradjica · registration

**Suggested next step.** Extend atomic coordination into the harder problem of reclaiming memory while other threads may still read it.

[RustLab 2026](https://rustlab.it/talks/talking-to-ghosts) · 2026-11-03 · Announced.

Future programme entry; concurrent memory reclamation is a follow-up topic, not a repeat of the EuroRust talk.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/talking-to-ghosts).

## Moving a quantum compiler from Python to Rust

Session ID: `talks-moving-a-quantum-compiler-from-python-to-rust`. [Official source](https://eurorust.eu/talks/moving-a-quantum-compiler-from-python-to-rust/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [LightSABRE: A Lightweight and Enhanced SABRE Algorithm](https://arxiv.org/abs/2409.08368)

Paper · Henry Zou, Matthew Treinish, Kevin Hartman, Alexander Ivrii, Jake Lishman · open access

**Same topic.** Study a concrete Qiskit routing algorithm developed mainly in Rust during the compiler migration.

Publication date: 2024-09-12.

Preprint abstract and metadata checked; performance figures belong to the paper's specific comparisons.

Source page checked: 2026-10-15. Evidence: [arxiv.org](https://arxiv.org/abs/2409.08368).

### [Release News: Qiskit SDK v2.0 is here!](https://www.ibm.com/quantum/blog/qiskit-2-0-release-summary)

Article · Qiskit Team · open access

**Project evolution.** Follow the Rust data-model changes and C interface introduced during Qiskit's migration.

Publication date: 2025-04-03.

A historical release report; consult current documentation for supported APIs.

Source page checked: 2026-10-15. Evidence: [www.ibm.com](https://www.ibm.com/quantum/blog/qiskit-2-0-release-summary).

### [PyO3 user guide](https://pyo3.rs/)

Documentation · PyO3 contributors · open access

**Suggested next step.** Understand the Python/Rust ownership boundary used when accelerating an existing Python library.

The root URL follows the current version; match documentation to your dependency version.

Source page checked: 2026-10-15. Evidence: [pyo3.rs](https://pyo3.rs/).

### [Introduction to transpilation](https://quantum.cloud.ibm.com/docs/en/guides/transpile)

Documentation · IBM Quantum contributors · open access

**Preparation.** Learn why quantum circuits need routing and transformation for the target hardware.

Source page checked: 2026-10-15. Evidence: [quantum.cloud.ibm.com](https://quantum.cloud.ibm.com/docs/en/guides/transpile).

## Building a high performance, thread-per-core, zero-copy OpenTelemetry engine in Rust

Session ID: `talks-building-a-high-performance-thread-per-core-zero-copy-opentelemetry-engine-in-rust`. [Official source](https://eurorust.eu/talks/building-a-high-performance-thread-per-core-zero-copy-opentelemetry-engine-in-rust/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [OTel-Arrow Phase 2: From Efficient Transport to Efficient Telemetry Pipelines](https://opentelemetry.io/blog/2026/otel-arrow-phase-2/)

Article · Laurent Querel, Joshua MacDonald, Reiley Yang, Albert Lockett, Cijo Thomas, Drew Relmas, Jake Dern · open access

**Same topic.** Read the coauthored account of bounded pipelines, thread-per-core execution, and columnar telemetry.

Publication date: 2026-06-12.

Reported benchmark results apply to the documented hardware and configurations.

Source page checked: 2026-10-15. Evidence: [opentelemetry.io](https://opentelemetry.io/blog/2026/otel-arrow-phase-2/).

### [OpenTelemetry Arrow](https://github.com/open-telemetry/otel-arrow)

Project · OpenTelemetry Arrow contributors · open access

**Suggested next step.** Inspect the protocol, dataflow engine, and benchmark material behind the architecture.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/open-telemetry/otel-arrow).

### [Arrow Columnar Format](https://arrow.apache.org/docs/format/Columnar.html)

Documentation · Apache Arrow contributors · open access

**Preparation.** Understand the columnar buffers that make direct telemetry processing possible.

Source page checked: 2026-10-15. Evidence: [arrow.apache.org](https://arrow.apache.org/docs/format/Columnar.html).

### [Shared-nothing Design](https://seastar.io/shared-nothing/)

Article · Seastar contributors · open access

**Preparation.** Compare per-core ownership and explicit message passing with shared-state concurrency.

Seastar's examples are C++; this is architectural background, not an OTel Rust API guide.

Source page checked: 2026-10-15. Evidence: [seastar.io](https://seastar.io/shared-nothing/).

## Your Infrastructure Should Compile, Too

Session ID: `talks-your-infrastructure-should-compile-too`. [Official source](https://eurorust.eu/talks/your-infrastructure-should-compile-too/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [Designing an algorithmic cloud infrastructure provisioning system](https://dev.to/encore/designing-an-algorithmic-cloud-infrastructure-provisioning-system-kde)

Article · André Eriksson · open access

**Preparation.** Read how the speaker connects application analysis, infrastructure dependencies, and deployment constraints.

Publication date: 2022-10-24.

A dated architecture article; product behavior may have changed.

Source page checked: 2026-10-15. Evidence: [dev.to](https://dev.to/encore/designing-an-algorithmic-cloud-infrastructure-provisioning-system-kde).

### [Encore for Rust](https://encore.dev/blog/encore-rs)

Article · Ivan Cernja · open access

**Project evolution.** See how the announced Rust integration expresses application resource requirements in code.

Publication date: 2026-09-22.

The article is published, but describes a pre-beta preview; it does not establish general availability.

Source page checked: 2026-10-15. Evidence: [encore.dev](https://encore.dev/blog/encore-rs).

### [Why I Needed Rust: Finally, Infrastructure Automation I can sleep on](https://www.rustikon.dev/talk/why-i-needed-rust-finally-infrastructure-automation-i-can-sleep-on)

Talk · Jesús Pérez · public source page

**Same topic.** Compare another engineer's use of type safety to address drift and failed infrastructure deployments.

[Rustikon 2026](https://www.rustikon.dev/talk/why-i-needed-rust-finally-infrastructure-automation-i-can-sleep-on) · 2026-03-19 · Delivery unverified.

Programme checked; a recording was not verified.

Listing checked: 2026-10-15. Evidence: [www.rustikon.dev](https://www.rustikon.dev/talk/why-i-needed-rust-finally-infrastructure-automation-i-can-sleep-on).

### [From Scripts to Contracts: what Rust's type system taught me about infrastructure composition](https://rustlab.it/talks/from-scripts-to-contracts-what-rusts-type-system-taught-me-about-infrastructure-c)

Talk · Jesús Pérez · registration

**Suggested next step.** Explore an announced design for typed deployment composition, recovery, and agent access to infrastructure.

[RustLab 2026](https://rustlab.it/talks/from-scripts-to-contracts-what-rusts-type-system-taught-me-about-infrastructure-c) · 2026-11-03 · Announced.

Future session; the abstract's system guarantees have not been independently evaluated.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/from-scripts-to-contracts-what-rusts-type-system-taught-me-about-infrastructure-c).

## Fearless Concurrency on the GPU

Session ID: `talks-fearless-concurrency-on-the-gpu`. [Official source](https://eurorust.eu/talks/fearless-concurrency-on-the-gpu/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [Fearless Concurrency on the GPU](https://arxiv.org/abs/2606.15991)

Paper · Melih Elibol, Jared Roesch, Isaac Gelado, Eric Buehler, Michael Garland · open access

**Same topic.** Read the ownership model behind safe tile kernels and the limits that still require runtime checks.

Publication date: 2026-06-14.

Preprint first posted June 14 and revised September 17, 2026; abstract and metadata checked, not the full proof.

Source page checked: 2026-10-15. Evidence: [arxiv.org](https://arxiv.org/abs/2606.15991).

### [cuTile Rust](https://github.com/NVlabs/cutile-rs)

Project · NVIDIA Research, cuTile Rust contributors · open access

**Suggested next step.** Explore the GPU kernel examples and host-side ownership API discussed in the session.

An early research project; hardware support, API stability, and safety assumptions need checking before use.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/NVlabs/cutile-rs).

### [Translating CUDA Tile Operations from Python to Rust Using Agentic AI](https://developer.nvidia.com/blog/translating-cuda-tile-operations-from-python-to-rust-using-agentic-ai/)

Article · Yinuo Liu, Melih Elibol, Dheemanth Manur · open access

**Suggested next step.** Study an agent-assisted porting workflow that checks intermediate representation, correctness, and performance.

Publication date: 2026-09-16.

The authors' workflow and benchmarks cover their stated kernel set and hardware.

Source page checked: 2026-10-15. Evidence: [developer.nvidia.com](https://developer.nvidia.com/blog/translating-cuda-tile-operations-from-python-to-rust-using-agentic-ai/).

### [Fearless Concurrency on the GPU](https://rustconf2026.sched.com/event/2KHsv)

Talk · Melih Elibol · access unverified

**Repeat appearance.** Find the same speaker and title in the earlier RustConf programme.

[RustConf 2026](https://rustconf2026.sched.com/) · 2026-09-09 · Delivery unverified.

Official timetable and speaker roster checked. Detail-page access was inconsistent; delivery and recording remain unverified.

Listing checked: 2026-10-15. Evidence: [rustconf.com](https://rustconf.com/speakers/), [rustconf2026.sched.com](https://rustconf2026.sched.com/).

## From Deref Into Borrow, a Primer on Standard Library Traits

Session ID: `talks-from-deref-into-borrow-a-primer-on-standard-library-traits`. [Official source](https://eurorust.eu/talks/from-deref-into-borrow-a-primer-on-standard-library-traits/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [The Rust Programming Language: Treating Smart Pointers Like Regular References](https://doc.rust-lang.org/book/ch15-02-deref.html)

Book · The Rust Programming Language contributors · open access

**Preparation.** Work through Deref and deref coercion before comparing reference and conversion traits.

Source page checked: 2026-10-15. Evidence: [doc.rust-lang.org](https://doc.rust-lang.org/book/ch15-02-deref.html).

### [Rust API Guidelines: Interoperability](https://rust-lang.github.io/api-guidelines/interoperability.html)

Documentation · Rust API Guidelines contributors · open access

**Suggested next step.** Apply the conversion and common-trait conventions when reviewing a public API.

Source page checked: 2026-10-15. Evidence: [rust-lang.github.io](https://rust-lang.github.io/api-guidelines/interoperability.html).

### [Borrow trait reference](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)

Documentation · Rust project contributors · open access

**Same topic.** Check the equality, ordering, and hashing obligations that distinguish Borrow from a simple reference conversion.

Source page checked: 2026-10-15. Evidence: [doc.rust-lang.org](https://doc.rust-lang.org/std/borrow/trait.Borrow.html).

### [Standard library conversion traits](https://doc.rust-lang.org/std/convert/index.html)

Documentation · Rust project contributors · open access

**Same topic.** Compare AsRef, From, Into, and fallible conversions alongside Deref and Borrow.

Source page checked: 2026-10-15. Evidence: [doc.rust-lang.org](https://doc.rust-lang.org/std/convert/index.html).

## Beyond Flamegraphs: Precise Rust Profiling with hotpath-rs

Session ID: `talks-beyond-flamegraphs-precise-rust-profiling-with-hotpath-rs`. [Official source](https://eurorust.eu/talks/beyond-flamegraphs-precise-rust-profiling-with-hotpath-rs/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [Rust Performance Profiling using hotpath-rs](https://www.youtube.com/watch?v=ir1WC_rO2Fk)

Video · Paweł Urbanek · open access

**Same topic.** Watch the speaker's earlier profiling explanation and Zed instrumentation demonstration.

[Rustikon 2026](https://www.rustikon.dev/talk/improving-rust-performance-debugging-workflow-with-tuis-and-llms) · 2026-03-20 · Published.

The speaker links the recording; the organizer dates the related hotpath session March 20, 2026. YouTube playback was not checked.

Listing checked: 2026-10-15. Evidence: [hotpath.rs](https://hotpath.rs/blog/rust-hotpath-rustikon), [www.rustikon.dev](https://www.rustikon.dev/talk/improving-rust-performance-debugging-workflow-with-tuis-and-llms).

### [Lessons Learned Building High-Performance Rust Profiler](https://hotpath.rs/blog/rust-performance-profiling)

Article · Paweł Urbanek · open access

**Same topic.** Read the implementation tradeoffs around instrumentation overhead, cache contention, and async futures.

Publication date: 2026-05-12.

Source page checked: 2026-10-15. Evidence: [hotpath.rs](https://hotpath.rs/blog/rust-performance-profiling).

### [The Rust Performance Book: Profiling](https://nnethercote.github.io/perf-book/profiling.html)

Book · Nicholas Nethercote, The Rust Performance Book contributors · open access

**Alternative approach.** Compare sampling and instrumentation tools before choosing what to measure.

Source page checked: 2026-10-15. Evidence: [nnethercote.github.io](https://nnethercote.github.io/perf-book/profiling.html).

### [Tango: Precise Performance Measurement through Direct Comparison](https://2025.rustweek.org/talks/denis/)

Talk · Denis Bazhenov · public source page

**Suggested next step.** Follow profiling with paired benchmarks that reduce noise when comparing candidate changes.

[RustWeek 2025](https://2025.rustweek.org/talks/denis/) · Delivery unverified.

The official abstract describes benchmarking, a complementary task rather than profiling; recording not checked.

Listing checked: 2026-10-15. Evidence: [2025.rustweek.org](https://2025.rustweek.org/talks/denis/).

## The Lämp

Session ID: `talks-the-lamp`. [Official source](https://eurorust.eu/talks/the-lamp/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [The Rust on ESP Book](https://docs.espressif.com/projects/rust/book/)

Book · Espressif, esp-rs contributors · open access

**Preparation.** Learn the toolchains and embedded Rust options used around ESP devices.

Hardware background, not the lamp's build instructions; match the target chip and current support.

Source page checked: 2026-10-15. Evidence: [docs.espressif.com](https://docs.espressif.com/projects/rust/book/).

### [rs-matter-embassy](https://github.com/sysgrok/rs-matter-embassy)

Project · rs-matter-embassy contributors · open access

**Same topic.** Explore Matter integration with Embassy for microcontroller-based home automation.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/sysgrok/rs-matter-embassy).

### [ESP32C6 and IKEA Dirigera integration report](https://github.com/sysgrok/rs-matter-embassy/issues/33)

Project · Uwe Pachler, rs-matter-embassy contributors · open access

**Preparation.** Read a concrete protocol-integration problem reported by the speaker.

A public issue report, not complete lamp source code or a reviewed hardware guide.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/sysgrok/rs-matter-embassy/issues/33).

### [The Embedded Rust Book](https://docs.rust-embedded.org/book/)

Book · Rust Embedded Devices Working Group · open access

**Preparation.** Review hardware access, peripherals, and embedded concurrency before following the lamp's firmware choices.

Source page checked: 2026-10-15. Evidence: [docs.rust-embedded.org](https://docs.rust-embedded.org/book/).

## Beyond cargo build

Session ID: `talks-beyond-cargo-build`. [Official source](https://eurorust.eu/talks/beyond-cargo-build/).

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

### [Why is my Rust build so slow?](https://fasterthanli.me/articles/why-is-my-rust-build-so-slow)

Article · Amos Wenger · open access

**Preparation.** Follow the speaker's investigation of build steps, incremental work, and compiler cost.

Publication date: 2021-12-30.

A historical investigation; its compiler regressions and timings do not describe current Rust.

Source page checked: 2026-10-15. Evidence: [fasterthanli.me](https://fasterthanli.me/articles/why-is-my-rust-build-so-slow).

### [Build systems à la carte](https://www.microsoft.com/en-us/research/publication/build-systems-la-carte/)

Paper · Andrey Mokhov, Neil Mitchell, Simon Peyton Jones · open access

**Preparation.** Use a research framework to assess dynamic dependencies, caching, and remote builds.

[ICFP 2018](https://www.microsoft.com/en-us/research/publication/build-systems-la-carte/) · Published.

Research landing page and summary checked; the full paper was not reviewed.

Source page checked: 2026-10-15. Evidence: [www.microsoft.com](https://www.microsoft.com/en-us/research/publication/build-systems-la-carte/).

### [Hermeticity](https://bazel.build/basics/hermeticity)

Documentation · Bazel contributors · open access

**Preparation.** Learn the reproducibility conditions behind sharing cached or remotely executed build outputs.

Source page checked: 2026-10-15. Evidence: [bazel.build](https://bazel.build/basics/hermeticity).

### [Tips for Speeding Up Cargo Build in CI](https://rustconf2026.sched.com/event/2KHtV)

Talk · Kaitlin Seng · access unverified

**Same topic.** Compare earlier practical CI build advice with the session's broader build-system proposal.

[RustConf 2026](https://rustconf2026.sched.com/) · 2026-09-09 · Delivery unverified.

Official timetable and speaker roster checked. Detail-page access was inconsistent; delivery and recording remain unverified.

Listing checked: 2026-10-15. Evidence: [rustconf.com](https://rustconf.com/speakers/), [rustconf2026.sched.com](https://rustconf2026.sched.com/).
