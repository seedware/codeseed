use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::InitCommand;
use crate::error::{CodeseedError, Result};
use crate::presets::{DEFAULT_PRESET_SKILL_IDS, PRESET_SKILLS_DIR};

const COMMON_TARGET: &str = "common";
const AGENT_TARGETS: &[&str] = &["common", "codex", "claude", "cursor"];

#[derive(Debug, Default)]
pub struct InitReport {
    pub created_dirs: Vec<PathBuf>,
    pub installed_skills: Vec<String>,
    pub generated_files: Vec<PathBuf>,
    pub generated_links: Vec<PathBuf>,
}

pub fn run(project: &Path, command: &InitCommand) -> Result<InitReport> {
    let project = normalize_project(project);
    let agent_dir = project.join(&command.agent_dir);
    let codeseed_dir = project.join(&command.codeseed_dir);

    let mut report = InitReport::default();

    create_dir(&agent_dir, command.force, &mut report)?;
    create_dir(&codeseed_dir, command.force, &mut report)?;

    for target in AGENT_TARGETS {
        create_dir(
            agent_dir.join("skills").join(target),
            command.force,
            &mut report,
        )?;
    }
    create_dir(
        agent_dir.join("generated").join("cursor-rules"),
        command.force,
        &mut report,
    )?;

    if !command.no_presets {
        install_default_presets(&agent_dir, command.force, &mut report)?;
    }

    write_state_file(
        &codeseed_dir,
        &command.agent_dir,
        &command.codeseed_dir,
        &report,
    )?;

    if !command.no_links {
        create_compatibility_entries(&project, &agent_dir, command.force, &mut report)?;
    }

    Ok(report)
}

fn normalize_project(project: &Path) -> PathBuf {
    if project.as_os_str().is_empty() {
        current_dir_or_dot()
    } else {
        project
            .canonicalize()
            .unwrap_or_else(|_| current_dir_or_dot().join(project))
    }
}

fn current_dir_or_dot() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn display_project(project: &Path) -> String {
    if let Ok(current_dir) = std::env::current_dir() {
        if project == current_dir {
            return ".".to_string();
        }
    }
    project.display().to_string()
}

pub fn project_display_path(project: &Path) -> String {
    let project = normalize_project(project);
    display_project(&project)
}

fn report_path(path: &Path) -> PathBuf {
    if let Ok(current_dir) = std::env::current_dir() {
        if let Ok(relative) = path.strip_prefix(current_dir) {
            return relative.to_path_buf();
        }
    }
    path.to_path_buf()
}

fn push_created_dir(report: &mut InitReport, path: &Path) {
    report.created_dirs.push(report_path(path));
}

fn push_generated_file(report: &mut InitReport, path: &Path) {
    report.generated_files.push(report_path(path));
}

fn push_generated_link(report: &mut InitReport, path: &Path) {
    report.generated_links.push(report_path(path));
}

fn state_path(path: &Path) -> String {
    if path.is_absolute() {
        if let Ok(current_dir) = std::env::current_dir() {
            if let Ok(relative) = path.strip_prefix(current_dir) {
                return relative.display().to_string();
            }
        }
    }
    path.display().to_string()
}

fn symlink_target(target: &Path) -> PathBuf {
    target
        .canonicalize()
        .unwrap_or_else(|_| target.to_path_buf())
}

fn symlink_target_for_link(target: &Path, link: &Path) -> PathBuf {
    let absolute_target = symlink_target(target);
    let Some(link_parent) = link.parent() else {
        return absolute_target;
    };
    let absolute_parent = link_parent
        .canonicalize()
        .unwrap_or_else(|_| link_parent.to_path_buf());

    relative_path(&absolute_parent, &absolute_target).unwrap_or(absolute_target)
}

