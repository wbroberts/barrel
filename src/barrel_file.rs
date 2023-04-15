use std::{
    error::Error,
    fs::{self, DirEntry, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use crate::{export::Export, FilterOptions};

pub struct BarrelFile;

type BarrelResult<T> = Result<T, Box<dyn Error>>;

impl BarrelFile {
    pub fn create(path: &PathBuf, options: &FilterOptions) -> BarrelResult<bool> {
        let mut entries = filter_entries(path, options)?;
        let exports = entries_to_exports(&mut entries, options);
        let is_new_file = create_barrel_file(path, exports)?;

        Ok(is_new_file)
    }
}

fn filter_entries(path: &PathBuf, options: &FilterOptions) -> BarrelResult<Vec<DirEntry>> {
    let dir = fs::read_dir(path)?;

    let entries = dir
        .filter_map(|d| {
            let d = d.unwrap();
            let meta = d.metadata().unwrap();
            let is_wanted_file = meta.is_file() && options.is_wanted_file(&d.path());
            let is_wanted_dir = meta.is_dir() && options.is_wanted_dir(&d.path());

            if is_wanted_file || is_wanted_dir {
                Some(d)
            } else {
                None
            }
        })
        .collect::<Vec<DirEntry>>();

    Ok(entries)
}

fn entries_to_exports(entries: &mut [DirEntry], options: &FilterOptions) -> Vec<Export> {
    entries.sort_by_key(|a| a.file_name());

    entries
        .iter_mut()
        .filter_map(|entry| to_export(&entry.path(), options))
        .collect::<Vec<Export>>()
}

fn to_export(entry: &PathBuf, options: &FilterOptions) -> Option<Export> {
    let mut file_export: Option<Export> = None;

    if entry.is_dir() {
        file_export = Some(Export::Module(entry.to_owned()));
        return file_export;
    }

    let f = File::open(entry.as_path()).unwrap();
    let buf = BufReader::new(f);

    for line in buf.lines() {
        let line = line.unwrap();

        if !line.starts_with("export ") {
            continue;
        }

        if !line.contains(" default ") {
            file_export = Some(Export::Named(entry.to_owned()));
        } else {
            let default_name = options.get_default_name(&line);
            file_export = Some(Export::Default(entry.to_owned(), default_name));
            break;
        }
    }

    file_export
}

fn create_barrel_file(dir: &Path, exports: Vec<Export>) -> BarrelResult<bool> {
    let (mut file, is_new) = open_barrel_file(dir)?;

    for export in exports {
        let export = export.to_value();

        writeln!(file, "{}", export).unwrap();
    }

    Ok(is_new)
}

fn open_barrel_file(path: &Path) -> Result<(File, bool), io::Error> {
    let index_path = path.join("index.ts");
    let is_new = !index_path.exists();

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&index_path)?;

    Ok((file, is_new))
}
