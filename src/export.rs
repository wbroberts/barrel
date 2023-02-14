#[derive(Debug)]
pub enum Export {
    None,
    Named,
    Default(String)
}

impl Export {
    pub fn to_value(&self) -> Option<String> {
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
