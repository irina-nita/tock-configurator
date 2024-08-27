// Copyright OxidOS Automotive 2024.

use parse::{constants::PERIPHERALS, peripheral};

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum UartType {
    Uart0,
}

#[derive(Debug, PartialEq)]
#[peripheral(serde, ident = ".nrf52.uarte0")]
pub struct Uart(UartType);

impl Default for Uart {
    fn default() -> Self {
        Self::new(UartType::Uart0)
    }
}

impl parse::Component for Uart {}

impl parse::Uart for Uart {}
impl std::fmt::Display for Uart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "uarte0")
    }
}
