use std::{fmt, io::Cursor};

use anyhow::{bail, Result};
use git2::{BranchType, Repository, Worktree};
use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};

#[derive(Debug, Clone)]
struct SelectionError;

impl fmt::Display for SelectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No selection was made")
    }
}

pub fn select_remote_branch(repo: &Repository, filter: Option<String>) -> Result<Vec<String>> {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .reverse(true)
        .multi(true)
        .build()
        .unwrap();

    let test = repo
        .branches(Some(BranchType::Remote))
        .unwrap()
        .map(|s| s.unwrap())
        .map(|(branch, _)| branch.name().unwrap().unwrap().to_owned())
        .filter(|branch| {
            if let Some(filter) = &filter {
                branch.contains(filter)
            } else {
                true
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(test));
    let selected_branches = Skim::run_with(&options, Some(items))
        .ok_or_else(|| panic!("Error in fuzzy finder"))
        .map(|out| out.selected_items)
        .unwrap_or_else(|_| Vec::new());
    let selected_branch_names = selected_branches
        .iter()
        .map(|s| s.output().to_string())
        .collect::<Vec<String>>();

    Ok(selected_branch_names)
}

pub fn select_worktree(repo: &Repository) -> Result<Worktree> {
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
        bail!(SelectionError)
    }

    let selected_worktree_name = skim_output
        .selected_items
        .get(0)
        .unwrap()
        .output()
        .to_string();

    println!("Selected worktree: {}", selected_worktree_name);
    Ok(repo.find_worktree(&selected_worktree_name).unwrap())
}
