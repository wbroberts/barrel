use std::{
    env, 
    path::{Path, PathBuf}, 
    result::Result, 
    io::{BufReader, BufRead, Write}, 
    fs::{File, OpenOptions}, 
    collections::BTreeMap
};

use glob::glob;
use regex::Regex;

#[derive(Debug)]
enum Export {
    None,
    Named,
    Default(String)
}

impl Export {
    fn to_value(&self) -> Option<String> {
        match self {
            Export::Named => Some("export * from".to_string()),
            Export::Default(name) => {
                let export = format!("export {{ default as {} }} from", name);
                Some(export)
            },
            _ => None
        }
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let path = match args.next() {
        Some(arg) => arg,
        None => "./".to_string()
    };
    let pattern = Path::new(&path).join("*.tsx");
    let pattern = pattern.to_str().unwrap();
    let mut file_map = BTreeMap::new();

    for entry in glob(pattern).unwrap().filter_map(Result::ok) {
        if entry.to_str().unwrap().contains(".test") {
            continue;
        }

        let file_export = get_file_export(&entry);
        let file_name = entry.file_stem().unwrap();
        let file_name = file_name.to_str().unwrap();

        file_map.insert(file_name.to_owned(), file_export);
    };

    let index_path = Path::new(&path).join("index.ts");
    let mut file =  OpenOptions::new().read(true).write(true).create(true).open(&index_path).unwrap();

    for (name, export_type) in file_map {
        let export = match export_type.to_value() {
            Some(e) => e,
            None => {
                continue;
            }
        };

        writeln!(file, "{} './{}';", export, name).unwrap();
    }

    println!("barrel file updated");
}

fn get_file_export(entry: &PathBuf) -> Export {
    let default_reg = Regex::new("export default ").unwrap();
    let mut file_export = Export::None;

    let f = File::open(entry.as_path()).unwrap();
    let buf = BufReader::new(f);

    for line in buf.lines() {
        let line = line.unwrap();

        if line.starts_with("import ") || line.len() == 0 || !line.starts_with("export ") {
            continue;
        }

        if !line.contains(" default ") {
            file_export = Export::Named;
        } else {
            let func_name = default_reg.replace(&line, "");
            let mut func_name = func_name.split(" ");
            let func_name = func_name.next().unwrap();

            file_export = Export::Default(func_name[..func_name.len() - 1].to_string());
            break;
        }
    }

    file_export
}

