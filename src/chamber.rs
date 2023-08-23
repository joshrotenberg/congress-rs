use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Chamber {
    House,
    Senate,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum ChamberCode {
    H,
    S,
}
