# Optional: use the local itinerary helper

Use this helper to draft or check a JSON itinerary. For planning without it,
open the [planning guide](planning-guide.md) or [agent guide](agent-guide.md).

Try a supplied profile, inspect the choices, then change the inputs.

## Prerequisites and tested environments

- Git and stable Rust. The workspace declares Rust 1.88 minimum. This walkthrough
  was tested with Rust/Cargo 1.98.0 on macOS arm64; the minimum was not retested.
- Network access for cloning and uncached build dependencies. After building,
  `validate`, `recommend`, and `check-itinerary` use local files, with no model
  provider, credentials, or running service.
- Start in your chosen parent directory. The clone command enters the repository;
  run later commands from its root.

[CI](../.github/workflows/ci.yml) tests Ubuntu with stable Rust.
This does not establish Windows or universal support.
For native downloads, see [release assets](#use-a-release-download).

## Generate the example

```sh
git clone https://github.com/hghalebi/eurorust-2026.git
cd eurorust-2026
cargo build --release --locked --package conference-data
mkdir -p .cache/quickstart
./target/release/conference-data validate
./target/release/conference-data recommend \
  --profile community/paths/backend-profile.json \
  --output .cache/quickstart/itinerary.json \
  --markdown .cache/quickstart/itinerary.md
./target/release/conference-data check-itinerary \
  --itinerary .cache/quickstart/itinerary.json
```

Recorded on October 14, 2026, with snapshot
`eurorust-2026-bd9ff3d15e58a1b2d39f`:

```text
Validated eurorust-2026-bd9ff3d15e58a1b2d39f: 53 entries
Wrote 18 selections to .cache/quickstart/itinerary.json
NOTE: Prerequisite readiness and ticket availability require participant confirmation.
Schedule feasibility checked; no booking or attendance claim
```

Counts and IDs may change after refresh. JSON records the profile, snapshot,
choices, reasons, and alternatives. Markdown adds official links, times, source
dates, and preparation. Git ignores `.cache/`.

## Use your own profile

Adapt a [fictional profile](../community/paths/README.md), or save this as
`.cache/quickstart/my-profile.json`:

```json
{
  "name": "Fictional one-day infrastructure participant",
  "goals": ["Explore infrastructure configuration and production failure handling"],
  "topics": ["infrastructure", "production-reliability", "security"],
  "available_windows": [
    {"start": "2026-10-16T10:00:00+02:00", "end": "2026-10-16T18:00:00+02:00"}
  ],
  "registered_workshops": [],
  "attendance_mode": "in_person"
}
```

```sh
./target/release/conference-data recommend \
  --profile .cache/quickstart/my-profile.json \
  --output .cache/quickstart/my-itinerary.json \
  --markdown .cache/quickstart/my-itinerary.md
./target/release/conference-data check-itinerary \
  --itinerary .cache/quickstart/my-itinerary.json
```

This example produced eight checked selections in the recorded snapshot.
It covers October 16 and excludes workshops because none is registered.
The [three supplied profiles](../community/paths/README.md) also produced
different choices from the same data.

Change `topics` to affect ranking; use existing [annotation tags](../community/annotations/sessions.json).
The CLI does not interpret prose `goals`. Confirm time windows and bookings.
Empty availability means unknown, so eligible slots remain conditional.
Keep personal inputs local. See the [field reference](cli.md#profile-fields).

## Reuse the owner example

Generate a copy in the ignored cache:

```sh
mkdir -p .cache/quickstart
./target/release/conference-data recommend --profile my-conference/profile.json --output .cache/quickstart/owner-itinerary.json --markdown .cache/quickstart/owner-itinerary.md
./target/release/conference-data check-itinerary --itinerary .cache/quickstart/owner-itinerary.json
```

The owner's availability and attendance mode are unknown, so this plan is
conditional. Use your own local profile to adapt it.

## Install a reusable binary

After building, install into the repository's ignored `bin/` directory:

```sh
mkdir -p bin
install -m 755 target/release/conference-data bin/conference-data
bin/conference-data --version
bin/conference-data validate
```

Run `bin/conference-data` from the repository root, as the other guides do.
This installation does not change your shell's `PATH`.

## Use a release download

[Release v0.1.0](https://github.com/hghalebi/eurorust-2026/releases/tag/v0.1.0)
contains `eurorust-2026-context.zip`, macOS arm64 tools in
`eurorust-tools-macos-arm64.tar.gz`, and `SHA256SUMS`.
Unpack context and tools separately; verify archive hashes against `SHA256SUMS`.
The tools include `USAGE.md` and `build-info.json`.

Run the extracted executable with `--root` set to the context directory,
then `validate`. This checks the downloaded context.
Releases are frozen and may differ from current `main`.
There are no prebuilt Linux or Windows binaries in this release.

## If a step fails

| Symptom | Check |
| --- | --- |
| Executable missing | Build it or use the release for its stated platform |
| Profile or output path missing | Create the output directory; run from the repository root. `--root` changes dataset lookup, not these paths |
| Export digest mismatch | Use [snapshot recovery](cli.md#recover-after-a-refresh-or-merge); preserve immutable sources |
| Different snapshot | Use the plan's retained context or regenerate from the active bundle |
| Zero selections | Check time windows and workshop IDs; only talks and registered workshops qualify |
| Access, readiness, or travel warning | Confirm the external fact; overlap checks cannot establish it |

See the [CLI reference](cli.md) for flags and errors, or
[maintenance guide](maintenance.md) for collection changes.
