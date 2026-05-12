use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::add::list_common_skills;
use crate::cli::{ClearCommand, InitLanguage};
use crate::error::{CodeseedError, Result};
use crate::init::{agents_md_content, context_index_content, normalize_project};
use crate::remove::prune_empty_dirs;
use crate::state::read_installed_skill_ids;

const CONFIRMATION_PHRASE: &str = "clear-codeseed-state";

#[derive(Debug, Default)]
pub struct ClearReport {
    pub dry_run: bool,
    pub planned_paths: Vec<PathBuf>,
    pub removed_paths: Vec<PathBuf>,
    pub pruned_dirs: Vec<PathBuf>,
}

pub fn run(project: &Path, command: &ClearCommand) -> Result<ClearReport> {
    let project = normalize_project(project);
    let agent_dir = project.join(&command.agent_dir);
    let codeseed_dir = project.join(&command.codeseed_dir);

    if !command.dry_run && !command.yes {
        confirm_interactively(&project)?;
    }

    let skill_ids = skill_ids_for_clear(&agent_dir, &codeseed_dir)?;
    let mut report = ClearReport {
        dry_run: command.dry_run,
        planned_paths: planned_paths(&project, &agent_dir, &codeseed_dir, &skill_ids),
        ..ClearReport::default()
    };

    if command.dry_run {
        return Ok(report);
    }

    for skill_id in &skill_ids {
        remove_if_exists(
            &project.join(".claude").join("skills").join(skill_id),
            &mut report.removed_paths,
        )?;
        remove_if_exists(
            &project
                .join(".cursor")
                .join("rules")
                .join(format!("{skill_id}.mdc")),
            &mut report.removed_paths,
        )?;
    }

    remove_exact_generated_file(
        &project.join("AGENTS.md"),
        &[
            agents_md_content(InitLanguage::En),
            agents_md_content(InitLanguage::ZhCn),
        ],
        &mut report.removed_paths,
    )?;
    remove_exact_generated_file(
        &project.join("docs").join("context").join("README.md"),
        &[
            context_index_content(InitLanguage::En),
            context_index_content(InitLanguage::ZhCn),
        ],
        &mut report.removed_paths,
    )?;
    remove_exact_generated_file(
        &project.join("docs").join("context").join("README.zh-CN.md"),
        &[context_index_content(InitLanguage::ZhCn)],
        &mut report.removed_paths,
    )?;

    remove_dir_or_file(&agent_dir, &mut report.removed_paths)?;
    remove_dir_or_file(&codeseed_dir, &mut report.removed_paths)?;

    prune_empty_dirs(
        &[
            project.join(".claude").join("skills"),
            project.join(".claude"),
            project.join(".cursor").join("rules"),
            project.join(".cursor"),
            project.join("docs").join("context"),
            project.join("docs"),
        ],
        &mut report.pruned_dirs,
    )?;

    Ok(report)
}

fn confirm_interactively(project: &Path) -> Result<()> {
    eprintln!(
        "This will remove Codeseed-managed state from {}.",
        project.display()
    );
    eprint!("Type {CONFIRMATION_PHRASE} to continue: ");
    io::stderr()
        .flush()
        .map_err(|source| CodeseedError::io(project, source))?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|source| CodeseedError::io(project, source))?;
    if input.trim() == CONFIRMATION_PHRASE {
        Ok(())
    } else {
        Err(CodeseedError::conflict(
            project,
            "clear cancelled because the confirmation phrase did not match",
        ))
    }
}

fn skill_ids_for_clear(agent_dir: &Path, codeseed_dir: &Path) -> Result<Vec<String>> {
    if codeseed_dir.join("state.json").is_file() {
        return read_installed_skill_ids(codeseed_dir);
    }
    list_common_skills(agent_dir)
}

fn planned_paths(
    project: &Path,
    agent_dir: &Path,
    codeseed_dir: &Path,
    skill_ids: &[String],
) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for skill_id in skill_ids {
        push_if_removable_link(
            &project.join(".claude").join("skills").join(skill_id),
            &mut paths,
        );
        push_if_removable_link(
            &project
                .join(".cursor")
                .join("rules")
                .join(format!("{skill_id}.mdc")),
            &mut paths,
        );
    }
    push_if_exact_generated_file(
        &project.join("AGENTS.md"),
        &[
            agents_md_content(InitLanguage::En),
            agents_md_content(InitLanguage::ZhCn),
        ],
        &mut paths,
    );
    push_if_exact_generated_file(
        &project.join("docs").join("context").join("README.md"),
        &[
            context_index_content(InitLanguage::En),
            context_index_content(InitLanguage::ZhCn),
        ],
        &mut paths,
    );
    push_if_exact_generated_file(
        &project.join("docs").join("context").join("README.zh-CN.md"),
        &[context_index_content(InitLanguage::ZhCn)],
        &mut paths,
    );
    push_if_exists(agent_dir, &mut paths);
    push_if_exists(codeseed_dir, &mut paths);
    paths
}