fn relative_path(from_dir: &Path, to_path: &Path) -> Option<PathBuf> {
    let from_components = from_dir.components().collect::<Vec<_>>();
    let to_components = to_path.components().collect::<Vec<_>>();

    if from_components.first() != to_components.first() {
        return None;
    }

    let shared_len = from_components
        .iter()
        .zip(to_components.iter())
        .take_while(|(left, right)| left == right)
        .count();

    let mut relative = PathBuf::new();
    for _ in shared_len..from_components.len() {
        relative.push("..");
    }
    for component in &to_components[shared_len..] {
        relative.push(component.as_os_str());
    }

    if relative.as_os_str().is_empty() {
        Some(PathBuf::from("."))
    } else {
        Some(relative)
    }
}

fn is_existing_correct_symlink(target: &Path, link: &Path) -> bool {
    if !is_symlink(link) {
        return false;
    }
    let Ok(existing_target) = fs::read_link(link) else {
        return false;
    };
    let desired_target = symlink_target_for_link(target, link);
    if existing_target == desired_target {
        return true;
    }
    let expected_target = symlink_target(target);
    if existing_target.is_absolute() {
        return false;
    }
    let Some(parent) = link.parent() else {
        return false;
    };
    parent.join(existing_target).canonicalize().ok() == expected_target.canonicalize().ok()
}

fn ensure_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| CodeseedError::io(parent, source))?;
    }
    Ok(())
}

fn project_relative(path: &Path, project: &Path) -> String {
    path.strip_prefix(project)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn agent_skill_path(agent_dir: &Path, skill_id: &str) -> PathBuf {
    agent_dir.join("skills").join(COMMON_TARGET).join(skill_id)
}

fn cursor_rule_path(agent_dir: &Path, skill_id: &str) -> PathBuf {
    agent_dir
        .join("generated")
        .join("cursor-rules")
        .join(format!("{skill_id}.mdc"))
}

fn claude_link_path(project: &Path, skill_id: &str) -> PathBuf {
    project.join(".claude").join("skills").join(skill_id)
}

fn cursor_link_path(project: &Path, skill_id: &str) -> PathBuf {
    project
        .join(".cursor")
        .join("rules")
        .join(format!("{skill_id}.mdc"))
}

fn agents_md_path(project: &Path) -> PathBuf {
    project.join("AGENTS.md")
}

fn codeseed_state_path(codeseed_dir: &Path) -> PathBuf {
    codeseed_dir.join("state.json")
}

fn create_dir(path: impl AsRef<Path>, force: bool, report: &mut InitReport) -> Result<()> {
    let path = path.as_ref();
    if path.is_dir() {
        return Ok(());
    }
    if path.exists() {
        if force {
            fs::remove_file(path).map_err(|source| CodeseedError::io(path, source))?;
        } else {
            return Err(CodeseedError::conflict(
                path,
                "exists and is not a directory; rerun with --force to replace it",
            ));
        }
    }
    fs::create_dir_all(path).map_err(|source| CodeseedError::io(path, source))?;
    push_created_dir(report, path);
    Ok(())
}

fn install_default_presets(agent_dir: &Path, force: bool, report: &mut InitReport) -> Result<()> {
    for skill_id in DEFAULT_PRESET_SKILL_IDS {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(PRESET_SKILLS_DIR)
            .join(skill_id);
        let destination = agent_skill_path(agent_dir, skill_id);

        copy_dir(&source, &destination, force)?;
        report.installed_skills.push((*skill_id).to_string());
    }
    Ok(())
}

fn copy_dir(source: &Path, destination: &Path, force: bool) -> Result<()> {
    if destination.exists() {
        if force {
            fs::remove_dir_all(destination)
                .map_err(|source| CodeseedError::io(destination, source))?;
        } else {
            return Ok(());
        }
    }

    fs::create_dir_all(destination).map_err(|source| CodeseedError::io(destination, source))?;

    for entry in fs::read_dir(source).map_err(|error| CodeseedError::io(source, error))? {
        let entry = entry.map_err(|error| CodeseedError::io(source, error))?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir(&source_path, &destination_path, force)?;
        } else {
            fs::copy(&source_path, &destination_path)
                .map_err(|source| CodeseedError::io(&destination_path, source))?;
        }
    }

    Ok(())
}

