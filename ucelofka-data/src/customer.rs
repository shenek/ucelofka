pub mod v1;
pub mod v2;

use std::convert::TryFrom;

use super::data_versions;
pub use latest::{Customer, Customers};
pub use v2 as latest;

pub const CURRENT_VERSION: u32 = latest::VERSION;

data_versions!(Customer, 1, 2);
