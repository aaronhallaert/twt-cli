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
