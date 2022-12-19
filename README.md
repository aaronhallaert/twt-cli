# twt-cli: Tmux worktree CLI

A fuzzy finder based command line interface to switch, create and remove git
worktrees with accompanying tmux windows written in Rust.

## Usage

### Create

```bash
twt-cli create <optional-branch>
```

If no branch is specified, the application fetches (with prune)
and lists all remote branches.
Pick one of the remote branches and a local branch is created with a worktree.
In addition to all the git features, a new tmux window is created checked out at
the new worktree.

### Switch

```bash
twt-cli switch
```

This command shows all worktrees. If one is picked, the Tmux window will switch
to the selected worktree.

### Remove

```bash
twt-cli remove
```

This command shows all worktrees. If one is picked, the selected worktree is
removed with the corresponding branch and if the Tmux window exists for this
worktree, it will be deleted.

## Note

- Upstream is hardcoded `origin`
- `/` character in branches will be replaced by `-` in worktree names.
- I work with an orphan branch instead of a bare repo for worktrees.
- Link between a worktree and a branch is sometimes hardcoded by name.
This can cause for unexpected behaviour if you check out a different
branch in a worktree.
