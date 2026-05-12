use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "codeseed",
    version,
    about = "Manage agent skills inside a project.",
    long_about = "Codeseed initializes and manages project-local agent skills, compatibility links, diagnostics, and reproducible skill state."
)]
pub struct Cli {
    /// Project root to operate on.
    #[arg(long, global = true, value_name = "DIR", default_value = ".")]
    pub project: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize Codeseed in the current or specified project directory.
    Init(InitCommand),

    /// Add a skill from presets, SkillHub, GitHub, URL, file, or directory.
    Add(AddCommand),

    /// List available or installed skills.
    List(ListCommand),

    /// Update the codeseed executable itself.
    Update(UpdateCommand),

    /// Remove an installed Codeseed-managed skill.
    #[command(name = "rm", alias = "remove")]
    Remove(RemoveCommand),

    /// Diagnose Codeseed state, skill manifests, and compatibility links.
    Doctor(DoctorCommand),

    /// Reconcile generated files from recorded Codeseed state.
    Sync(SyncCommand),

    /// Clear Codeseed-managed state and generated agent content.
    Clear(ClearCommand),
}

#[derive(Debug, Args)]
pub struct InitCommand {
    /// Agent-facing directory to create and manage.
    #[arg(long, value_name = "DIR", default_value = ".agent")]
    pub agent_dir: PathBuf,

    /// Codeseed metadata directory to create and manage.
    #[arg(long, value_name = "DIR", default_value = ".codeseed")]
    pub codeseed_dir: PathBuf,

    /// Skip installing bundled preset skills.
    #[arg(long)]
    pub no_presets: bool,

    /// Skip creating compatibility links for known agents.
    #[arg(long)]
    pub no_links: bool,

    /// Language for generated instructions and installed preset skill documents.
    #[arg(long, value_enum, default_value = "en")]
    pub language: InitLanguage,

    /// Overwrite incompatible generated Codeseed state when possible.
    #[arg(long, short)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct AddCommand {
    /// Skill source. This can be preset:<id>, a SkillHub id, URL, local file, local directory, or GitHub reference.
    #[arg(value_name = "SOURCE")]
    pub source: String,

    /// Resolve the source from a specific SkillHub endpoint instead of the default SkillHub.
    #[arg(long, value_name = "URL")]
    pub hub: Option<String>,

    /// Install the skill under a specific local name.
    #[arg(long, value_name = "NAME")]
    pub name: Option<String>,

    /// Override the placement target inferred from the skill manifest.
    #[arg(long, value_enum)]
    pub target: Option<SkillTarget>,

    /// Override the destination directory for this skill.
    #[arg(long, value_name = "DIR")]
    pub target_dir: Option<PathBuf>,

    /// Replace an existing managed skill with the same id or name.
    #[arg(long, short)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct ListCommand {
    /// List installed project skills instead of built-in available skills.
    #[arg(long)]
    pub installed: bool,

    /// Output format.
    #[arg(long, value_enum, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct UpdateCommand {
    /// Codeseed version to install.
    #[arg(long, value_name = "VERSION", default_value = "latest")]
    pub version: String,

    /// Codeseed home directory. Defaults to ~/.codeseed in the installer.
    #[arg(long, value_name = "DIR")]
    pub home: Option<PathBuf>,

    /// Directory for the codeseed executable. Defaults to ~/.codeseed/bin in the installer.
    #[arg(long, value_name = "DIR")]
    pub bin_dir: Option<PathBuf>,

    /// Installer strategy.
    #[arg(long, value_enum, default_value = "auto")]
    pub mode: UpdateMode,

    /// Install script URL used when no local installer is available.
    #[arg(
        long,
        value_name = "URL",
        default_value = "https://raw.githubusercontent.com/seedware/codeseed/refs/heads/main/scripts/install.sh"
    )]
    pub script_url: String,

    /// Show the update plan without executing it.
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Args)]
pub struct RemoveCommand {
    /// Installed skill id or name.
    #[arg(value_name = "SKILL")]
    pub skill: String,

    /// Remove generated files even when the local state is partially inconsistent.
    #[arg(long, short)]
    pub force: bool,

    /// Also remove empty generated parent directories after the skill is removed.
    #[arg(long)]
    pub prune: bool,
}

