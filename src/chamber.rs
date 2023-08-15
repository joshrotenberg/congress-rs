use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Chamber {
    House,
    Senate,
}
