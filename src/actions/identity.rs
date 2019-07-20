use std::path::Path;

use crate::data::identity::Identities;

pub fn list(data_path: &Path) {
    let identity_path = data_path.join(Path::new("identities"));
    println!("{}", Identities::load(identity_path.as_path()));
}
