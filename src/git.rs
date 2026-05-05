use std::process::Command;

use anyhow::{anyhow, Context, Result};

fn run_git(args: &[&str]) -> Result<std::process::Output> {
    Command::new("git")
        .args(args)
        .output()
        .context("failed to spawn git")
}

pub fn ensure_git_repo() -> Result<()> {
    let out = run_git(&["rev-parse", "--git-dir"])?;
    if !out.status.success() {
        return Err(anyhow!("not a git repository"));
    }
    Ok(())
}

pub fn is_main_worktree() -> Result<bool> {
    let out = run_git(&["rev-parse", "--git-common-dir"])?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(anyhow!(
            "git rev-parse --git-common-dir failed: {}",
            stderr.trim()
        ));
    }
    let common_dir = String::from_utf8(out.stdout)
        .context("git rev-parse output is not UTF-8")?
        .trim()
        .to_string();

    let out = run_git(&["rev-parse", "--git-dir"])?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(anyhow!("git rev-parse --git-dir failed: {}", stderr.trim()));
    }
    let git_dir = String::from_utf8(out.stdout)
        .context("git rev-parse output is not UTF-8")?
        .trim()
        .to_string();

    Ok(common_dir == git_dir)
}

pub fn current_branch() -> Result<Option<String>> {
    let out = run_git(&["symbolic-ref", "--short", "HEAD"])?;
    if !out.status.success() {
        return Ok(None);
    }
    let branch = String::from_utf8(out.stdout)
        .context("git symbolic-ref output is not UTF-8")?
        .trim()
        .to_string();
    Ok(Some(branch))
}
