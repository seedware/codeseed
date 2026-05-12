use std::fs;
use std::path::Path;

use crate::cli::InitLanguage;
use crate::error::{CodeseedError, Result};

pub(crate) fn read_language_or_default(codeseed_dir: &Path) -> InitLanguage {
    let path = codeseed_dir.join("state.json");
    let Ok(content) = fs::read_to_string(path) else {
        return InitLanguage::En;
    };
    match json_string_field(&content, "language").as_deref() {
        Some("zh-CN") | Some("zh-cn") | Some("zh") => InitLanguage::ZhCn,
        _ => InitLanguage::En,
    }
}

pub(crate) fn read_installed_skill_ids(codeseed_dir: &Path) -> Result<Vec<String>> {
    let path = codeseed_dir.join("state.json");
    let content = fs::read_to_string(&path).map_err(|source| CodeseedError::io(&path, source))?;
    let mut ids = Vec::new();
    for line in content.lines() {
        if let Some(id) = json_string_field(line, "id") {
            ids.push(id);
        }
    }
    ids.sort();
    ids.dedup();
    Ok(ids)
}

fn json_string_field(content: &str, key: &str) -> Option<String> {
    let marker = format!("\"{key}\"");
    let start = content.find(&marker)? + marker.len();
    let after_key = &content[start..];
    let colon = after_key.find(':')?;
    let after_colon = after_key[colon + 1..].trim_start();
    let value_start = after_colon.strip_prefix('"')?;
    let end = value_start.find('"')?;
    Some(value_start[..end].to_string())
}
