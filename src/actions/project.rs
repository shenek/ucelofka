use anyhow::{anyhow, Result};
use includedir::Files;
use std::{
    env,
    fs::{create_dir_all, write},
    path::Path,
};

include!(concat!(env!("OUT_DIR"), "/default.rs"));

pub fn make(data_path: &str) -> Result<()> {
    // create dirs
    create_dir_all(data_path).map_err(|err| anyhow!("failed to create project root {}", err))?;

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
        write(Path::new(data_path).join(inner_path_stripped), data)
            .map_err(|err| anyhow!("failed to write file {}", err))?;
    }
    Ok(())
}