fn write_state_file(
    codeseed_dir: &Path,
    agent_dir: &Path,
    codeseed_dir_arg: &Path,
    report: &InitReport,
) -> Result<()> {
    let path = codeseed_state_path(codeseed_dir);
    let skills = report
        .installed_skills
        .iter()
        .map(|skill| format!("    {{ \"id\": \"{skill}\", \"source\": \"preset:{skill}\", \"target\": \"common\" }}"))
        .collect::<Vec<_>>()
        .join(",\n");
    let content = format!(
        concat!(
            "{{\n",
            "  \"schema\": 1,\n",
            "  \"agentDir\": \"{}\",\n",
            "  \"codeseedDir\": \"{}\",\n",
            "  \"installedSkills\": [\n",
            "{}\n",
            "  ]\n",
            "}}\n"
        ),
        state_path(agent_dir),
        state_path(codeseed_dir_arg),
        skills
    );

    write_file(&path, content.as_bytes())
}

fn create_compatibility_entries(
    project: &Path,
    agent_dir: &Path,
    force: bool,
    report: &mut InitReport,
) -> Result<()> {
    create_dir(project.join(".claude").join("skills"), force, report)?;
    create_dir(project.join(".cursor").join("rules"), force, report)?;

    for skill_id in DEFAULT_PRESET_SKILL_IDS {
        let skill_path = agent_skill_path(agent_dir, skill_id);
        let claude_link = claude_link_path(project, skill_id);
        create_symlink(&skill_path, &claude_link, force)?;
        push_generated_link(report, &claude_link);

        let cursor_rule = cursor_rule_path(agent_dir, skill_id);
        write_cursor_rule(&cursor_rule, skill_id, project, agent_dir)?;
        push_generated_file(report, &cursor_rule);

        let cursor_link = cursor_link_path(project, skill_id);
        create_symlink(&cursor_rule, &cursor_link, force)?;
        push_generated_link(report, &cursor_link);
    }

    let agents_md = agents_md_path(project);
    if !agents_md.exists() {
        write_file(&agents_md, agents_md_content().as_bytes())?;
        push_generated_file(report, &agents_md);
    }

    Ok(())
}

fn write_cursor_rule(path: &Path, skill_id: &str, project: &Path, agent_dir: &Path) -> Result<()> {
    let skill_path = project_relative(&agent_skill_path(agent_dir, skill_id), project);
    let content = format!(
        concat!(
            "---\n",
            "description: Use the Codeseed preset skill {0} when maintaining Codeseed skills or project skill metadata.\n",
            "globs:\n",
            "alwaysApply: false\n",
            "---\n\n",
            "Use the Codeseed-managed skill at `{1}/SKILL.md`.\n",
            "Follow its instructions when the task is about creating, reviewing, or improving Codeseed-managed skills.\n"
        ),
        skill_id, skill_path
    );
    write_file(path, content.as_bytes())
}

fn agents_md_content() -> String {
    concat!(
        "# Codeseed Agent Instructions\n\n",
        "This repository is managed by Codeseed for project-local agent skills.\n\n",
        "## Skills\n\n",
        "- Canonical skills live under `.agent/skills/`.\n",
        "- Codeseed metadata lives under `.codeseed/`.\n",
        "- Before changing skill files, inspect the matching `skill.toml` and `SKILL.md`.\n",
        "- Every user-facing Markdown skill document should have a Chinese version when practical.\n\n",
        "## Verification\n\n",
        "- Run `cargo fmt --check` after Rust edits.\n",
        "- Run `cargo test` after CLI or skill-management changes.\n"
    )
    .to_string()
}

fn write_file(path: &Path, content: &[u8]) -> Result<()> {
    ensure_parent(path)?;
    fs::write(path, content).map_err(|source| CodeseedError::io(path, source))
}

