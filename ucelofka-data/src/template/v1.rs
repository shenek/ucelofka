use anyhow::{anyhow, Result};
use html2text::{from_read_with_decorator, render::text_renderer::RichDecorator};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::{read_to_string, File},
    path::{Path, PathBuf},
};

pub const VERSION: u32 = 1;
pub const TEXT_WIDTH: usize = 80;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Template {
    pub name: String,
    pub path: PathBuf,
    pub text: Option<String>,
    pub raw: Option<String>,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.path.to_str().unwrap())?;
        if let Some(text) = &self.text {
            write!(f, "\n{}", text)?;
        }
        Ok(())
    }
}

impl Template {
    fn new(path: PathBuf) -> Self {
        Self {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            path,
            text: None,
            raw: None,
        }
    }

    fn fill_text(&mut self) -> Result<()> {
        let file = File::open(&self.path)
            .map_err(|e| anyhow!("Failed to open file {}: {}", self.path.to_str().unwrap(), e))?;

        self.text = Some(from_read_with_decorator(
            file,
            TEXT_WIDTH,
            RichDecorator::new(),
        ));

        Ok(())
    }

    fn fill_raw(&mut self) -> Result<()> {
        self.raw = Some(read_to_string(&self.path)?);
        Ok(())
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

    pub fn get(&self, name: &str) -> Result<Option<Template>> {
        for template in &self.templates {
            if template.name == name {
                let mut cloned = template.clone();
                cloned.fill_text()?;
                cloned.fill_raw()?;
                return Ok(Some(cloned));
            }
        }
        Ok(None)
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
