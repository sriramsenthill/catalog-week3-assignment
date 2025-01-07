use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CollectionType {
    Depths,
    Swaps,
    Runepools,
    Earnings,
}

impl CollectionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Depths => "depths",
            Self::Swaps => "swaps",
            Self::Runepools => "runepools",
            Self::Earnings => "earnings",
        }
    }
}
