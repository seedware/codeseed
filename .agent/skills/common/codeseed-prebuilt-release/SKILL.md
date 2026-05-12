---
name: codeseed-prebuilt-release
description: Publish Codeseed prebuilt release archives for the install script. Use when building, packaging, uploading, or verifying codeseed-<target>.tar.gz release assets, initially for the current host platform.
license: MIT
compatibility: Designed for Codeseed releases that use scripts/install.sh prebuilt downloads.
metadata:
  codeseed.version: "0.1.0"
---

# Codeseed Prebuilt Release

Use this skill when publishing a Codeseed prebuilt archive consumed by `scripts/install.sh`.

This workflow intentionally starts with the current host platform only. Do not imply that other platforms were built until cross-platform packaging is added.

## Archive Contract

`scripts/install.sh` downloads release assets named:

```text
codeseed-<target>.tar.gz
```

The current target names are:

1. `aarch64-apple-darwin`
2. `x86_64-apple-darwin`
3. `aarch64-unknown-linux-gnu`
4. `x86_64-unknown-linux-gnu`

The archive must contain an executable file named `codeseed`.

## Workflow

1. Confirm the release tag or version. Prefer tags such as `v0.1.0`.
2. Check the working tree and avoid packaging unrelated local changes.
3. Run `cargo fmt --check` and `cargo test` before publishing.
4. Build and package the current host platform:

```bash
presets/skills/codeseed-prebuilt-release/scripts/package-current-target.sh
```

If the skill is already installed in the project, this equivalent path is also valid:

```bash
.agent/skills/common/codeseed-prebuilt-release/scripts/package-current-target.sh
```

5. Inspect the archive before upload:

```bash
tar -tzf dist/codeseed-<target>.tar.gz
```

6. Upload the asset to the GitHub release:

```bash
gh release upload <tag> dist/codeseed-<target>.tar.gz --clobber
```

If the release does not exist yet:

```bash
gh release create <tag> dist/codeseed-<target>.tar.gz --title <tag>
```

7. Verify the install path against the uploaded asset:

```bash
tmp_home="$(mktemp -d)"
CODESEED_HOME="$tmp_home" CODESEED_INSTALL_MODE=prebuilt ./scripts/install.sh --version <tag> --repo seedware/codeseed
"$tmp_home/bin/codeseed" --version
```

## Reporting

Report the uploaded tag, target, asset name, and install verification result. Also state plainly that only the current host platform was published when that is the case.
