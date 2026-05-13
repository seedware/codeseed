pub const PRESET_SKILLS_DIR: &str = "presets/skills";
pub const PRESET_SOURCE_PREFIX: &str = "preset:";
pub const DEFAULT_PRESET_SKILL_IDS: &[&str] = &["codeseed-skill-author", "codeseed-context-index"];
pub const BUILT_IN_PRESET_SKILL_IDS: &[&str] = &[
    "codeseed-skill-author",
    "codeseed-context-index",
    "codeseed-multi-git-remote",
    "codeseed-prebuilt-release",
];

pub struct PresetFile {
    pub path: &'static str,
    pub content: &'static str,
}

const CONTEXT_INDEX_FILES: &[PresetFile] = &[
    PresetFile {
        path: "skill.toml",
        content: include_str!("../presets/skills/codeseed-context-index/skill.toml"),
    },
    PresetFile {
        path: "SKILL.md",
        content: include_str!("../presets/skills/codeseed-context-index/SKILL.md"),
    },
];

const MULTI_GIT_REMOTE_FILES: &[PresetFile] = &[
    PresetFile {
        path: "skill.toml",
        content: include_str!("../presets/skills/codeseed-multi-git-remote/skill.toml"),
    },
    PresetFile {
        path: "SKILL.md",
        content: include_str!("../presets/skills/codeseed-multi-git-remote/SKILL.md"),
    },
];

const PREBUILT_RELEASE_FILES: &[PresetFile] = &[
    PresetFile {
        path: "skill.toml",
        content: include_str!("../presets/skills/codeseed-prebuilt-release/skill.toml"),
    },
    PresetFile {
        path: "SKILL.md",
        content: include_str!("../presets/skills/codeseed-prebuilt-release/SKILL.md"),
    },
];

const SKILL_AUTHOR_FILES: &[PresetFile] = &[
    PresetFile {
        path: "skill.toml",
        content: include_str!("../presets/skills/codeseed-skill-author/skill.toml"),
    },
    PresetFile {
        path: "SKILL.md",
        content: include_str!("../presets/skills/codeseed-skill-author/SKILL.md"),
    },
];

pub fn embedded_preset_files(skill_id: &str) -> Option<&'static [PresetFile]> {
    match skill_id {
        "codeseed-context-index" => Some(CONTEXT_INDEX_FILES),
        "codeseed-multi-git-remote" => Some(MULTI_GIT_REMOTE_FILES),
        "codeseed-prebuilt-release" => Some(PREBUILT_RELEASE_FILES),
        "codeseed-skill-author" => Some(SKILL_AUTHOR_FILES),
        _ => None,
    }
}

pub fn embedded_preset_manifest(skill_id: &str) -> Option<&'static str> {
    embedded_preset_files(skill_id)?
        .iter()
        .find(|file| file.path == "skill.toml")
        .map(|file| file.content)
}

#[cfg(test)]
mod tests {
    use super::{
        embedded_preset_files, embedded_preset_manifest, BUILT_IN_PRESET_SKILL_IDS,
        DEFAULT_PRESET_SKILL_IDS, PRESET_SKILLS_DIR, PRESET_SOURCE_PREFIX,
    };

    #[test]
    fn default_preset_ids_are_explicit() {
        assert_eq!(
            DEFAULT_PRESET_SKILL_IDS,
            ["codeseed-skill-author", "codeseed-context-index"]
        );
    }

    #[test]
    fn built_in_preset_ids_are_explicit() {
        assert_eq!(
            BUILT_IN_PRESET_SKILL_IDS,
            [
                "codeseed-skill-author",
                "codeseed-context-index",
                "codeseed-multi-git-remote",
                "codeseed-prebuilt-release"
            ]
        );
    }

    #[test]
    fn preset_source_prefix_is_stable() {
        assert_eq!(PRESET_SOURCE_PREFIX, "preset:");
    }

    #[test]
    fn preset_skill_directory_is_documented() {
        assert_eq!(PRESET_SKILLS_DIR, "presets/skills");
    }

    #[test]
    fn all_built_in_presets_are_embedded() {
        for skill_id in BUILT_IN_PRESET_SKILL_IDS {
            let files = embedded_preset_files(skill_id).expect("preset files should be embedded");
            assert_eq!(files.len(), 2);
            assert!(embedded_preset_manifest(skill_id).is_some());
        }
    }
}
