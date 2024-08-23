// Copyright OxidOS Automotive 2024.

use std::rc::Rc;

use crate::menu::capsule_popup;
use crate::state::Data;
use parse::peripherals::{Chip, DefaultPeripherals};

/// Menu for configuring the SpiController capsule.
pub fn config<C: Chip + 'static + serde::Serialize>(
    chip: Rc<C>,
    choice: Option<Rc<<<C as parse::peripherals::Chip>::Peripherals as DefaultPeripherals>::Spi>>,
) -> cursive::views::LinearLayout {
    match choice {
        None => config_unknown(chip),
        Some(inner) => match chip.peripherals().spi() {
            Ok(spi_peripherals) => {
                capsule_popup::<C, _>(crate::views::radio_group_with_null_known(
                    Vec::from(spi_peripherals),
                    on_spi_submit::<C>,
                    inner,
                ))
            }
            Err(_) => capsule_popup::<C, _>(crate::menu::no_support("SPI")),
        },
    }
}

fn config_unknown<C: Chip + 'static + serde::ser::Serialize>(
    chip: Rc<C>,
) -> cursive::views::LinearLayout {
    match chip.peripherals().spi() {
        Ok(spi_peripherals) => capsule_popup::<C, _>(crate::views::radio_group_with_null(
            Vec::from(spi_peripherals),
            on_spi_submit::<C>,
        )),
        Err(_) => capsule_popup::<C, _>(crate::menu::no_support("SPI")),
    }
}

/// Configure an Spi ased on the submitted Spi.
fn on_spi_submit<C: Chip + 'static + serde::ser::Serialize>(
    siv: &mut cursive::Cursive,
    submit: &Option<Rc<<C::Peripherals as DefaultPeripherals>::Spi>>,
) {
    if let Some(data) = siv.user_data::<Data<C>>() {
        if let Some(spi) = submit {
            data.platform.update_spi(spi.clone());
        } else {
            data.platform.remove_spi();
        }
    }
}