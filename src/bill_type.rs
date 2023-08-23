use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum BillType {
    // House bill
    #[serde(rename(deserialize = "HR"), alias = "hr")]
    House,
    /// Senate bill
    #[serde(rename(deserialize = "S"), alias = "s")]
    Senate,
    /// House Joint Resoultion
    #[serde(rename(deserialize = "HJRES"), alias = "hjres")]
    HouseJointResolution,
    /// Senate Joint Resoultion
    #[serde(rename(deserialize = "SJRES"), alias = "sjres")]
    SenateJointResolution,
    /// House Concurrent Resolution
    #[serde(rename(deserialize = "HCONRES"), alias = "hconres")]
    HouseConcurrentResolution,
    /// Senate Concurrent Resolution
    #[serde(rename(deserialize = "SCONRES"), alias = "sconres")]
    SenateConcurrentResolution,
    /// House Resolution
    #[serde(rename(deserialize = "HRES"), alias = "hres")]
    HouseResolution,
    /// Senate Resolution
    #[serde(rename(deserialize = "SRES"), alias = "sres")]
    SenateResolution,
}

impl std::fmt::Display for BillType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::House => write!(f, "hr"),
            Self::Senate => write!(f, "s"),
            Self::HouseJointResolution => write!(f, "hjres"),
            Self::SenateJointResolution => write!(f, "sjres"),
            Self::HouseConcurrentResolution => write!(f, "hconres"),
            Self::SenateConcurrentResolution => write!(f, "sconres"),
            Self::HouseResolution => write!(f, "hres"),
            Self::SenateResolution => write!(f, "sres"),
        }
    }
}

impl FromStr for BillType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "HR" => Ok(Self::House),
            "S" => Ok(Self::Senate),
            "HJRES" => Ok(Self::HouseJointResolution),
            "SJRES" => Ok(Self::SenateJointResolution),
            "HCONRES" => Ok(Self::HouseConcurrentResolution),
            "SCONRES" => Ok(Self::SenateConcurrentResolution),
            "HRES" => Ok(Self::HouseResolution),
            "SRES" => Ok(Self::SenateResolution),
            &_ => unreachable!(),
        }
    }
}
