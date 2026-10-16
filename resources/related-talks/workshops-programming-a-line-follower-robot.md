# Programming a Line Follower Robot

[Official EuroRust session](https://eurorust.eu/workshops/programming-a-line-follower-robot/) · [All session resources](README.md)

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed.

Speakers and their work: [Massimiliano Mantione](../speakers/massimiliano-mantione.md), [Michele Mantione](../speakers/michele-mantione.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Program simulated line-following robots with embedded async Rust, telemetry, and control refinements; this workshop does not supply physical robots.
- **Prerequisites:** Basic Rust familiarity; prior embedded experience is unnecessary and async knowledge is optional.
- **Setup:** Bring a Rust development laptop with rustup, Git, and an editor; exercises use simulation.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Motion Control with Velocity Inputs, Part 2](https://modernrobotics.northwestern.edu/nu-gm-book-resource/11-3-motion-control-with-velocity-inputs-part-2-of-3/). Explains how integral feedback reduces tracking error before tuning a line follower.
- **Bring a question:** Which assumptions need evidence from the real hardware, and which can you test in simulation?
- **Other sessions starting at this time:** [Beyond p99: Observing Async Rust](workshops-beyond-p99-observing-async-rust.md) · [Building a virtual machine monitor in Rust](workshops-building-a-virtual-machine-monitor-in-rust.md) · [C→Rust, Beyond the Basics](workshops-c-to-rust.md) · [Containers Are Dead: Long Live WebAssembly](workshops-containers-are-dead.md) · [Hands-On Embedded Rust](workshops-hands-on-embedded-rust.md) · [Rust in Action - Applications of Computer Vision and Control](workshops-rust-in-action.md) · [Rust on the Field: Integrating Real Autonomous Systems](workshops-rust-on-the-field-integrating-real-autonomous-systems.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

### [Programming a line follower robot](https://rustlab.it/talks/programming-a-line-follower-robot)

Workshop · Massimiliano Mantione, Michele Mantione · public source page

**Repeat appearance.** Shows an earlier workshop by the same mentors on simulated robots, PID control and telemetry.

[RustLab 2025](https://rustlab.it/talks/programming-a-line-follower-robot) · 2025-11-02 · Delivery unverified.

Date comes from the page’s embedded event data; current site branding says 2026. Delivery and recording are unverified.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/programming-a-line-follower-robot).

### [Accurate Robot Simulations with Bevy and WASM](https://rustlab.it/talks/accurate-robot-simulations-with-bevy-and-wasm-workshop)

Workshop · Massimiliano Mantione, Michele Mantione · registration

**Suggested next step.** Extends line-follower control into deterministic simulation, telemetry and separate WASM robot logic.

[RustLab 2026](https://rustlab.it/talks/accurate-robot-simulations-with-bevy-and-wasm-workshop) · 2026-11-01 · Announced.

Announced workshop on November 1, 2026; no recording is claimed.

Listing checked: 2026-10-15. Evidence: [rustlab.it](https://rustlab.it/talks/accurate-robot-simulations-with-bevy-and-wasm-workshop).

## Read, watch, and try

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

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/workshops-programming-a-line-follower-robot.json)
