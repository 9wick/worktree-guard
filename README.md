# worktree-guard

Git hook helper that enforces worktree isolation rules: prevents commits in root worktree and branch switching on protected branches.

## Installation

### Cargo

```bash
cargo install worktree-guard
```

### npm

```bash
npm install -g worktree-guard
```

## Usage

### With Lefthook

Add to your `lefthook.yml`:

```yaml
pre-commit:
  commands:
    worktree-guard:
      run: worktree-guard pre-commit

pre-checkout:
  commands:
    worktree-guard:
      run: worktree-guard pre-checkout --checkout-type "{3}"
```

### Commands

```bash
# Block commits in the main (root) worktree
worktree-guard pre-commit

# Block checkout/switch from the base branch in main worktree
worktree-guard pre-checkout --base-branch main --checkout-type 1

# Check if currently in the main worktree
worktree-guard is-main

# Print the current branch name
worktree-guard current-branch
```

### Options

- `pre-checkout --base-branch <BRANCH>`: Base branch name (default: `main`)
- `pre-checkout --checkout-type <TYPE>`: `1` for branch checkout, `0` for file checkout

## How it works

This tool helps enforce a worktree-based workflow:

1. **Main worktree** stays on the base branch (e.g., `main`)
2. **Linked worktrees** are used for feature branches
3. Commits are blocked in the main worktree
4. Branch switching away from the base branch is blocked in the main worktree

## Why worktree-guard?

When using git worktrees, it's easy to accidentally commit in the wrong worktree or switch branches unexpectedly. worktree-guard prevents these common mistakes by enforcing simple rules at the git hook level.

## License

MIT
