use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::{UpdateCommand, UpdateMode};
use crate::error::{CodeseedError, Result};
use crate::init::normalize_project;

const LOCAL_INSTALL_SCRIPT: &str = "scripts/install.sh";

#[derive(Debug)]
pub struct UpdateReport {
    pub plan: String,
    pub executed: bool,
}

pub fn run(project: &Path, command: &UpdateCommand) -> Result<UpdateReport> {
    let project = normalize_project(project);
    let local_script = project.join(LOCAL_INSTALL_SCRIPT);
    let args = install_args(command);
    let plan = if local_script.is_file() {
        format!("sh {} {}", local_script.display(), args.join(" "))
    } else {
        format!(
            "download {} then run sh install.sh {}",
            command.script_url,
            args.join(" ")
        )
    };

    if command.dry_run {
        return Ok(UpdateReport {
            plan,
            executed: false,
        });
    }

    if local_script.is_file() {
        run_script(&local_script, &args)?;
    } else {
        let downloaded = download_install_script(&command.script_url)?;
        run_script(&downloaded, &args)?;
        std::fs::remove_file(downloaded).ok();
    }

    Ok(UpdateReport {
        plan,
        executed: true,
    })
}

fn install_args(command: &UpdateCommand) -> Vec<String> {
    let mut args = vec!["--version".to_string(), command.version.clone()];

    if let Some(home) = &command.home {
        args.push("--home".to_string());
        args.push(home.display().to_string());
    }

    if let Some(bin_dir) = &command.bin_dir {
        args.push("--bin-dir".to_string());
        args.push(bin_dir.display().to_string());
    }

    match command.mode {
        UpdateMode::Auto => {}
        UpdateMode::Local => args.push("--local".to_string()),
        UpdateMode::Prebuilt => args.push("--prebuilt".to_string()),
    }

    args
}

fn run_script(script: &Path, args: &[String]) -> Result<()> {
    let status = ProcessCommand::new("sh")
        .arg(script)
        .args(args)
        .status()
        .map_err(|source| CodeseedError::io("sh", source))?;

    if status.success() {
        Ok(())
    } else {
        Err(CodeseedError::conflict(
            script,
            format!("installer exited with status {status}"),
        ))
    }
}

fn download_install_script(url: &str) -> Result<PathBuf> {
    let path = temp_script_path();

    if download_with_curl(url, &path)
        .or_else(|_| download_with_wget(url, &path))
        .is_err()
    {
        return Err(CodeseedError::conflict(
            url,
            "failed to download install script with curl or wget",
        ));
    }

    Ok(path)
}

fn download_with_curl(url: &str, path: &Path) -> Result<()> {
    let status = ProcessCommand::new("curl")
        .args(["-fsSL", url, "-o"])
        .arg(path)
        .status()
        .map_err(|source| CodeseedError::io("curl", source))?;

    if status.success() {
        Ok(())
    } else {
        Err(CodeseedError::conflict(
            "curl",
            format!("download failed with status {status}"),
        ))
    }
}

fn download_with_wget(url: &str, path: &Path) -> Result<()> {
    let status = ProcessCommand::new("wget")
        .arg("-qO")
        .arg(path)
        .arg(url)
        .status()
        .map_err(|source| CodeseedError::io("wget", source))?;

    if status.success() {
        Ok(())
    } else {
        Err(CodeseedError::conflict(
            "wget",
            format!("download failed with status {status}"),
        ))
    }
}

fn temp_script_path() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    std::env::temp_dir().join(format!("codeseed-install-{stamp}.sh"))
}

#[cfg(test)]
mod tests {
    use crate::cli::{UpdateCommand, UpdateMode};

    use super::run;

    #[test]
    fn dry_run_reports_local_plan_when_script_exists() {
        let command = UpdateCommand {
            version: "latest".to_string(),
            home: None,
            bin_dir: None,
            mode: UpdateMode::Auto,
            script_url: "https://example.com/install.sh".to_string(),
            dry_run: true,
        };

        let report =
            run(std::path::Path::new("."), &command).expect("dry-run update should succeed");

        assert!(!report.executed);
        assert!(report.plan.contains("scripts/install.sh"));
    }
}
