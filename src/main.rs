mod fuzzy;
mod gitc;
mod tmux;

use std::{env, process::exit};

use regex::Regex;

use clap::{Args, Parser, Subcommand};
use git2::{BranchType, Repository};

#[derive(Parser, Debug)]
#[clap(
    author = "Aaron Hallaert",
    version,
    about = "Checking out git worktrees in new tmux windows"
)]
struct TWTArgs {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    Create(CreateCommand),
    Remove,
    Switch,
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    pub branch_to_create: Option<String>,
}

fn main() {
    let args = TWTArgs::parse();
    let current_dir = env::current_dir().unwrap();
    let repo = Repository::open(&current_dir)
        .unwrap_or_else(|_| panic!("{:?} is not a git repository", &current_dir));

    match args.action {
        Action::Create(create_command) => handle_create_command(repo, create_command),
        Action::Switch => handle_switch_command(repo),
        Action::Remove => handle_remove_command(repo),
    };
}

fn handle_switch_command(repo: Repository) {
    let selected_worktree = fuzzy::select_worktree(&repo).unwrap_or_else(|_| exit(1));
    let worktree_path = selected_worktree.path().to_str().unwrap();

    tmux::change_window(selected_worktree.name().unwrap(), worktree_path);
}

fn handle_remove_command(repo: Repository) {
    let selected_worktree = fuzzy::select_worktree(&repo).unwrap_or_else(|_| exit(1));
    let worktree_repo = Repository::open(selected_worktree.path().to_str().unwrap())
        .unwrap_or_else(|_| panic!("Could not open workdir repo"));

    let re = Regex::new(r"[\W]").unwrap();
    let worktree_name = selected_worktree.name().unwrap();
    let branch_name = worktree_repo
        .branches(Some(BranchType::Local))
        .unwrap()
        .map(|b| b.unwrap().0)
        .find(|branch| {
            let test_branch = re.replace_all(branch.name().unwrap().unwrap(), "");
            let worktree_branch = re.replace_all(worktree_name, "");

            test_branch == worktree_branch
        })
        .unwrap();

    gitc::remove_worktree(worktree_name);
    gitc::remove_branch(branch_name.name().unwrap().unwrap());
    tmux::remove_window(worktree_name);
}

fn handle_create_command(repo: Repository, create_command: CreateCommand) {
    let branch = match &create_command.branch_to_create {
        Some(branch_to_create) => {
            println!("Handling branch: {}", branch_to_create);
            match repo.find_branch(branch_to_create, BranchType::Local) {
                Ok(b) => b,
                Err(_) => {
                    gitc::prune_remote("origin");
                    match repo.find_branch(
                        format!("origin/{}", &branch_to_create).as_str(),
                        BranchType::Remote,
                    ) {
                        Ok(branch) => {
                            println!("Remote branch exists");
                            let local_branch_name =
                                gitc::create_local_from_remote(branch.name().unwrap().unwrap());

                            repo.find_branch(&local_branch_name, BranchType::Local)
                                .unwrap()
                        }
                        Err(_) => {
                            // create branch
                            match repo.branch(
                                branch_to_create,
                                &repo.head().unwrap().peel_to_commit().unwrap(),
                                false,
                            ) {
                                Ok(b) => {
                                    println!("Successfully created branch: {}", branch_to_create);
                                    b
                                }
                                Err(e) => {
                                    eprintln!("Error creating branch: {}", e);
                                    exit(1)
                                }
                            }
                        }
                    }
                }
            }
        }
        None => {
            gitc::prune_remote("origin");
            if let Ok(remote_branch_name) = fuzzy::select_remote_branch(&repo) {
                println!("Selected worktree: {}", &remote_branch_name);

                let local_branch_name = gitc::create_local_from_remote(&remote_branch_name);

                repo.find_branch(&local_branch_name, BranchType::Local)
                    .unwrap()
            } else {
                exit(1);
            }
        }
    };

    let worktree_name = branch.name().unwrap().unwrap().replace('/', "-");

    match repo.find_worktree(&worktree_name) {
        Ok(worktree) => {
            println!(
                "Worktree already exists for this branch: {}",
                worktree.name().unwrap()
            );

            tmux::change_window(&worktree_name, worktree.path().to_str().unwrap())
        }
        Err(_) => {
            // create a new worktree
            println!(
                "No worktree exists for this branch: {}",
                branch.name().unwrap().unwrap()
            );

            // repo.is_worktree()
            println!(
                "Trying to create worktree `{}` of branch `{}`",
                &worktree_name,
                &branch.name().unwrap().unwrap()
            );
            let path = format!(
                "{}{}",
                repo.path().to_str().unwrap().split_once(".git").unwrap().0,
                worktree_name
            );

            gitc::create_worktree(branch.name().unwrap().unwrap(), &path);
            tmux::change_window(&worktree_name, &path)
        }
    }
}
