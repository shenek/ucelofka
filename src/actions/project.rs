use anyhow::{anyhow, Result};
use git2::Repository;
use includedir::Files;
use std::{
    env,
    fs::{create_dir_all, write},
    path::Path,
};

include!(concat!(env!("OUT_DIR"), "/default.rs"));

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

    DEFAULTS.set_passthrough(env::var_os("PASSTHROUGH").is_some());

    // write all files
    let inner_paths: Vec<&'static str> = DEFAULTS.file_names().collect();

    for inner_path in inner_paths {
        // remove 'default' from the inner path
        let inner_path_stripped = Path::new(inner_path)
            .strip_prefix("default")
            .map_err(|err| anyhow!("{}", err))?;

        // first create directory
        if let Some(parent) = inner_path_stripped.parent() {
            let target = Path::new(data_path).join(parent);
            create_dir_all(target).map_err(|err| anyhow!("failed to create dir {}", err))?;
        }
        let data: Vec<u8> = DEFAULTS.get(inner_path).unwrap().iter().copied().collect();
        let target_path = Path::new(data_path).join(inner_path_stripped);
        write(target_path.clone(), data).map_err(|err| anyhow!("failed to write file {}", err))?;
        if let Some(repo_instance) = repo.as_mut() {
            println!("adding {:?}", inner_path_stripped);
            // add a file to repository
            let mut index = repo_instance
                .index()
                .map_err(|err| anyhow!("Failed to get repo index ({})", err))?;
            index
                .add_path(inner_path_stripped)
                .map_err(|err| anyhow!("Failed to add a file {} ({})", data_path, err))?;
            index
                .write()
                .map_err(|err| anyhow!("Failed to write to index ({})", err))?;
        }
    }
    Ok(())
}
