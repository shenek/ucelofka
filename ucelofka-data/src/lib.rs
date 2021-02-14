pub mod account;
pub mod customer;
pub mod entry;
pub mod identity;
pub mod invoice;
pub mod template;

pub use anyhow::{anyhow, Result};
pub use paste::paste;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

fn default_version() -> u32 {
    1
}

pub trait Versioned {
    fn latest(data: &str) -> Result<Self>
    where
        Self: Sized;
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
struct Ver {
    #[serde(default = "default_version")]
    #[serde(skip_serializing)]
    pub _version: u32,
}

impl Into<u32> for Ver {
    fn into(self) -> u32 {
        self._version
    }
}

impl TryFrom<&str> for Ver {
    type Error = serde_yaml::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Ok(serde_yaml::from_str(input)?)
    }
}

pub fn detect_version(data: &str) -> u32 {
    Ver::try_from(data)
        .unwrap_or_else(|_| Ver {
            _version: default_version(),
        })
        .into()
}

#[macro_export]
macro_rules! data_display {
    ($type: ident) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", serde_yaml::to_string(self).unwrap())?;
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! data_try_from {
    ($type: ident) => {
        impl std::convert::TryFrom<String> for $type {
            type Error = serde_yaml::Error;

            fn try_from(input: String) -> Result<Self, Self::Error> {
                Ok(serde_yaml::from_str(&input)?)
            }
        }
    };
}

#[macro_export]
macro_rules! data_versions {
    ($type: ident, $( $v:literal),* ) => {
        $crate::paste! {
        impl $crate::Versioned for latest::$type {
            fn latest(data: &str) -> $crate::Result<Self>
            where
                Self: Sized,
            {
                Ok(match $crate::detect_version(data) {
                    $(
                    $v => [<v $v>]::$type::try_from(data.to_string())
                        .map_err(|err| $crate::anyhow!("Failed to read account v{} data: {}", stringify!($v), err))?
                        .into(),
                    )*
                    _ => unimplemented!(),
                })
            }
        }
        }
    };
}
