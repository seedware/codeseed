# Installing Codeseed

Chinese version: [install.zh-CN.md](install.zh-CN.md).

Codeseed installs into the user's home directory by default:

```text
~/.codeseed/
├── bin/
│   └── codeseed
├── cache/
└── config/
```

`~/.codeseed/bin/codeseed` is the executable. Future global configuration, such as default SkillHub settings, can live under `~/.codeseed/config/`.

## Install From A Shell Script

```bash
curl -fsSL https://raw.githubusercontent.com/seedware/codeseed/refs/heads/main/scripts/install.sh | sh
```

The installer checks the current environment:

1. If it is executed inside a Codeseed source checkout and `cargo` is available, it runs `cargo build --release`.
2. Otherwise it tries to download a prebuilt binary for macOS or Linux.
3. If no prebuilt binary is available and both `git` and `cargo` are available, it clones the repository and builds from source.

## Local Source Install

From this repository:

```bash
./scripts/install.sh --local
```

## Update Codeseed

Use the CLI update command to update the `codeseed` executable itself. Run a dry run first when you want to inspect the installer plan:

```bash
codeseed update --dry-run
codeseed update
```

`codeseed update` reuses the local `scripts/install.sh` when it is run from a Codeseed source checkout. Otherwise it downloads and runs the configured install script. The default installer target remains `~/.codeseed/bin/codeseed`.

The shell-script installer can also be rerun directly:

```bash
curl -fsSL https://raw.githubusercontent.com/seedware/codeseed/refs/heads/main/scripts/install.sh | sh
```

Updating the executable does not automatically rewrite project-local skills. Refresh project skills separately from the project root; see [skill-layout.md](skill-layout.md#refreshing-project-skills).

## Options

```bash
./scripts/install.sh --version latest
./scripts/install.sh --home "$HOME/.codeseed"
./scripts/install.sh --bin-dir "$HOME/.codeseed/bin"
./scripts/install.sh --prebuilt
```

After installation, make sure `~/.codeseed/bin` is in `PATH`:

```bash
export PATH="$HOME/.codeseed/bin:$PATH"
```
