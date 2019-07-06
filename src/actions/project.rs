use includedir::Files;
use std::{
    env,
    fs::{create_dir_all, write},
    path::Path,
};

include!(concat!(env!("OUT_DIR"), "/default.rs"));

pub fn make(data_path: &str) {
    // create dirs
    if let Err(err) = create_dir_all(data_path) {
        panic!(format!("failed to create project root {}", err));
    }

    DEFAULTS.set_passthrough(env::var_os("PASSTHROUGH").is_some());

    // write all files
    let inner_paths: Vec<&'static str> = DEFAULTS.file_names().collect();

    for inner_path in inner_paths {
        // remove 'default' from the inner path
        let inner_path_stripped = match Path::new(inner_path).strip_prefix("default") {
            Ok(path) => path,
            Err(err) => panic!(format!("{}", err)),
        };

        // first create directory
        if let Some(parent) = inner_path_stripped.parent() {
            let target = Path::new(data_path).join(parent);
            if let Err(err) = create_dir_all(target) {
                panic!(format!("failed to create dir {}", err));
            }
        }
        let data: Vec<u8> = DEFAULTS.get(inner_path).unwrap().iter().copied().collect();
        if let Err(err) = write(Path::new(data_path).join(inner_path_stripped), data) {
            panic!(format!("failed to write file {}", err));
        }
    }
}
