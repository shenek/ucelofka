pub mod v1;

pub use std::convert::TryFrom;

use super::data_versions;
pub use latest::{Account, Accounts};
pub use v1 as latest;

pub const CURRENT_VERSION: u32 = v1::VERSION;

data_versions!(Account, 1);
