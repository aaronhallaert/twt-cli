# twt-cli: Tmux worktree CLI

A fuzzy finder based command line interface to switch, create and remove git
worktrees with accompanying tmux windows written in Rust.

## Usage

### Create

```bash
twt-cli create [BRANCH_NAME]
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

## Git shortcuts

### Backport

```bash
twt-cli backport <BRANCH_NAME> <FROM_COMMIT> [TO_COMMIT]
```

This command lists all remote release branches (hardcoded filter `release/`).
The picked release branch(es) (select multiple with <kbd>TAB</kbd>) is used as
the base of the new `<BRANCH_NAME>`.
All commits from the given commits (`[TO_COMMIT]` is optional) are cherry-picked
onto this new branch.

Note: The branch name should have the following format `#{initials}/#{feature_name}`.
Release branches are expected to be of the following format `release/#{release_id}`.
The backport branch will eventually have the name: `#{initials}/#{release_id}-#{feature_name}`.

### Media

https://user-images.githubusercontent.com/37899722/208522716-08734ec2-cc56-4cbe-aef7-0857e6db7e20.mov

## Note

- Upstream is hardcoded `origin`
- `/` character in branches will be replaced by `-` in worktree names.
- I work with an orphan branch instead of a bare repo for worktrees.
- Link between a worktree and a branch is sometimes hardcoded by name.
  This can cause for unexpected behaviour if you check out a different
  branch in a worktree.
