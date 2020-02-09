use anyhow::{anyhow, Result};
use git2::Repository;
use include_dir::{include_dir, Dir, DirEntry};
use std::{
    fs::{create_dir_all, write},
    path::Path,
};

static DEFAULTS: Dir = include_dir!("default/");

pub fn make(data_path: &str, git: bool) -> Result<()> {
    // create dirs
    create_dir_all(data_path).map_err(|err| anyhow!("failed to create project root {}", err))?;

    // initialize git repository
    let mut repo =
        if git {
            // initialize a repo
            Some(Repository::init(data_path).map_err(|err| {
                anyhow!("failed to create git repository in {} ({})", data_path, err)
            })?)
        } else {
            None
        };

    let inner_paths = DEFAULTS.find("**").map_err(|err| anyhow!("{}", err))?;

    for inner_path in inner_paths {
        if inner_path.path() == Path::new("") {
            continue; // skip root
        }

        match inner_path {
            DirEntry::File(file) => {
                let relative_path = file.path();
                let full = Path::new(data_path).join(relative_path);
                write(full.clone(), file.contents())
                    .map_err(|err| anyhow!("failed to write file {}", err))?;
                if let Some(repo_instance) = repo.as_mut() {
                    println!("adding {:?}", relative_path);
                    // add a file to repository
                    let mut index = repo_instance
                        .index()
                        .map_err(|err| anyhow!("Failed to get repo index ({})", err))?;
                    index
                        .add_path(relative_path)
                        .map_err(|err| anyhow!("Failed to add a file {} ({})", data_path, err))?;
                    index
                        .write()
                        .map_err(|err| anyhow!("Failed to write to index ({})", err))?;
                }
            }
            DirEntry::Dir(dir) => {
                let full = Path::new(data_path).join(dir.path());
                create_dir_all(full.clone())
                    .map_err(|err| anyhow!("failed to create dir {}", err))?;
            }
        };
    }
    Ok(())
}
