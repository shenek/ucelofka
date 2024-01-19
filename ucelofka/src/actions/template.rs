use anyhow::{anyhow, Result};
use std::path::Path;

use crate::data::template::{Template, Templates};

pub fn list(data_path: &Path) -> Result<Templates> {
    let templates_path = data_path.join(Path::new("templates"));
    Templates::load(templates_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Result<Template> {
    list(data_path)?
        .get(id)?
        .ok_or_else(|| anyhow!("Template {} not found.", id))
}
