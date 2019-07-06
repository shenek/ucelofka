use std::{
    fmt,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
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

#[derive(Debug, Default)]
pub struct Templates {
    templates: Vec<Template>,
}

impl Templates {
    pub fn load(template_dir: &Path) -> Self {
        let mut paths: Vec<PathBuf> = match template_dir.read_dir() {
            Ok(list) => {
                let res: Vec<PathBuf> = list
                    .map(|e| match e {
                        Ok(item) => template_dir.join(item.path()),
                        Err(err) => panic!(format!("{}", err)),
                    })
                    .collect();
                res
            }
            Err(err) => panic!(format!("{}", err)),
        };
        // sort novices by filename
        paths.sort();

        let mut templates: Vec<Template> = Vec::new();
        for path in paths {
            let path_buf = path.to_path_buf();
            templates.push(Template::new(path_buf));
        }
        Self { templates }
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