fn push_if_removable_link(path: &Path, paths: &mut Vec<PathBuf>) {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return;
    };
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        paths.push(path.to_path_buf());
    }
}

fn push_if_exact_generated_file(
    path: &Path,
    expected_contents: &[String],
    paths: &mut Vec<PathBuf>,
) {
    let Ok(content) = fs::read_to_string(path) else {
        return;
    };
    if expected_contents
        .iter()
        .any(|expected| expected == &content)
    {
        paths.push(path.to_path_buf());
    }
}

fn push_if_exists(path: &Path, paths: &mut Vec<PathBuf>) {
    if path.exists() || fs::symlink_metadata(path).is_ok() {
        paths.push(path.to_path_buf());
    }
}

fn remove_exact_generated_file(
    path: &Path,
    expected_contents: &[String],
    removed_paths: &mut Vec<PathBuf>,
) -> Result<()> {
    let Ok(content) = fs::read_to_string(path) else {
        return Ok(());
    };
    if expected_contents
        .iter()
        .any(|expected| expected == &content)
    {
        fs::remove_file(path).map_err(|source| CodeseedError::io(path, source))?;
        removed_paths.push(path.to_path_buf());
    }
    Ok(())
}

fn remove_if_exists(path: &Path, removed_paths: &mut Vec<PathBuf>) -> Result<()> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };
    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        return Err(CodeseedError::conflict(
            path,
            "exists as a directory and will not be removed as a compatibility link",
        ));
    }
    fs::remove_file(path).map_err(|source| CodeseedError::io(path, source))?;
    removed_paths.push(path.to_path_buf());
    Ok(())
}

fn remove_dir_or_file(path: &Path, removed_paths: &mut Vec<PathBuf>) -> Result<()> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };
    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(path).map_err(|source| CodeseedError::io(path, source))?;
    } else {
        fs::remove_file(path).map_err(|source| CodeseedError::io(path, source))?;
    }
    removed_paths.push(path.to_path_buf());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::cli::{ClearCommand, InitCommand, InitLanguage};
    use crate::init::run as init_run;

    use super::run;

    #[test]
    fn clear_dry_run_reports_plan_without_removing() {
        let project = temp_project_dir();
        init_project(&project, InitLanguage::En);
        let command = ClearCommand {
            agent_dir: ".agent".into(),
            codeseed_dir: ".codeseed".into(),
            dry_run: true,
            yes: false,
            confirm: None,
        };

        let report = run(&project, &command).expect("clear dry-run should succeed");

        assert!(report.dry_run);
        assert!(report
            .planned_paths
            .iter()
            .any(|path| path.ends_with(".agent")));
        assert!(project.join(".agent").exists());
        assert!(project.join(".codeseed").exists());

        std::fs::remove_dir_all(project).ok();
    }

    #[test]
    fn clear_removes_managed_state_and_generated_links() {
        let project = temp_project_dir();
        init_project(&project, InitLanguage::ZhCn);
        let command = ClearCommand {
            agent_dir: ".agent".into(),
            codeseed_dir: ".codeseed".into(),
            dry_run: false,
            yes: true,
            confirm: Some("clear-codeseed-state".to_string()),
        };

        let report = run(&project, &command).expect("clear should succeed");

        assert!(!report.dry_run);
        assert!(!project.join(".agent").exists());
        assert!(!project.join(".codeseed").exists());
        assert!(!project.join(".claude").exists());
        assert!(!project.join(".cursor").exists());
        assert!(!project.join("AGENTS.md").exists());
        assert!(!project.join("docs").exists());

        std::fs::remove_dir_all(project).ok();
    }

    fn init_project(project: &std::path::Path, language: InitLanguage) {
        let init = InitCommand {
            agent_dir: ".agent".into(),
            codeseed_dir: ".codeseed".into(),
            no_presets: false,
            no_links: false,
            language,
            force: false,
        };
        init_run(project, &init).expect("init should succeed");
    }

    fn temp_project_dir() -> std::path::PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be available")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("codeseed-clear-test-{stamp}"));
        std::fs::create_dir_all(&path).expect("temp project should be created");
        path
    }
}
