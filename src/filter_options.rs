use std::path::PathBuf;

use globset::{Glob, GlobMatcher};
use regex::Regex;

pub struct FilterOptions {
    ignore_reg: Regex,
    default_reg: Regex,
    name_reg: Regex,
    glob: GlobMatcher,
}

impl FilterOptions {
    pub fn new() -> FilterOptions {
        let glob = Glob::new("*.{ts,tsx}").unwrap().compile_matcher();
        let ignore_reg = Regex::new("(.test|.stories|index)").unwrap();
        let default_reg = Regex::new("(export|default|function)").unwrap();
        let name_reg = Regex::new("[^a-zA-Z0-9]").unwrap();

        FilterOptions {
            ignore_reg,
            default_reg,
            name_reg,
            glob,
        }
    }

    pub fn get_default_name(&self, line: &str) -> String {
        let without_export = self.default_reg.replace_all(line, "");
        let func_name = self.name_reg.replace_all(&without_export, "");

        func_name.into()
    }

    pub fn is_wanted_file(&self, path: &PathBuf) -> bool {
        let name = path.file_stem().unwrap();
        let name = name.to_str().unwrap();

        self.glob.is_match(path) && !self.ignore_reg.is_match(name)
    }

    pub fn is_wanted_dir(&self, path: &PathBuf) -> bool {
        path.join("index.ts").exists()
    }
}

#[test]
fn test_is_wanted_file() {
    let options = FilterOptions::new();
    let test_path = PathBuf::from("files/Component.test.tsx");
    let stories_path = PathBuf::from("files/Component.stories.tsx");
    let component_path = PathBuf::from("files/Component.component.tsx");
    let index_path = PathBuf::from("files/index.ts");

    assert_eq!(options.is_wanted_file(&test_path), false);
    assert_eq!(options.is_wanted_file(&stories_path), false);
    assert_eq!(options.is_wanted_file(&index_path), false);
    assert_eq!(options.is_wanted_file(&component_path), true);
}

#[test]
fn test_get_default_name() {
    let options = FilterOptions::new();
    let const_default = "export default MyCustomComponent;";
    let function_default = "export default function MyCustomFunctionComponent() {";
    let numbered_export = "export default Count2;";

    assert_eq!(options.get_default_name(const_default), "MyCustomComponent");

    assert_eq!(
        options.get_default_name(function_default),
        "MyCustomFunctionComponent"
    );

    assert_eq!(options.get_default_name(numbered_export), "Count2");
}
