use std::path::PathBuf;

#[derive(Debug)]
pub enum Export {
    Named(PathBuf),
    Default(PathBuf, String),
    Module(PathBuf),
}

impl Export {
    pub fn to_value(&self) -> String {
        match self {
            Export::Named(path) | Export::Module(path) => {
                let export = format!("export * from './{}';", self.get_file_stem(path));

                export
            }
            Export::Default(path, name) => {
                let export = format!(
                    "export {{ default as {} }} from './{}';",
                    name,
                    self.get_file_stem(path)
                );
                export
            }
        }
    }

    fn get_file_stem(&self, path: &PathBuf) -> String {
        let file_name = path.file_stem().unwrap();
        let file_name = file_name.to_owned();
        let file_name = file_name.to_str().unwrap();

        file_name.to_string()
    }
}

#[test]
fn test_export_to_value() {
    let named = Export::Named(PathBuf::from("file/Header.tsx"));
    assert_eq!(named.to_value(), "export * from './Header';");

    let module = Export::Module(PathBuf::from("file/Module"));
    assert_eq!(module.to_value(), "export * from './Module';");

    let default = Export::Default(
        PathBuf::from("file/Default.component.tsx"),
        "DefaultFunc".to_string(),
    );
    assert_eq!(
        default.to_value(),
        "export { default as DefaultFunc } from './Default.component';"
    );
}
