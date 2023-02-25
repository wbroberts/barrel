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
