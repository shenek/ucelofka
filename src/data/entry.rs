use std::path::Path;

#[derive(Debug)]
pub struct Entry {}

#[derive(Debug, Default)]
pub struct Entries {
    entries: Vec<Entry>,
}

impl Entries {
    pub fn load(path: &Path) -> Self {
        Self::default()
    }
}
