use std::{
    env, 
    path::{Path, PathBuf}, 
    io::{BufReader, BufRead, Write}, 
    fs::{File, OpenOptions, self, DirEntry, ReadDir}, 
    collections::BTreeMap
};

use globset::{Glob, GlobMatcher};
use regex::Regex;

mod export;

use export::Export;

fn main() {
    let path = get_dir();
    let dir = fs::read_dir(&path).unwrap();
    let glob = Glob::new("*.{ts,tsx}").unwrap().compile_matcher();
    let entries = get_entries(&glob, dir);
    let file_export_map = create_file_export_map(&entries);

    let index_path = Path::new(&path).join("index.ts");
    let mut file =  OpenOptions::new().read(true).write(true).create(true).open(&index_path).unwrap();

    for (name, export_type) in file_export_map {
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

fn get_dir() -> String {
    let mut args = env::args().skip(1);

    match args.next() {
        Some(arg) => arg,
        None => "./".to_string()
    }
}

fn get_entries(glob: &GlobMatcher, dir: ReadDir) -> Vec<DirEntry> {
    dir.filter_map(|d| {
        let d = d.unwrap();
        let meta = d.metadata().unwrap();
        
        if meta.is_file() && is_wanted_path(&glob, &d.path()) {
            Some(d)
        } else {
            None
        }
    }).collect::<Vec<DirEntry>>()
}

fn is_wanted_path(glob: &GlobMatcher, path: &Path) -> bool {
    let name = path.file_stem().unwrap();
    let name = name.to_str().unwrap();

    glob.is_match(path) && !name.contains(".test") && name != "index"
}


fn create_file_export_map(entries: &Vec<DirEntry>) -> BTreeMap<String, Export> {
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
