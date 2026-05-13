use std::fs;
use std::path::{Path, PathBuf};

use crate::add::list_common_skills;
use crate::cli::ListCommand;
use crate::error::{CodeseedError, Result};
use crate::init::normalize_project;
use crate::presets::{embedded_preset_manifest, BUILT_IN_PRESET_SKILL_IDS, PRESET_SKILLS_DIR};

const DEFAULT_AGENT_DIR: &str = ".agent";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SkillSummary {
    pub id: String,
    pub name: String,
    pub version: String,
    pub target: String,
    pub source: String,
    pub installed: bool,
}

#[derive(Debug, Default)]
pub struct ListReport {
    pub skills: Vec<SkillSummary>,
}

pub fn run(project: &Path, command: &ListCommand) -> Result<ListReport> {
    let project = normalize_project(project);
    let installed_ids = list_common_skills(&project.join(DEFAULT_AGENT_DIR))?;

    let skills = if command.installed {
        installed_ids
            .iter()
            .map(|skill_id| read_installed_skill(&project, skill_id))
            .collect::<Result<Vec<_>>>()?
    } else {
        BUILT_IN_PRESET_SKILL_IDS
            .iter()
            .map(|skill_id| {
                read_preset_skill(skill_id, installed_ids.iter().any(|id| id == skill_id))
            })
            .collect::<Result<Vec<_>>>()?
    };

    Ok(ListReport { skills })
}

pub fn format_text(report: &ListReport) -> String {
    if report.skills.is_empty() {
        return "No skills found.\n".to_string();
    }

    let mut output = String::new();
    output.push_str("ID                           VERSION  TARGET  INSTALLED  SOURCE\n");
    for skill in &report.skills {
        output.push_str(&format!(
            "{:<28} {:<8} {:<7} {:<9} {}\n",
            skill.id,
            skill.version,
            skill.target,
            if skill.installed { "yes" } else { "no" },
            skill.source
        ));
    }
    output
}

pub fn format_json(report: &ListReport) -> String {
    let skills = report
        .skills
        .iter()
        .map(|skill| {
            format!(
                concat!(
                    "    {{",
                    "\"id\":\"{}\",",
                    "\"name\":\"{}\",",
                    "\"version\":\"{}\",",
                    "\"target\":\"{}\",",
                    "\"source\":\"{}\",",
                    "\"installed\":{}",
                    "}}"
                ),
                escape_json(&skill.id),
                escape_json(&skill.name),
                escape_json(&skill.version),
                escape_json(&skill.target),
                escape_json(&skill.source),
                skill.installed
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");
    format!("{{\n  \"skills\": [\n{}\n  ]\n}}\n", skills)
}

fn read_preset_skill(skill_id: &str, installed: bool) -> Result<SkillSummary> {
    let manifest = if let Some(source) = preset_skill_source(skill_id) {
        read_manifest(&source.join("skill.toml"))?
    } else {
        let content = embedded_preset_manifest(skill_id).ok_or_else(|| {
            CodeseedError::conflict(skill_id, "does not have embedded preset content")
        })?;
        parse_manifest(content)
    };
    Ok(SkillSummary {
        id: manifest.id,
        name: manifest.name,
        version: manifest.version,
        target: manifest.target,
        source: format!("preset:{skill_id}"),
        installed,
    })
}

fn read_installed_skill(project: &Path, skill_id: &str) -> Result<SkillSummary> {
    let source = project
        .join(DEFAULT_AGENT_DIR)
        .join("skills")
        .join("common")
        .join(skill_id);
    let manifest = read_manifest(&source.join("skill.toml"))?;
    Ok(SkillSummary {
        id: manifest.id,
        name: manifest.name,
        version: manifest.version,
        target: manifest.target,
        source: format!("installed:{}", source.display()),
        installed: true,
    })
}

fn preset_skill_source(skill_id: &str) -> Option<PathBuf> {
    let local_source = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(PRESET_SKILLS_DIR)
        .join(skill_id);
    if local_source.is_dir() {
        return Some(local_source);
    }
    let manifest_source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(PRESET_SKILLS_DIR)
        .join(skill_id);
    manifest_source.is_dir().then_some(manifest_source)
}

#[derive(Debug)]
struct SkillManifest {
    id: String,
    name: String,
    version: String,
    target: String,
}

fn read_manifest(path: &Path) -> Result<SkillManifest> {
    let content = fs::read_to_string(path).map_err(|source| CodeseedError::io(path, source))?;
    Ok(parse_manifest(&content))
}

fn parse_manifest(content: &str) -> SkillManifest {
    SkillManifest {
        id: manifest_value(content, "id").unwrap_or_else(|| "unknown".to_string()),
        name: manifest_value(content, "name").unwrap_or_else(|| "Unknown".to_string()),
        version: manifest_value(content, "version").unwrap_or_else(|| "unknown".to_string()),
        target: manifest_value(content, "target").unwrap_or_else(|| "common".to_string()),
    }
}

fn manifest_value(content: &str, key: &str) -> Option<String> {
    let prefix = format!("{key} = ");
    content.lines().find_map(|line| {
        let line = line.trim();
        let value = line.strip_prefix(&prefix)?;
        Some(value.trim().trim_matches('"').to_string())
    })
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use crate::cli::ListCommand;
    use crate::init::{run as init_run, InitReport};

    use super::{format_json, format_text, run};

    #[test]
    fn lists_built_in_available_skills() {
        let command = ListCommand {
            installed: false,
            format: crate::cli::OutputFormat::Text,
        };

        let report = run(std::path::Path::new("."), &command).expect("list should succeed");

        assert!(report
            .skills
            .iter()
            .any(|skill| skill.id == "codeseed-skill-author"));
        assert!(report
            .skills
            .iter()
            .any(|skill| skill.id == "codeseed-multi-git-remote"));
        assert!(format_text(&report).contains("codeseed-skill-author"));
        assert!(format_json(&report).contains("\"skills\""));
    }

    #[test]
    fn lists_installed_skills() {
        let project = temp_project_dir();
        let init = crate::cli::InitCommand {
            agent_dir: ".agent".into(),
            codeseed_dir: ".codeseed".into(),
            no_presets: false,
            no_links: true,
            force: false,
        };
        let _: InitReport = init_run(&project, &init).expect("init should succeed");
        let command = ListCommand {
            installed: true,
            format: crate::cli::OutputFormat::Text,
        };

        let report = run(&project, &command).expect("list should succeed");

        assert_eq!(report.skills.len(), 2);
        assert!(report
            .skills
            .iter()
            .any(|skill| skill.id == "codeseed-skill-author"));
        assert!(report
            .skills
            .iter()
            .any(|skill| skill.id == "codeseed-context-index"));

        std::fs::remove_dir_all(project).ok();
    }

    fn temp_project_dir() -> std::path::PathBuf {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock should be available")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("codeseed-list-test-{stamp}"));
        std::fs::create_dir_all(&path).expect("temp project should be created");
        path
    }
}
