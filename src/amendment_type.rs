use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum AmendmentType {
    #[serde(rename(deserialize = "HAMDT"), alias = "hamdt")]
    House,
    #[serde(rename(deserialize = "SAMDT"), alias = "samdt")]
    Senate,
}

impl std::fmt::Display for AmendmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::House => write!(f, "hamdt"),
            Self::Senate => write!(f, "samdt"),
        }
    }
}
