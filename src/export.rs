#[derive(Debug)]
pub enum Export {
    None,
    Named,
    Default(String),
    Module,
}

impl Export {
    pub fn to_value(&self) -> Option<String> {
        match self {
            Export::Named | Export::Module => Some("export * from".to_string()),
            Export::Default(name) => {
                let export = format!("export {{ default as {} }} from", name);
                Some(export)
            },
            _ => None
        }
    }
}