fn create_symlink(target: &Path, link: &Path, force: bool) -> Result<()> {
    if link.exists() || is_symlink(link) {
        if force {
            remove_existing_link_or_file(link)?;
        } else if is_existing_correct_symlink(target, link) {
            return Ok(());
        } else {
            return Err(CodeseedError::conflict(
                link,
                "exists and points somewhere else; rerun with --force to replace it",
            ));
        }
    }

    ensure_parent(link)?;

    platform_symlink(&symlink_target_for_link(target, link), link)
        .map_err(|source| CodeseedError::io(link, source))
}

fn is_symlink(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
}

fn remove_existing_link_or_file(path: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(path).map_err(|source| CodeseedError::io(path, source))?;
    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(path).map_err(|source| CodeseedError::io(path, source))
    } else {
        fs::remove_file(path).map_err(|source| CodeseedError::io(path, source))
    }
}

#[cfg(unix)]
fn platform_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn platform_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    if target.is_dir() {
        std::os::windows::fs::symlink_dir(target, link)
    } else {
        std::os::windows::fs::symlink_file(target, link)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::cli::InitCommand;

    use super::{relative_path, run};

    #[test]
    fn init_creates_project_layout() {
        let project = temp_project_dir();
        let command = InitCommand {
            agent_dir: ".agent".into(),
            codeseed_dir: ".codeseed".into(),
            no_presets: false,
            no_links: false,
            force: false,
        };

        let report = run(&project, &command).expect("init should succeed");

        assert!(project.join(".agent/skills/common").is_dir());
        assert!(project.join(".agent/skills/codex").is_dir());
        assert!(project.join(".agent/skills/claude").is_dir());
        assert!(project.join(".agent/skills/cursor").is_dir());
        assert!(project
            .join(".agent/skills/common/codeseed-skill-author/SKILL.md")
            .is_file());
        assert!(project.join(".codeseed/state.json").is_file());
        assert!(project
            .join(".claude/skills/codeseed-skill-author")
            .exists());
        assert!(project
            .join(".cursor/rules/codeseed-skill-author.mdc")
            .exists());
        assert_eq!(
            std::fs::read_link(project.join(".claude/skills/codeseed-skill-author"))
                .expect("claude skill should be a symlink"),
            std::path::PathBuf::from("../../.agent/skills/common/codeseed-skill-author")
        );
        assert_eq!(
            std::fs::read_link(project.join(".cursor/rules/codeseed-skill-author.mdc"))
                .expect("cursor rule should be a symlink"),
            std::path::PathBuf::from(
                "../../.agent/generated/cursor-rules/codeseed-skill-author.mdc"
            )
        );
        assert!(project.join("AGENTS.md").is_file());
        assert_eq!(
            report.installed_skills,
            vec!["codeseed-skill-author".to_string()]
        );

        std::fs::remove_dir_all(project).ok();
    }

    #[test]
    fn relative_path_walks_from_link_parent_to_target() {
        let from = std::path::Path::new("/workspace/project/.claude/skills");
        let to = std::path::Path::new("/workspace/project/.agent/skills/common/example");

        assert_eq!(
            relative_path(from, to),
            Some(std::path::PathBuf::from(
                "../../.agent/skills/common/example"
            ))
        );
    }

    #[test]
    fn init_can_skip_presets_and_links() {
        let project = temp_project_dir();
        let command = InitCommand {
            agent_dir: ".agent".into(),
            codeseed_dir: ".codeseed".into(),
            no_presets: true,
            no_links: true,
            force: false,
        };

        run(&project, &command).expect("init should succeed");

        assert!(project.join(".agent/skills/common").is_dir());
        assert!(!project
            .join(".agent/skills/common/codeseed-skill-author")
            .exists());
        assert!(!project.join(".claude").exists());
        assert!(!project.join(".cursor").exists());
        assert!(!project.join("AGENTS.md").exists());

        std::fs::remove_dir_all(project).ok();
    }

    fn temp_project_dir() -> std::path::PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be available")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("codeseed-init-test-{stamp}"));
        std::fs::create_dir_all(&path).expect("temp project should be created");
        path
    }
}
