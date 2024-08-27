// Copyright OxidOS Automotive 2024.

use parse::constants::PERIPHERALS;
use parse::peripheral;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum RngType {
    Rng,
}

#[derive(Debug)]
#[peripheral(serde, ident = "rng")]
pub struct Rng(RngType);

impl parse::Component for Rng {}
impl parse::Rng for Rng {}

impl std::fmt::Display for Rng {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rng")
    }
}
