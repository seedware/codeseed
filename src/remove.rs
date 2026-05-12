use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::add::list_common_skills;
use crate::cli::RemoveCommand;
use crate::error::{CodeseedError, Result};
use crate::init::{normalize_project, write_state_file};
use crate::state::{read_installed_skill_ids, read_language_or_default};

const DEFAULT_AGENT_DIR: &str = ".agent";
const DEFAULT_CODESEED_DIR: &str = ".codeseed";

#[derive(Debug, Default)]
pub struct RemoveReport {
    pub removed_skill: String,
    pub removed_paths: Vec<PathBuf>,
    pub pruned_dirs: Vec<PathBuf>,
    pub updated_state: bool,
}

pub fn run(project: &Path, command: &RemoveCommand) -> Result<RemoveReport> {
    let project = normalize_project(project);
    let agent_dir = project.join(DEFAULT_AGENT_DIR);
    let codeseed_dir = project.join(DEFAULT_CODESEED_DIR);
    let language = read_language_or_default(&codeseed_dir);
    let skill_id = resolve_skill_id(&agent_dir, &codeseed_dir, &command.skill, command.force)?;

    let mut report = RemoveReport {
        removed_skill: skill_id.clone(),
        ..RemoveReport::default()
    };
    let cursor_rule = agent_dir
        .join("generated")
        .join("cursor-rules")
        .join(format!("{skill_id}.mdc"));
    let claude_link = project.join(".claude").join("skills").join(&skill_id);
    let cursor_link = project
        .join(".cursor")
        .join("rules")
        .join(format!("{skill_id}.mdc"));

    ensure_generated_path_removable(&cursor_rule, command.force)?;
    ensure_generated_path_removable(&claude_link, command.force)?;
    ensure_generated_path_removable(&cursor_link, command.force)?;

    remove_skill_path(
        &agent_dir.join("skills").join("common").join(&skill_id),
        &mut report.removed_paths,
    )?;
    remove_generated_path(&cursor_rule, command.force, &mut report.removed_paths)?;
    remove_generated_path(&claude_link, command.force, &mut report.removed_paths)?;
    remove_generated_path(&cursor_link, command.force, &mut report.removed_paths)?;

    if command.prune {
        prune_empty_dirs(
            &[
                agent_dir.join("generated").join("cursor-rules"),
                agent_dir.join("generated"),
                project.join(".claude").join("skills"),
                project.join(".claude"),
                project.join(".cursor").join("rules"),
                project.join(".cursor"),
            ],
            &mut report.pruned_dirs,
        )?;
    }

    if codeseed_dir.is_dir() {
        let installed_skills = list_common_skills(&agent_dir)?;
        write_state_file(
            &codeseed_dir,
            Path::new(DEFAULT_AGENT_DIR),
            Path::new(DEFAULT_CODESEED_DIR),
            &installed_skills,
            language,
        )?;
        report.updated_state = true;
    } else if !command.force {
        return Err(CodeseedError::conflict(
            &codeseed_dir,
            "Codeseed state is missing; rerun with --force to remove partially generated files",
        ));
    }

    Ok(report)
}

fn resolve_skill_id(
    agent_dir: &Path,
    codeseed_dir: &Path,
    query: &str,
    force: bool,
) -> Result<String> {
    let mut candidates = BTreeSet::new();
    for id in list_common_skills(agent_dir)? {
        candidates.insert(id);
    }
    if codeseed_dir.join("state.json").is_file() {
        for id in read_installed_skill_ids(codeseed_dir)? {
            candidates.insert(id);
        }
    }

    let mut matches = Vec::new();
    for id in candidates {
        if id == query || manifest_name(agent_dir, &id).as_deref() == Some(query) {
            matches.push(id);
        }
    }

    match matches.len() {
        0 if force => Ok(query.to_string()),
        0 => Err(CodeseedError::conflict(
            query,
            "is not an installed Codeseed-managed skill; rerun with --force to clean partial files",
        )),
        1 => Ok(matches.remove(0)),
        _ => Err(CodeseedError::conflict(
            query,
            "matches more than one installed skill",
        )),
    }
}

fn manifest_name(agent_dir: &Path, skill_id: &str) -> Option<String> {
    let path = agent_dir
        .join("skills")
        .join("common")
        .join(skill_id)
        .join("skill.toml");
    let content = fs::read_to_string(path).ok()?;
    manifest_value(&content, "name")
}

fn manifest_value(content: &str, key: &str) -> Option<String> {
    let prefix = format!("{key} = ");
    content.lines().find_map(|line| {
        let line = line.trim();
        let value = line.strip_prefix(&prefix)?;
        Some(value.trim().trim_matches('"').to_string())
    })
}