#[derive(Debug, Args)]
pub struct DoctorCommand {
    /// Treat warnings as failures.
    #[arg(long)]
    pub strict: bool,

    /// Attempt safe repairs for missing generated directories or compatibility links.
    #[arg(long)]
    pub fix: bool,

    /// Output format.
    #[arg(long, value_enum, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct SyncCommand {
    /// Show planned changes without modifying files.
    #[arg(long)]
    pub dry_run: bool,

    /// Check whether generated files match recorded state, without repairing them.
    #[arg(long, conflicts_with = "dry_run")]
    pub check: bool,

    /// Remove stale generated files when they are still owned by Codeseed.
    #[arg(long)]
    pub prune: bool,
}

#[derive(Debug, Args)]
pub struct ClearCommand {
    /// Agent-facing directory to remove from the target project.
    #[arg(long, value_name = "DIR", default_value = ".agent")]
    pub agent_dir: PathBuf,

    /// Codeseed metadata directory to remove from the target project.
    #[arg(long, value_name = "DIR", default_value = ".codeseed")]
    pub codeseed_dir: PathBuf,

    /// Show what would be removed without modifying files.
    #[arg(long)]
    pub dry_run: bool,

    /// Skip the interactive confirmation prompt.
    #[arg(long, conflicts_with = "dry_run", requires = "confirm")]
    pub yes: bool,

    /// Required phrase when using --yes. Must be clear-codeseed-state.
    #[arg(
        long,
        value_name = "PHRASE",
        conflicts_with = "dry_run",
        requires = "yes",
        value_parser = ["clear-codeseed-state"]
    )]
    pub confirm: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum InitLanguage {
    #[value(name = "en")]
    En,
    #[value(name = "zh-CN", alias = "zh-cn", alias = "zh")]
    ZhCn,
}

