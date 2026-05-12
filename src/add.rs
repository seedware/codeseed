use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::{AddCommand, SkillTarget};
use crate::error::{CodeseedError, Result};
use crate::init::{
    create_compatibility_entries_for_skills, create_dir, install_preset_skill, normalize_project,
    write_state_file, InitReport,
};
use crate::presets::PRESET_SOURCE_PREFIX;
use crate::state::read_language_or_default;

const DEFAULT_AGENT_DIR: &str = ".agent";
const DEFAULT_CODESEED_DIR: &str = ".codeseed";

#[derive(Debug, Default)]
pub struct AddReport {
    pub installed_skill: String,
    pub generated_files: Vec<PathBuf>,
    pub generated_links: Vec<PathBuf>,
}

pub fn run(project: &Path, command: &AddCommand) -> Result<AddReport> {
    let skill_id = parse_preset_source(&command.source)?;
    validate_preset_options(command)?;

    let project = normalize_project(project);
    let agent_dir = project.join(DEFAULT_AGENT_DIR);
    let codeseed_dir = project.join(DEFAULT_CODESEED_DIR);
    let language = read_language_or_default(&codeseed_dir);

    let mut init_report = InitReport::default();
    ensure_managed_dirs(&agent_dir, &codeseed_dir, command.force, &mut init_report)?;
    install_preset_skill(&agent_dir, &skill_id, command.force, language)?;
    create_compatibility_entries_for_skills(
        &project,
        &agent_dir,
        &[skill_id.as_str()],
        command.force,
        language,
        &mut init_report,
    )?;

    let installed_skills = list_common_skills(&agent_dir)?;
    write_state_file(
        &codeseed_dir,
        Path::new(DEFAULT_AGENT_DIR),
        Path::new(DEFAULT_CODESEED_DIR),
        &installed_skills,
        language,
    )?;

    Ok(AddReport {
        installed_skill: skill_id,
        generated_files: init_report.generated_files,
        generated_links: init_report.generated_links,
    })
}

fn parse_preset_source(source: &str) -> Result<String> {
    let Some(skill_id) = source.strip_prefix(PRESET_SOURCE_PREFIX) else {
        return Err(CodeseedError::conflict(
            source,
            "only preset:<skill-id> sources are implemented right now",
        ));
    };
    if skill_id.is_empty() {
        return Err(CodeseedError::conflict(
            source,
            "preset source must include a skill id",
        ));
    }
    Ok(skill_id.to_string())
}

fn validate_preset_options(command: &AddCommand) -> Result<()> {
    if command.hub.is_some() {
        return Err(CodeseedError::conflict(
            "--hub",
            "cannot be used with preset sources",
        ));
    }
    if command.name.is_some() {
        return Err(CodeseedError::conflict(
            "--name",
            "renaming preset skills is not implemented yet",
        ));
    }
    if command.target_dir.is_some() {
        return Err(CodeseedError::conflict(
            "--target-dir",
            "custom preset target directories are not implemented yet",
        ));
    }
    if let Some(target) = command.target {
        if target != SkillTarget::Common {
            return Err(CodeseedError::conflict(
                "--target",
                "preset skills can currently only be installed to common",
            ));
        }
    }
    Ok(())
}

fn ensure_managed_dirs(
    agent_dir: &Path,
    codeseed_dir: &Path,
    force: bool,
    report: &mut InitReport,
) -> Result<()> {
    create_dir(agent_dir, force, report)?;
    create_dir(codeseed_dir, force, report)?;
    create_dir(agent_dir.join("skills").join("common"), force, report)?;
    create_dir(agent_dir.join("skills").join("codex"), force, report)?;
    create_dir(agent_dir.join("skills").join("claude"), force, report)?;
    create_dir(agent_dir.join("skills").join("cursor"), force, report)?;
    create_dir(
        agent_dir.join("generated").join("cursor-rules"),
        force,
        report,
    )?;
    Ok(())
}

pub(crate) fn list_common_skills(agent_dir: &Path) -> Result<Vec<String>> {
    let common_dir = agent_dir.join("skills").join("common");
    let mut skills = Vec::new();
    if !common_dir.exists() {
        return Ok(skills);
    }

    for entry in fs::read_dir(&common_dir).map_err(|error| CodeseedError::io(&common_dir, error))? {
        let entry = entry.map_err(|error| CodeseedError::io(&common_dir, error))?;
        let path = entry.path();
        if path.is_dir() {
            skills.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    skills.sort();
    Ok(skills)
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::cli::AddCommand;

    use super::run;

    #[test]
    fn add_installs_preset_skill() {
        let project = temp_project_dir();
        let command = AddCommand {
            source: "preset:codeseed-multi-git-remote".to_string(),
            hub: None,
            name: None,
            target: None,
            target_dir: None,
            force: false,
        };

        let report = run(&project, &command).expect("add should succeed");

        assert_eq!(report.installed_skill, "codeseed-multi-git-remote");
        assert!(project
            .join(".agent/skills/common/codeseed-multi-git-remote/SKILL.md")
            .is_file());
        assert!(project
            .join(".claude/skills/codeseed-multi-git-remote")
            .exists());
        assert!(project
            .join(".cursor/rules/codeseed-multi-git-remote.mdc")
            .exists());
        assert!(project.join(".codeseed/state.json").is_file());

        std::fs::remove_dir_all(project).ok();
    }

    fn temp_project_dir() -> std::path::PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be available")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("codeseed-add-test-{stamp}"));
        std::fs::create_dir_all(&path).expect("temp project should be created");
        path
    }
}