fn remove_skill_path(path: &Path, removed_paths: &mut Vec<PathBuf>) -> Result<()> {
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

fn ensure_generated_path_removable(path: &Path, force: bool) -> Result<()> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };
    if metadata.is_dir() && !metadata.file_type().is_symlink() && !force {
        return Err(CodeseedError::conflict(
            path,
            "exists as a directory and will not be removed as a generated link; rerun with --force to remove it",
        ));
    }
    Ok(())
}

fn remove_generated_path(path: &Path, force: bool, removed_paths: &mut Vec<PathBuf>) -> Result<()> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };

    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        if force {
            fs::remove_dir_all(path).map_err(|source| CodeseedError::io(path, source))?;
        } else {
            return Err(CodeseedError::conflict(
                path,
                "exists as a directory and will not be removed as a generated link; rerun with --force to remove it",
            ));
        }
    } else if metadata.file_type().is_symlink() || metadata.is_file() || force {
        fs::remove_file(path).map_err(|source| CodeseedError::io(path, source))?;
    } else {
        return Err(CodeseedError::conflict(
            path,
            "exists but is not a removable generated file; rerun with --force to remove it",
        ));
    }

    removed_paths.push(path.to_path_buf());
    Ok(())
}

pub(crate) fn prune_empty_dirs(paths: &[PathBuf], pruned_dirs: &mut Vec<PathBuf>) -> Result<()> {
    for path in paths {
        match fs::remove_dir(path) {
            Ok(()) => pruned_dirs.push(path.to_path_buf()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) if error.kind() == std::io::ErrorKind::DirectoryNotEmpty => {}
            Err(error) => return Err(CodeseedError::io(path, error)),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::cli::{InitCommand, InitLanguage, RemoveCommand};
    use crate::init::run as init_run;

    use super::run;

    #[test]
    fn rm_removes_installed_skill_and_updates_state() {
        let project = temp_project_dir();
        let init = InitCommand {
            agent_dir: ".agent".into(),
            codeseed_dir: ".codeseed".into(),
            no_presets: false,
            no_links: false,
            language: InitLanguage::En,
            force: false,
        };
        init_run(&project, &init).expect("init should succeed");

        let command = RemoveCommand {
            skill: "codeseed-skill-author".to_string(),
            force: false,
            prune: false,
        };
        let report = run(&project, &command).expect("remove should succeed");

        assert_eq!(report.removed_skill, "codeseed-skill-author");
        assert!(!project
            .join(".agent/skills/common/codeseed-skill-author")
            .exists());
        assert!(!project
            .join(".agent/generated/cursor-rules/codeseed-skill-author.mdc")
            .exists());
        assert!(!project
            .join(".claude/skills/codeseed-skill-author")
            .exists());
        assert!(!project
            .join(".cursor/rules/codeseed-skill-author.mdc")
            .exists());

        let state =
            std::fs::read_to_string(project.join(".codeseed/state.json")).expect("state exists");
        assert!(!state.contains("\"id\": \"codeseed-skill-author\""));
        assert!(state.contains("\"id\": \"codeseed-context-index\""));

        std::fs::remove_dir_all(project).ok();
    }

    #[test]
    fn rm_can_resolve_by_manifest_name() {
        let project = temp_project_dir();
        let init = InitCommand {
            agent_dir: ".agent".into(),
            codeseed_dir: ".codeseed".into(),
            no_presets: false,
            no_links: true,
            language: InitLanguage::En,
            force: false,
        };
        init_run(&project, &init).expect("init should succeed");

        let command = RemoveCommand {
            skill: "Codeseed Context Index".to_string(),
            force: false,
            prune: true,
        };
        let report = run(&project, &command).expect("remove should succeed");

        assert_eq!(report.removed_skill, "codeseed-context-index");
        assert!(!project
            .join(".agent/skills/common/codeseed-context-index")
            .exists());

        std::fs::remove_dir_all(project).ok();
    }

    #[test]
    fn rm_does_not_remove_user_directory_at_generated_link_without_force() {
        let project = temp_project_dir();
        let init = InitCommand {
            agent_dir: ".agent".into(),
            codeseed_dir: ".codeseed".into(),
            no_presets: false,
            no_links: true,
            language: InitLanguage::En,
            force: false,
        };
        init_run(&project, &init).expect("init should succeed");
        std::fs::create_dir_all(project.join(".claude/skills/codeseed-skill-author"))
            .expect("user directory should be created");

        let command = RemoveCommand {
            skill: "codeseed-skill-author".to_string(),
            force: false,
            prune: false,
        };
        let result = run(&project, &command);

        assert!(result.is_err());
        assert!(project
            .join(".agent/skills/common/codeseed-skill-author")
            .exists());
        assert!(project
            .join(".claude/skills/codeseed-skill-author")
            .is_dir());

        std::fs::remove_dir_all(project).ok();
    }

    fn temp_project_dir() -> std::path::PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be available")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("codeseed-remove-test-{stamp}"));
        std::fs::create_dir_all(&path).expect("temp project should be created");
        path
    }
}
