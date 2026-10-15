# Where typed software meets a changing physical world

**Status: source-based preparation; registration unconfirmed.** Workshop:
`workshops-rust-on-the-field-integrating-real-autonomous-systems`.
[Read the official description](https://eurorust.eu/workshops/rust-on-the-field-integrating-real-autonomous-systems/).

The Quantum Systems-sponsored workshop uses MOSAIC-UXS to explore Rust connectors
for heterogeneous systems and unreliable transport. It asks for comfort with
async Rust and Tokio; previous robotics or protocol experience is not required.
It describes provided hardware and laptop setup instructions sent before the
workshop. Confirm current access and logistics with the organizer.

## Questions to bring

- How should timeouts, retries, and stale observations affect a connector's API?
- Which shared data models make failures easier to inspect?
- What evidence shows that the physical system matches the software's belief?

## Choose the workshop for your goal

The other robotics options have different teaching environments: the
line-follower exercises use simulation; the embedded workshop describes supplied
Pico hardware and USB setup. Compare goals, prerequisites, bookings, and available
time on the official pages.

Keep simulation results and physical-system validation separate. Local type
safety does not establish the state of hardware outside the process.

[Read about protocols, shutdown, and retries](../../resources/related-talks/workshops-rust-on-the-field-integrating-real-autonomous-systems.md).