impl InitLanguage {
    pub fn as_state_value(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::ZhCn => "zh-CN",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum SkillTarget {
    Common,
    Codex,
    Claude,
    Cursor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum UpdateMode {
    Auto,
    Local,
    Prebuilt,
}

#[cfg(test)]
mod tests {
    use clap::{CommandFactory, Parser};

    use super::{Cli, Command, InitLanguage, OutputFormat, SkillTarget, UpdateMode};

    #[test]
    fn clap_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_init_defaults() {
        let cli = Cli::parse_from(["codeseed", "init"]);

        let Command::Init(command) = cli.command else {
            panic!("expected init command");
        };

        assert_eq!(cli.project.to_string_lossy(), ".");
        assert_eq!(command.agent_dir.to_string_lossy(), ".agent");
        assert_eq!(command.codeseed_dir.to_string_lossy(), ".codeseed");
        assert!(!command.no_presets);
        assert!(!command.no_links);
        assert_eq!(command.language, InitLanguage::En);
        assert!(!command.force);
    }

    #[test]
    fn parses_init_language() {
        let cli = Cli::parse_from(["codeseed", "init", "--language", "zh-CN"]);

        let Command::Init(command) = cli.command else {
            panic!("expected init command");
        };

        assert_eq!(command.language, InitLanguage::ZhCn);
    }

    #[test]
    fn parses_add_with_overrides() {
        let cli = Cli::parse_from([
            "codeseed",
            "add",
            "github:seedware/example-skill",
            "--hub",
            "https://skills.example.com",
            "--name",
            "example",
            "--target",
            "codex",
            "--target-dir",
            ".agent/skills/codex",
            "--force",
        ]);

        let Command::Add(command) = cli.command else {
            panic!("expected add command");
        };

        assert_eq!(command.source, "github:seedware/example-skill");
        assert_eq!(command.hub.as_deref(), Some("https://skills.example.com"));
        assert_eq!(command.name.as_deref(), Some("example"));
        assert_eq!(command.target, Some(SkillTarget::Codex));
        assert_eq!(
            command.target_dir.as_ref().unwrap().to_string_lossy(),
            ".agent/skills/codex"
        );
        assert!(command.force);
    }

    #[test]
    fn parses_list_defaults_to_available_text() {
        let cli = Cli::parse_from(["codeseed", "list"]);

        let Command::List(command) = cli.command else {
            panic!("expected list command");
        };

        assert!(!command.installed);
        assert_eq!(command.format, OutputFormat::Text);
    }

    #[test]
    fn parses_list_installed_json() {
        let cli = Cli::parse_from(["codeseed", "list", "--installed", "--format", "json"]);

        let Command::List(command) = cli.command else {
            panic!("expected list command");
        };

        assert!(command.installed);
        assert_eq!(command.format, OutputFormat::Json);
    }

    #[test]
    fn parses_update_defaults() {
        let cli = Cli::parse_from(["codeseed", "update"]);

        let Command::Update(command) = cli.command else {
            panic!("expected update command");
        };

        assert_eq!(command.version, "latest");
        assert_eq!(command.home, None);
        assert_eq!(command.bin_dir, None);
        assert_eq!(command.mode, UpdateMode::Auto);
        assert!(!command.dry_run);
        assert!(command.script_url.contains("scripts/install.sh"));
    }

    #[test]
    fn parses_update_overrides() {
        let cli = Cli::parse_from([
            "codeseed",
            "update",
            "--version",
            "v0.2.0",
            "--home",
            "/tmp/codeseed",
            "--bin-dir",
            "/tmp/codeseed/bin",
            "--mode",
            "prebuilt",
            "--dry-run",
        ]);

        let Command::Update(command) = cli.command else {
            panic!("expected update command");
        };

        assert_eq!(command.version, "v0.2.0");
        assert_eq!(
            command.home.as_ref().unwrap().to_string_lossy(),
            "/tmp/codeseed"
        );
        assert_eq!(
            command.bin_dir.as_ref().unwrap().to_string_lossy(),
            "/tmp/codeseed/bin"
        );
        assert_eq!(command.mode, UpdateMode::Prebuilt);
        assert!(command.dry_run);
    }

    #[test]
    fn parses_rm_alias() {
        let cli = Cli::parse_from(["codeseed", "remove", "writer", "--prune"]);

        let Command::Remove(command) = cli.command else {
            panic!("expected remove command");
        };

        assert_eq!(command.skill, "writer");
        assert!(command.prune);
    }

    #[test]
    fn parses_rm_primary_name() {
        let cli = Cli::parse_from(["codeseed", "rm", "writer"]);

        let Command::Remove(command) = cli.command else {
            panic!("expected remove command");
        };

        assert_eq!(command.skill, "writer");
    }

    #[test]
    fn parses_doctor_json_strict() {
        let cli = Cli::parse_from(["codeseed", "doctor", "--format", "json", "--strict"]);

        let Command::Doctor(command) = cli.command else {
            panic!("expected doctor command");
        };

        assert_eq!(command.format, OutputFormat::Json);
        assert!(command.strict);
    }

    #[test]
    fn parses_clear_defaults_to_interactive_confirmation() {
        let cli = Cli::parse_from(["codeseed", "clear"]);

        let Command::Clear(command) = cli.command else {
            panic!("expected clear command");
        };

        assert_eq!(command.agent_dir.to_string_lossy(), ".agent");
        assert_eq!(command.codeseed_dir.to_string_lossy(), ".codeseed");
        assert!(!command.dry_run);
        assert!(!command.yes);
        assert_eq!(command.confirm, None);
    }

    #[test]
    fn parses_clear_non_interactive_confirmation() {
        let cli = Cli::parse_from([
            "codeseed",
            "clear",
            "--yes",
            "--confirm",
            "clear-codeseed-state",
        ]);

        let Command::Clear(command) = cli.command else {
            panic!("expected clear command");
        };

        assert!(command.yes);
        assert_eq!(command.confirm.as_deref(), Some("clear-codeseed-state"));
    }

    #[test]
    fn rejects_clear_yes_without_confirmation_phrase() {
        let result = Cli::try_parse_from(["codeseed", "clear", "--yes"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_clear_confirmation_phrase_without_yes() {
        let result =
            Cli::try_parse_from(["codeseed", "clear", "--confirm", "clear-codeseed-state"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_clear_wrong_confirmation_phrase() {
        let result = Cli::try_parse_from(["codeseed", "clear", "--yes", "--confirm", "wrong"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_clear_dry_run_with_yes() {
        let result = Cli::try_parse_from([
            "codeseed",
            "clear",
            "--dry-run",
            "--yes",
            "--confirm",
            "clear-codeseed-state",
        ]);

        assert!(result.is_err());
    }
}
