// Copyright OxidOS Automotive 2024.

use parse::constants::PERIPHERALS;
use parse::peripheral;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum TemperatureType {
    Temp,
}

#[derive(Debug)]
#[peripheral(serde, ident = "temperature")]
pub struct Temperature(TemperatureType);

impl parse::Component for Temperature {}
impl parse::Temperature for Temperature {}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "temperature")
    }
}
