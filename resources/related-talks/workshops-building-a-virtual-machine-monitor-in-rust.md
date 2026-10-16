# Building a virtual machine monitor in Rust

[Official EuroRust session](https://eurorust.eu/workshops/building-a-virtual-machine-monitor-in-rust/) · [All session resources](README.md)

Curated from primary pages, abstracts and project descriptions. Books, recordings and code were not fully read, watched or executed. No verified future conference match is included in this pass.

Speakers and their work: [Johan Sannemo](../speakers/johan-sannemo.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Build a Linux-booting virtual-machine monitor while learning virtualization hardware, KVM interfaces, and device behavior.
- **Prerequisites:** Comfort writing Rust; virtualization background is not required.
- **Setup:** Bring a laptop with an SSH client; the organizer describes provided cloud instances with nested virtualization.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [The KVM API](https://docs.kernel.org/virt/kvm/api.html). Defines the VM, vCPU and memory interfaces used when building a Linux VMM.
- **Bring a question:** What confirms the intended configuration became the actual running state?
- **Other sessions starting at this time:** [Beyond p99: Observing Async Rust](workshops-beyond-p99-observing-async-rust.md) · [C→Rust, Beyond the Basics](workshops-c-to-rust.md) · [Containers Are Dead: Long Live WebAssembly](workshops-containers-are-dead.md) · [Hands-On Embedded Rust](workshops-hands-on-embedded-rust.md) · [Programming a Line Follower Robot](workshops-programming-a-line-follower-robot.md) · [Rust in Action - Applications of Computer Vision and Control](workshops-rust-in-action.md) · [Rust on the Field: Integrating Real Autonomous Systems](workshops-rust-on-the-field-integrating-real-autonomous-systems.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

### [Firecracker: Lightweight Virtualization for Serverless Applications](https://www.usenix.org/conference/nsdi20/presentation/agache)

Paper · Alexandru Agache, Marc Brooker, Andreea Florescu, Alexandra Iordache, Anthony Liguori, Rolf Neugebauer, Phil Piwonka, Diana-Maria Popa · open access

**Same topic.** Explains the device, isolation and overhead choices behind a production microVM monitor.

[USENIX NSDI 2020](https://www.usenix.org/conference/nsdi20/presentation/agache) · Published.

The publisher page includes the paper, slides and a recording; results describe the 2020 system.

Listing checked: 2026-10-15. Evidence: [www.usenix.org](https://www.usenix.org/conference/nsdi20/presentation/agache).

## Read, watch, and try

### [The KVM API](https://docs.kernel.org/virt/kvm/api.html)

Documentation · Linux kernel contributors · open access

**Preparation.** Defines the VM, vCPU and memory interfaces used when building a Linux VMM.

Source page checked: 2026-10-15. Evidence: [docs.kernel.org](https://docs.kernel.org/virt/kvm/api.html).

### [rust-vmm reference VMM](https://github.com/rust-vmm/vmm-reference)

Project · rust-vmm contributors · open access

**Suggested next step.** Shows how reusable Rust virtualization components fit together in a small VMM.

Reference implementation; no build or guest boot was performed.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/rust-vmm/vmm-reference).

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/workshops-building-a-virtual-machine-monitor-in-rust.json)
