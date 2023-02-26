use std::{
    collections::HashSet,
    io::{BufRead, Error},
    path::{Path, PathBuf},
    process::{Command, Output},
};

pub fn get_staged() -> Result<Output, Error> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg("--cached")
        .output()?;

    Ok(output)
}

pub fn add_updates(paths: &HashSet<PathBuf>) -> Result<Output, Error> {
    let additions = paths
        .into_iter()
        .map(|p| String::from(p.to_str().unwrap()))
        .collect::<Vec<String>>()
        .join(" ");

    let output = Command::new("git").arg("add").arg(additions).output()?;

    Ok(output)
}

pub fn handle_git() -> Result<HashSet<PathBuf>, Error> {
    let mut parents: HashSet<PathBuf> = HashSet::new();

    get_staged()?.stdout.lines().for_each(|line| {
        let line = line.unwrap();
        let parent = get_parent(line);

        parents.insert(parent);
    });

    Ok(parents)
}

fn get_parent(line: String) -> PathBuf {
    let path = Path::new(&line);
    let parent = path.parent().unwrap();

    parent.to_path_buf()
}
