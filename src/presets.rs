pub const PRESET_SKILLS_DIR: &str = "presets/skills";
pub const PRESET_SOURCE_PREFIX: &str = "preset:";
pub const DEFAULT_PRESET_SKILL_IDS: &[&str] = &["codeseed-skill-author"];

#[cfg(test)]
mod tests {
    use super::{DEFAULT_PRESET_SKILL_IDS, PRESET_SKILLS_DIR, PRESET_SOURCE_PREFIX};

    #[test]
    fn default_preset_ids_are_explicit() {
        assert_eq!(DEFAULT_PRESET_SKILL_IDS, ["codeseed-skill-author"]);
    }

    #[test]
    fn preset_source_prefix_is_stable() {
        assert_eq!(PRESET_SOURCE_PREFIX, "preset:");
    }

    #[test]
    fn preset_skill_directory_is_documented() {
        assert_eq!(PRESET_SKILLS_DIR, "presets/skills");
    }
}
