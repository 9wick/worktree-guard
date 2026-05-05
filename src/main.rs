use std::process::ExitCode;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

mod git;

#[derive(Parser)]
#[command(name = "worktree-guard", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Block commits in the main (root) worktree
    PreCommit,
    /// Block checkout/switch from the base branch in main worktree
    PreCheckout {
        /// Base branch name
        #[arg(long, default_value = "main")]
        base_branch: String,
        /// Whether this is a branch checkout (1) or file checkout (0)
        #[arg(long)]
        checkout_type: Option<String>,
    },
    /// Check if currently in the main worktree (exit 0 = main, exit 1 = linked)
    IsMain,
    /// Print the current branch name
    CurrentBranch,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Cmd::PreCommit => pre_commit(),
        Cmd::PreCheckout {
            base_branch,
            checkout_type,
        } => pre_checkout(&base_branch, checkout_type.as_deref()),
        Cmd::IsMain => is_main(),
        Cmd::CurrentBranch => current_branch(),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("worktree-guard: {e}");
            ExitCode::FAILURE
        }
    }
}

fn pre_commit() -> Result<()> {
    git::ensure_git_repo()?;
    if git::is_main_worktree()? {
        bail!("commits are not allowed in the main worktree. Use a linked worktree instead.");
    }
    Ok(())
}

fn pre_checkout(base_branch: &str, checkout_type: Option<&str>) -> Result<()> {
    git::ensure_git_repo()?;

    if checkout_type == Some("0") {
        return Ok(());
    }

    if !git::is_main_worktree()? {
        return Ok(());
    }

    let Some(branch) = git::current_branch()? else {
        return Ok(());
    };

    if branch == base_branch {
        bail!("switching from base branch '{base_branch}' is not allowed in main worktree.");
    }

    Ok(())
}

fn is_main() -> Result<()> {
    git::ensure_git_repo()?;
    if git::is_main_worktree()? {
        Ok(())
    } else {
        bail!("not in main worktree");
    }
}

fn current_branch() -> Result<()> {
    git::ensure_git_repo()?;
    match git::current_branch()? {
        Some(branch) => {
            println!("{branch}");
            Ok(())
        }
        None => bail!("not on a branch (detached HEAD)"),
    }
}
