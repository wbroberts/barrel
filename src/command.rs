use std::{
    io::Error,
    process::{Command, Output},
};

pub fn _get_staged() -> Result<Output, Error> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg("--cached")
        .output()?;

    Ok(output)
}

// fn handle_git() -> BarrelResult {
//     let mut parents: HashSet<PathBuf> = HashSet::new();

//     get_staged()?.stdout.lines().for_each(|line| {
//         let line = line.unwrap();
//         let parent = get_parent(line);

//         parents.insert(parent);
//     });

//     let results: Vec<BarrelResult> = parents.into_iter().map(|p| create_barrel(p)).collect();

//     println!("{:?}", results);

//     Ok(())
// }

// fn get_parent(line: String) -> PathBuf {
//     let path = Path::new(&line);
//     let parent = path.parent().unwrap();

//     parent.to_path_buf()
// }
