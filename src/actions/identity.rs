use std::path::Path;

use crate::data::{
    identity::{Identities, Identity},
    Records,
};

pub fn list(data_path: &Path) -> Identities {
    let identity_path = data_path.join(Path::new("identities"));
    Identities::load(identity_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Option<Identity> {
    list(data_path).get(id)
}
