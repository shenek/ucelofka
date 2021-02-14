use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    path::{Path, PathBuf},
};

pub const VERSION: u32 = 1;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Template {
    pub name: String,
    pub path: PathBuf,
}

impl<'a> fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.path.to_str().unwrap())?;
        Ok(())
    }
}

impl Template {
    fn new(path: PathBuf) -> Self {
        Self {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            path,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Templates {
    pub templates: Vec<Template>,
}

impl Templates {
    pub fn load(template_dir: &Path) -> Result<Self> {
        let mut paths: Vec<PathBuf> = template_dir
            .read_dir()?
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|e| template_dir.join(e.path()))
            .collect();

        // sort novices by filename
        paths.sort();

        let mut templates: Vec<Template> = Vec::new();
        for path in paths {
            let path_buf = path.to_path_buf();
            templates.push(Template::new(path_buf));
        }
        Ok(Self { templates })
    }

    pub fn get(&self, name: &str) -> Option<Template> {
        for template in &self.templates {
            if template.name == name {
                return Some(template.clone());
            }
        }
        None
    }
}

impl fmt::Display for Templates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for template in &self.templates {
            writeln!(f, "{}", template)?;
        }
        Ok(())
    }
}
