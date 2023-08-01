use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Identification {
    pub name: String,
    pub value: String,
}

impl ToString for Identification {
    fn to_string(&self) -> String {
        format!("{}:{}", self.name, self.value)
    }
}
