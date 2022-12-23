use std::{env, io::Cursor, process::exit};

use regex::Regex;

use std::process::Command;

use clap::{Args, Parser, Subcommand};
use git2::{Branch, BranchType, Repository, Worktree};
use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};

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
    println!("SWITCH WORKTREE");
    let selected_worktree = fuzzy_select_worktree(&repo);

    let worktree_path = selected_worktree.path().to_str().unwrap();
    println!("Trying to change directories to: {}", worktree_path);

    change_tmux_window(selected_worktree.name().unwrap(), worktree_path);
}

fn handle_remove_command(repo: Repository) {
    println!("REMOVE WORKTREE");
    let selected_worktree = fuzzy_select_worktree(&repo);
    let worktree_repo = Repository::open(selected_worktree.path().to_str().unwrap())
        .unwrap_or_else(|_| panic!("Could not open workdir repo"));

    let re = Regex::new(r"[\W]").unwrap();
    let branch_name = worktree_repo
        .branches(Some(BranchType::Local))
        .unwrap()
        .map(|b| b.unwrap().0)
        .find(|branch| {
            let test_branch = re.replace_all(branch.name().unwrap().unwrap(), "");
            let worktree_branch = re.replace_all(selected_worktree.name().unwrap(), "");

            test_branch == worktree_branch
        })
        .unwrap();

    let worktree_name = selected_worktree.name().unwrap();

    let mut remove_worktree = Command::new("git");
    remove_worktree
        .arg("worktree")
        .arg("remove")
        .arg(worktree_name);
    remove_worktree.output().unwrap();

    let mut remove_branch = Command::new("git");
    remove_branch
        .arg("branch")
        .arg("--delete")
        .arg(branch_name.name().unwrap().unwrap());
    remove_branch.output().unwrap();

    let mut remove_tmux_window = Command::new("tmux");
    remove_tmux_window
        .arg("kill-window")
        .arg("-t")
        .arg(worktree_name);
    remove_tmux_window.output().unwrap();
}

fn handle_create_command(repo: Repository, create_command: CreateCommand) {
    println!("CREATE A NEW WORKTREE");
    println!("Refetching origin");
    let mut prune_remote = Command::new("git");
    prune_remote.arg("fetch").arg("origin").arg("prune");
    prune_remote.output().unwrap();

    let branch = match &create_command.branch_to_create {
        Some(branch_to_create) => {
            println!("Handling branch: {}", branch_to_create);
            match repo.find_branch(branch_to_create, BranchType::Local) {
                Ok(b) => b,
                Err(_) => {
                    match repo.find_branch(
                        format!("origin/{}", &branch_to_create).as_str(),
                        BranchType::Remote,
                    ) {
                        Ok(branch) => {
                            println!("Remote branch exists");
                            let local_branch_name = create_local_branch_from_remote(&branch);

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
        None => fuzzy_select_and_pull_remote_branch(&repo),
    };

    let worktree_name = branch.name().unwrap().unwrap().replace('/', "-");

    match repo.find_worktree(&worktree_name) {
        Ok(worktree) => {
            println!(
                "Worktree already exists for this branch: {}",
                worktree.name().unwrap()
            );

            change_tmux_window(&worktree_name, worktree.path().to_str().unwrap())
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

            let mut create_worktree = Command::new("git");
            create_worktree
                .arg("worktree")
                .arg("add")
                .arg(&path)
                .arg(branch.name().unwrap().unwrap());
            create_worktree.output().unwrap();

            change_tmux_window(&worktree_name, &path)
        }
    }
}

fn change_tmux_window(selected_worktree: &str, worktree_path: &str) {
    let mut find_window = Command::new("tmux");
    find_window.arg("list-windows");
    if String::from_utf8(find_window.output().unwrap().stdout)
        .unwrap()
        .contains(selected_worktree)
    {
        let mut swap_window = Command::new("tmux");
        swap_window
            .arg("select-window")
            .arg("-t")
            .arg(selected_worktree);
        swap_window.output().unwrap();
    } else {
        let mut create_new_window = Command::new("tmux");
        create_new_window
            .arg("neww")
            .arg("-n")
            .arg(selected_worktree)
            .arg("-c")
            .arg(worktree_path);
        create_new_window.output().unwrap();
    }
}

fn fuzzy_select_worktree(repo: &Repository) -> Worktree {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .reverse(true)
        .multi(false)
        .build()
        .unwrap();

    let test = repo
        .worktrees()
        .unwrap()
        .iter()
        .map(|s| s.unwrap())
        .collect::<Vec<&str>>()
        .join("\n");

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(test));
    let skim_output = Skim::run_with(&options, Some(items))
        .ok_or_else(|| panic!("Error in fuzzy finder"))
        .unwrap();

    if skim_output.is_abort {
        println!("No worktree was selected");
        exit(1);
    }

    let selected_worktree_name = skim_output
        .selected_items
        .get(0)
        .unwrap()
        .output()
        .to_string();

    println!("Selected worktree: {}", selected_worktree_name);
    repo.find_worktree(&selected_worktree_name).unwrap()
}

fn fuzzy_select_and_pull_remote_branch(repo: &Repository) -> Branch {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .reverse(true)
        .multi(false)
        .build()
        .unwrap();

    let test = repo
        .branches(Some(BranchType::Remote))
        .unwrap()
        .map(|s| s.unwrap())
        .map(|(branch, _)| branch.name().unwrap().unwrap().to_owned())
        .collect::<Vec<String>>()
        .join("\n");

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(test));
    let skim_output = Skim::run_with(&options, Some(items))
        .ok_or_else(|| panic!("Error in fuzzy finder"))
        .unwrap();

    if skim_output.is_abort {
        println!("No remote branch was selected");
        exit(1);
    }

    let selected_branch_name = skim_output
        .selected_items
        .get(0)
        .unwrap()
        .output()
        .to_string();

    println!("Selected worktree: {}", selected_branch_name);
    let remote_branch = repo
        .find_branch(&selected_branch_name, BranchType::Remote)
        .unwrap();

    let local_branch_name = create_local_branch_from_remote(&remote_branch);

    repo.find_branch(&local_branch_name, BranchType::Local)
        .unwrap()
}

fn create_local_branch_from_remote(remote_branch: &Branch) -> String {
    // pull the remote branch and return the local branch
    let local_branch_name = remote_branch
        .name()
        .unwrap()
        .unwrap()
        .replace("origin/", "");

    let mut fetch_branch = Command::new("git");
    fetch_branch
        .arg("fetch")
        .arg("origin")
        .arg(format!("{}:{}", local_branch_name, local_branch_name));
    fetch_branch.output().unwrap();

    let mut set_tracking_branch = Command::new("git");
    set_tracking_branch
        .arg("branch")
        .arg("-u")
        .arg(remote_branch.name().unwrap().unwrap())
        .arg(&local_branch_name);
    set_tracking_branch.output().unwrap();

    local_branch_name
}
