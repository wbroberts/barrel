use std::{
    collections::BTreeMap,
    fs::{self, DirEntry, File, OpenOptions, ReadDir},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use colored::Colorize;
use regex::Regex;

use barrel::{Arguments, Export, Options};

#[macro_use]
extern crate lazy_static;

fn main() {
    let barrel = Arguments::parse();
    let dir = fs::read_dir(&barrel.path).unwrap();
    let entries = get_entries(&barrel.options, dir);
    let export_map = create_file_export_map(entries);

    if export_map.len() > 0 {
        create_barrel_file(&barrel.path, export_map);
        println!("✔ {}", "Done".green());
    } else {
        println!("💤 {}", "Nothing to export".cyan());
    }
}

fn get_entries(config: &Options, dir: ReadDir) -> Vec<DirEntry> {
    dir.filter_map(|d| {
        let d = d.unwrap();
        let meta = d.metadata().unwrap();

        if meta.is_file() && is_wanted_path(&config, &d.path()) {
            Some(d)
        } else if meta.is_dir() && has_barrel(&d.path()) {
            Some(d)
        } else {
            None
        }
    })
    .collect::<Vec<DirEntry>>()
}

fn is_wanted_path(config: &Options, path: &Path) -> bool {
    let name = path.file_stem().unwrap();
    let name = name.to_str().unwrap();

    config.glob.is_match(path) && !config.ignore.is_match(name)
}

fn has_barrel(path: &Path) -> bool {
    path.join("index.ts").exists()
}

fn create_file_export_map(entries: Vec<DirEntry>) -> BTreeMap<String, Export> {
    let mut file_map = BTreeMap::new();

    for entry in entries {
        let path = entry.path();
        let export = get_file_export(&path);
        let name = path.file_stem().unwrap();
        let name = String::from(name.to_str().unwrap());

        file_map.insert(name, export);
    }

    file_map
}

fn get_default_func_name(line: &str) -> String {
    lazy_static! {
        static ref DEFAULT: Regex = Regex::new("(export default )(function)").unwrap();
        static ref NAME: Regex = Regex::new("[^a-zA-Z]").unwrap();
    }

    let without_export = DEFAULT.replace(line, "");
    let func_name = NAME.replace_all(&without_export, "");

    func_name.into()
}

fn get_file_export(entry: &PathBuf) -> Export {
    let mut file_export = Export::None;

    if entry.is_dir() {
        file_export = Export::Module;
        return file_export;
    }

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
            let func_name = get_default_func_name(&line);
            file_export = Export::Default(func_name.to_string());
            break;
        }
    }

    file_export
}

fn create_barrel_file(path: &Path, export_map: BTreeMap<String, Export>) {
    let mut file = get_barrel_file(path);

    for (name, export_type) in export_map {
        let export = match export_type.to_value() {
            Some(e) => e,
            None => {
                continue;
            }
        };

        writeln!(file, "{} './{}';", export, name).unwrap();
    }
}

fn get_barrel_file(path: &Path) -> File {
    let index_path = path.join("index.ts");

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&index_path)
        .unwrap()
}
