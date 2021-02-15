pub mod v1;
pub mod v2;

use std::convert::TryFrom;

use super::data_versions;
pub use latest::{Identities, Identity};
pub use v2 as latest;

pub const CURRENT_VERSION: u32 = v1::VERSION;

data_versions!(Identity, 1, 2);
