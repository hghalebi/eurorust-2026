# Optional: use the local itinerary helper

This walkthrough is for readers who want a command-line draft or a local check
of an itinerary in JSON. You can use the [planning guide](planning-guide.md),
read the notes, and [work with your agent](agent-guide.md) directly from the
conference files without this helper.

To use the helper, run a supplied profile first, inspect the choices, then change
the inputs that matter to you. The following sections cover installation and
the optional workflow in full.

## Prerequisites and tested environments

- Git and a current stable Rust toolchain for a source build. The workspace
  declares Rust 1.88 as its minimum; this walkthrough was executed with Rust and
  Cargo 1.98.0 on macOS arm64. The minimum toolchain was not separately retested.
- Network access for cloning and any uncached build dependencies. After building,
  `validate`, `recommend`, and `check-itinerary` use local files. They call no model
  provider and require no provider credentials or running service.
- For a new checkout, start in a directory where you want the repository. The
  clone sequence enters its root; run all later commands there.

[CI](../.github/workflows/ci.yml) also builds and tests on Ubuntu with stable Rust.
That is evidence for the configured environment, not a Windows or universal
platform support guarantee. For a native download, see [release assets](#use-a-release-download).

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

Counts and snapshot IDs can change after a refresh. Read your actual output.
The JSON records the profile, snapshot, selected session IDs, reasons, and
alternatives. The Markdown adds official links, exact times, source dates, and
preparation notes. `.cache/` is ignored by Git.

## Use your own profile

Start with a [fictional profile](../community/paths/README.md), or save this
complete example as `.cache/quickstart/my-profile.json`:

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
It limits the plan to October 16 and excludes workshops because none is listed
as registered. Three supplied profiles also produced different session choices
from the same data; see [the examples](../community/paths/README.md).

Change `topics` to influence CLI ranking. `goals` preserves your intent for people
and agents; the deterministic CLI does not interpret that prose. Use known topic
tags from [annotations](../community/annotations/sessions.json), and confirm your
actual time windows and registrations. Empty availability means unknown, so all
eligible slots are considered conditionally. Keep real participant inputs local.
The [CLI reference](cli.md#profile-fields) describes every field.

## Reuse the owner example

To reproduce the published owner example locally, use its existing profile.
These commands keep your copy in the ignored cache:

```sh
mkdir -p .cache/quickstart
./target/release/conference-data recommend --profile my-conference/profile.json --output .cache/quickstart/owner-itinerary.json --markdown .cache/quickstart/owner-itinerary.md
./target/release/conference-data check-itinerary --itinerary .cache/quickstart/owner-itinerary.json
```

The owner profile leaves availability and attendance mode unknown, so the
result remains conditional. To adapt the choices, use your own local profile.

## Install a reusable binary

After the source build above, install the tool into the repository's ignored
`bin/` directory:

```sh
mkdir -p bin
install -m 755 target/release/conference-data bin/conference-data
bin/conference-data --version
bin/conference-data validate
```

This is a repository-local installation, not a change to your shell's `PATH`.
Run `bin/conference-data` from the repository root. The other guides use this
form for repeated work.

## Use a release download

[Release v0.1.0](https://github.com/hghalebi/eurorust-2026/releases/tag/v0.1.0)
contains `eurorust-2026-context.zip`, macOS arm64 executables in
`eurorust-tools-macos-arm64.tar.gz`, and `SHA256SUMS`. Unpack the context and tools
into separate directories and verify the archive hashes against `SHA256SUMS`.
The tools archive includes `USAGE.md` and `build-info.json`.

Invoke the extracted `conference-data` executable with `--root` pointing to the
unpacked context directory, followed by `validate`. This validates that downloaded
context. A release is a frozen snapshot; its commit, data, and copy may differ
from current `main`. The release offers no prebuilt Linux or Windows binary.

## If a step fails

| What you see | What to check |
| --- | --- |
| Executable missing | Complete the source build or use the native release for its stated platform |
| Profile or output path missing | Create the output directory; run from the repository root. `--root` selects dataset files, not profile/output path resolution |
| Export digest mismatch | A view differs from its manifest. Use [snapshot recovery](cli.md#recover-after-a-refresh-or-merge); preserve the immutable source |
| Different snapshot | Use the plan's retained snapshot deliberately, or regenerate from the active bundle |
| Zero selections | Check time windows and workshop IDs. The CLI chooses talks and explicitly registered workshops |
| Access, readiness, or travel warning | Confirm the external fact yourself. A successful overlap check cannot establish it |

For exact flags and failure behavior, read the [CLI reference](cli.md). To change
collection or data rules, use the [maintenance guide](maintenance.md).
