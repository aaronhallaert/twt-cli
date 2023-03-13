use std::process::Command;

pub fn remove_worktree(worktree_name: &str) {
    let mut remove_worktree = Command::new("git");
    remove_worktree
        .arg("worktree")
        .arg("remove")
        .arg(worktree_name);

    if let Ok(status_value) = remove_worktree.status() {
        if !status_value.success() {
            std::process::exit(1);
        }
    }
}

pub fn remove_branch(branch_name: &str) {
    let mut remove_branch = Command::new("git");
    remove_branch.arg("branch").arg("--delete").arg(branch_name).arg("-f");
    
    if let Ok(status_value) = remove_branch.status() {
        if !status_value.success() {
            std::process::exit(1);
        }
    }
}

pub fn prune_remote(remote: &str) {
    println!("Refetching origin");

    let mut prune_remote = Command::new("git");
    prune_remote.arg("fetch").arg(remote).arg("--prune");

    if let Ok(status_value) = prune_remote.status() {
        if !status_value.success() {
            std::process::exit(1);
        }
    }
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
    if let Ok(status_value) = cherrypick_cmd.status() {
        if !status_value.success() {
            std::process::exit(1);
        }
    }
}

pub fn checkout(branch_name: &str) {
    let mut checkout_cmd = Command::new("git");
    checkout_cmd.arg("checkout").arg(branch_name);
    if let Ok(status_value) = checkout_cmd.status() {
        if !status_value.success() {
            std::process::exit(1);
        }
    }
}

pub fn create_local(branch_name: &str) {
    let mut create_cmd = Command::new("git");
    create_cmd.arg("branch").arg(branch_name);
    if let Ok(status_value) = create_cmd.status() {
        if !status_value.success() {
            std::process::exit(1);
        }
    }
}


pub fn create_worktree(branch_name: &str, path: &str) {
    let mut create_worktree = Command::new("git");
    create_worktree
        .arg("worktree")
        .arg("add")
        .arg(path)
        .arg(branch_name);

    if let Ok(status_value) = create_worktree.status() {
        if !status_value.success() {
            std::process::exit(1);
        }
    }
}

pub fn fetch_branch(local_branch_name: &str) {
    let mut fetch_branch = Command::new("git");
    fetch_branch
        .arg("fetch")
        .arg("origin")
        .arg(format!("{}:{}", local_branch_name, local_branch_name));
    
    if let Ok(status_value) = fetch_branch.status() {
        if !status_value.success() {
            std::process::exit(1);
        }
    }
}

pub fn set_tracking_branch(local_branch_name: &str, remote_branch_name: &str) {
    let mut set_tracking_branch = Command::new("git");
    set_tracking_branch
        .arg("branch")
        .arg("-u")
        .arg(remote_branch_name)
        .arg(local_branch_name);
    
    if let Ok(status_value) = set_tracking_branch.status() {
        if !status_value.success() {
            std::process::exit(1);
        }
    }
}

pub fn create_local_from_remote(remote_branch_name: &str) -> String {
    // pull the remote branch and return the local branch
    let local_branch_name = remote_branch_name.replace("origin/", "");

    fetch_branch(&local_branch_name);
    set_tracking_branch(&local_branch_name, remote_branch_name);

    local_branch_name
}
