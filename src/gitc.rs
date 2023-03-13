use std::process::Command;

pub fn remove_worktree(worktree_name: &str) {
    let mut remove_worktree = Command::new("git");
    remove_worktree
        .arg("worktree")
        .arg("remove")
        .arg(worktree_name);
    remove_worktree.output().unwrap();
}

pub fn remove_branch(branch_name: &str) {
    let mut remove_branch = Command::new("git");
    remove_branch.arg("branch").arg("--delete").arg(branch_name).arg("-f");
    remove_branch.output().unwrap();
}

pub fn prune_remote(remote: &str) {
    println!("Refetching origin");

    let mut prune_remote = Command::new("git");
    prune_remote.arg("fetch").arg(remote).arg("prune");
    prune_remote.output().unwrap();
}

pub fn cherrypick(from_hash: &str, to_hash: Option<String>) {
    let mut cherrypick_cmd = Command::new("git");

    match to_hash {
        Some(hash) => {
            cherrypick_cmd.arg("cherry-pick").arg(format!("{}..{}", from_hash, hash));
        }
        None => {
            cherrypick_cmd.arg("cherry-pick").arg(from_hash);
        }
    }
    cherrypick_cmd.output().unwrap();

    let mut rebase_cmd = Command::new("git");
    rebase_cmd.arg("rebase").arg("--interactive").arg("--autostash").arg("--keep-empty").arg("HEAD");
    rebase_cmd.output().unwrap();
}

pub fn checkout(branch_name: &str) {
    let mut checkout_cmd = Command::new("git");
    checkout_cmd.arg("checkout").arg(branch_name);
    checkout_cmd.output().unwrap();
}

pub fn create_local(branch_name: &str) {
    let mut create_cmd = Command::new("git");
    create_cmd.arg("branch").arg(branch_name);
    create_cmd.output().unwrap();
}


pub fn create_worktree(branch_name: &str, path: &str) {
    let mut create_worktree = Command::new("git");
    create_worktree
        .arg("worktree")
        .arg("add")
        .arg(path)
        .arg(branch_name);

    create_worktree.output().unwrap();
}

pub fn fetch_branch(local_branch_name: &str) {
    let mut fetch_branch = Command::new("git");
    fetch_branch
        .arg("fetch")
        .arg("origin")
        .arg(format!("{}:{}", local_branch_name, local_branch_name));
    fetch_branch.output().unwrap();
}

pub fn set_tracking_branch(local_branch_name: &str, remote_branch_name: &str) {
    let mut set_tracking_branch = Command::new("git");
    set_tracking_branch
        .arg("branch")
        .arg("-u")
        .arg(remote_branch_name)
        .arg(local_branch_name);
    set_tracking_branch.output().unwrap();
}

pub fn create_local_from_remote(remote_branch_name: &str) -> String {
    // pull the remote branch and return the local branch
    let local_branch_name = remote_branch_name.replace("origin/", "");

    fetch_branch(&local_branch_name);
    set_tracking_branch(&local_branch_name, remote_branch_name);

    local_branch_name
}
