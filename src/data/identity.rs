use std::path::Path;

#[derive(Debug)]
pub struct Identity {}

#[derive(Debug, Default)]
pub struct Identities {
    identities: Vec<Identities>,
}

impl Identities {
    pub fn load(path: &Path) -> Self {
        Self::default()
    }
}
