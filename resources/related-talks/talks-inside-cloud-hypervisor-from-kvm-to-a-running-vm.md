# Inside Cloud Hypervisor: From KVM to a Running VM

[Official EuroRust session](https://eurorust.eu/talks/inside-cloud-hypervisor-from-kvm-to-a-running-vm/) · [All session resources](README.md)

Selected for this session after checking primary source pages. Connections are editorial judgments, not speaker endorsements. Books, papers, and recordings were not reviewed in full; listing checks and access limits are noted below.

Speakers and their work: [Philipp Schuster](../speakers/philipp-schuster.md).

## Decide and prepare

These preparation suggestions are editorial. Check the official description for fit and access.

- **Focus:** Trace VM boot through memory, virtual CPUs, devices, and kernel boundaries while examining the development workflow of a Rust hypervisor.
- **Prerequisites:** Not established; ask the organizer.
- **Setup:** Not established; check the session page.
- **Access:** Ticket conditions unconfirmed; check the official page. Remote participation: unconfirmed.
- **Start with:** [Quickly Boot a Minimal Linux Kernel + Initrd in Cloud Hypervisor or QEMU](https://phip1611.de/blog/quickly-boot-a-minimal-linux-kernel-initrd-in-cloud-hypervisor-or-qemu/). Build the kernel and initrd inputs needed to follow the session's VM boot sequence.
- **Bring a question:** What confirms the intended configuration became the actual running state?
- **Other sessions starting at this time:** [What's next for Rust's type system](talks-what-s-next-for-rust-s-type-system.md). Choose one; check its full time and access requirements.

After studying or attending, record one sourced claim, your interpretation, and the next test in a [session note](../../templates/session.md).

## Related talks and workshops

### [Firecracker: Lightweight Virtualization for Serverless Applications](https://www.usenix.org/conference/nsdi20/presentation/agache)

Paper · Alexandru Agache, Marc Brooker, Andreea Florescu, Alexandra Iordache, Anthony Liguori, Rolf Neugebauer, Phil Piwonka, Diana-Maria Popa · open access

**Alternative approach.** Compare a serverless VMM's isolation and device choices with Cloud Hypervisor.

[USENIX NSDI 2020](https://www.usenix.org/conference/nsdi20/presentation/agache) · Published.

The proceedings page links the paper, slides, and recording; the full paper and video were not reviewed.

Source page checked: 2026-10-15. Evidence: [www.usenix.org](https://www.usenix.org/conference/nsdi20/presentation/agache).

## Read, watch, and try

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

### [Cloud Hypervisor](https://github.com/cloud-hypervisor/cloud-hypervisor)

Project · Cloud Hypervisor contributors · open access

**Suggested next step.** Follow the VM monitor's current architecture, guest boot instructions, and device support.

Source page checked: 2026-10-15. Evidence: [github.com](https://github.com/cloud-hypervisor/cloud-hypervisor).

Source review: **2026-10-15**. Draft checked against source; community review pending. Connections are editorial suggestions.

[Sources and reuse](../../docs/source-and-reuse-policy.md) · [Suggest a correction](../../CONTRIBUTING.md#learning-resources-and-archives) · [Structured source](../../community/session-resources/talks-inside-cloud-hypervisor-from-kvm-to-a-running-vm.json)
