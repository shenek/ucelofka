use std::path::Path;

use crate::data::identity::{Identities, Identity};

pub fn list(data_path: &Path) -> Identities {
    let identity_path = data_path.join(Path::new("identities"));
    Identities::load(identity_path.as_path())
}

pub fn get(data_path: &Path, id: &str) -> Option<Identity> {
    for identity in list(data_path).identities {
        if identity.id == id {
            return Some(identity);
        }
    }
    None
}
