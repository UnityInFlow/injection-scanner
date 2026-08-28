# Release Checklist

The real tag-to-verified-binaries procedure, read out of `.github/workflows/release.yml`. Follow
this document instead of reading the workflow from scratch each time.

## 1. Pre-tag gate (local)

Run the same four checks the `Test gate (test + clippy + fmt)` CI job runs, in this order:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo build --release --locked
```

Then, before tagging:

1. Bump `version` in `Cargo.toml` to the new release version.
2. Refresh `Cargo.lock` — run a build (`cargo build --release --locked` again, or `cargo check`) so
   the lockfile records the new version, and commit the updated `Cargo.lock` alongside the bump.
3. Update `CHANGELOG.md` — promote `[Unreleased]` to a new `## [X.Y.Z] - YYYY-MM-DD` heading with
   today's date, add the matching compare link at the bottom of the file, and leave a fresh empty
   `[Unreleased]` section above it.

**Why the version bump is its own checklist item, not folklore:** the `test` job in `release.yml`
reads the crate version out of `cargo metadata` (selected by package name `injection-scanner`, not
by index) and compares it against `${GITHUB_REF_NAME#v}`. If they disagree, the job fails loudly
and the release stops there — but that guard was added after a tag was pushed without a matching
bump, which built and shipped binaries reporting the *previous* version, with a signed provenance
attestation over them, past every other gate. Do this step; do not rely on remembering it.

## 2. Tag and push

```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

The workflow trigger is `on: push: tags: ['v*']` and nothing else. A fork cannot push a tag to this
repository, so nothing else can start a release.

## 3. Watch the four jobs, by name, in dependency order

1. **`test`** ("Test gate (test + clippy + fmt)") — the tag-vs-`Cargo.toml` guard, then
   `cargo test --locked`, `cargo clippy -- -D warnings`, `cargo fmt --check`.
2. **`build-binaries`** ("Build ${{ matrix.target }}", `needs: test`) — six matrix legs, one Linux
   host cross-compiling all of them via `cargo zigbuild`.
3. **`release`** ("Create GitHub Release", `needs: [build-binaries]`) — validates the four required
   Linux assets, generates `SHA256SUMS.txt`, signs build provenance, creates the Release.
4. **`verify-published-assets`** ("Verify published asset contract (spec-ci-plugin)",
   `needs: [release]`) — runs after publication and walks the consumer's exact download path.

**Which failures are fatal:** any of the four Linux legs (`x86_64-unknown-linux-musl`,
`aarch64-unknown-linux-musl`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`) failing is
fatal — the `release` job hard-requires all four and will not publish without them. The two
`*-apple-darwin` legs are `continue-on-error: true` and marked `experimental`; a failure there is a
warning that defers macOS to a later release, not a broken release.

## 4. What is published

| Target triple | Asset name | Required? |
|---|---|---|
| `x86_64-unknown-linux-musl` | `injection-scanner-x86_64-unknown-linux-musl` | hard-required (consumer) |
| `aarch64-unknown-linux-musl` | `injection-scanner-aarch64-unknown-linux-musl` | hard-required (consumer) |
| `x86_64-unknown-linux-gnu` | `injection-scanner-x86_64-unknown-linux-gnu` | hard-required |
| `aarch64-unknown-linux-gnu` | `injection-scanner-aarch64-unknown-linux-gnu` | hard-required |
| `x86_64-apple-darwin` | `injection-scanner-x86_64-apple-darwin` | conditional (`continue-on-error`) |
| `aarch64-apple-darwin` | `injection-scanner-aarch64-apple-darwin` | conditional |

Plus `SHA256SUMS.txt`, generated over every raw binary present in the artifact set.

## 5. Post-release verification (manual, on the published Release)

`verify-published-assets` already walks the consumer's exact path automatically. This manual pass
is the human confirmation that it did — and the place to notice a red job before treating the
release as done.

```bash
curl -fsSL -o injection-scanner-x86_64-unknown-linux-musl \
  https://github.com/UnityInFlow/injection-scanner/releases/download/vX.Y.Z/injection-scanner-x86_64-unknown-linux-musl

gh attestation verify injection-scanner-x86_64-unknown-linux-musl \
  --repo UnityInFlow/injection-scanner

curl -fsSL -o SHA256SUMS.txt \
  https://github.com/UnityInFlow/injection-scanner/releases/download/vX.Y.Z/SHA256SUMS.txt
sha256sum -c --ignore-missing SHA256SUMS.txt

chmod +x injection-scanner-x86_64-unknown-linux-musl
./injection-scanner-x86_64-unknown-linux-musl --version   # confirm it prints vX.Y.Z
```

## 6. Consumer check against `spec-ci-plugin`

The `spec-ci-plugin` GitHub Action (`04-spec-ci-plugin/src/injection-scanner.ts`) downloads the two
musl assets at a **pinned tag**, verifies them against `SHA256SUMS.txt`, `chmod +x`es them, and
executes them. After a release:

1. Confirm both musl URLs return `200` at the new tag — the `verify-published-assets` job already
   did this, but re-check manually if that job did not run or is stale.
2. Decide whether `spec-ci-plugin`'s default `injection-scanner-version` should move to the new tag.
   It currently pins `v0.0.3`. A release is not finished until this is answered either way, even if
   the answer is "not yet."

## 7. Two hard constraints — do not change these back

- **The musl assets stay raw, unextensioned, and target-triple-named.** Not tarballs, no extension,
  exact names. `spec-ci-plugin` curls and executes them directly, so these names are a public API of
  this repository; renaming one is a breaking change that surfaces in *another* repository's CI, not
  this one. `tests/release_contract_test.rs` and the `verify-published-assets` job both defend this
  and see different failures — see `CONTRIBUTING.md` §"The release asset contract".
- **`release.yml` runs on `ubuntu-latest` deliberately.** This repository is public, and the org
  runner group enforces `allows_public_repositories: false`, so a self-hosted job cannot be
  scheduled here at all — a `runs-on: [orangepi]` label would queue until cancelled. The pipeline
  uses no org secrets, only the built-in `GITHUB_TOKEN`, and is tag-triggered only, which a fork
  cannot fire. See the August 2026 row in the root `CLAUDE.md` decisions log and issue #45. Do not
  "restore" a self-hosted label here or in `ci.yml`.

## 8. Rollback

A bad release is fixed by pushing a **new** tag, not by mutating a published one. Deleting or moving
a tag breaks the pinned URL `spec-ci-plugin` fetches and invalidates the build-provenance
attestation, which is bound to `refs/tags/<tag>`.
