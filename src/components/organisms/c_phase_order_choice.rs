use egui::Ui;

use crate::{enums::PHASE_ORDER_LIST, structs::Interface, components::atoms::heading};

pub fn phase_order_choice(ui: &mut Ui, interface: &mut Interface) {
  heading(ui, "Phase Order");

  ui.horizontal(|ui| {
      for index in 0..=2 {
          let (value, label) = PHASE_ORDER_LIST[index];
          let matching = interface.electric_info.phase_order == value;
          if ui.radio(matching, label).clicked() {
              interface.electric_info.phase_order = value;
          }
      }
  });

  ui.horizontal(|ui| {
      for index in 3..=5 {
          let (value, label) = PHASE_ORDER_LIST[index];
          let matching = interface.electric_info.phase_order == value;
          if ui.radio(matching, label).clicked() {
              interface.electric_info.phase_order = value;
          }
      }
  });
  // for (value, label) in PHASE_ORDER_LIST {
  //     let matching = interface.phase_order == value;
  //     if ui.radio(matching, label).clicked() {
  //         interface.phase_order = value;
  //     }
  // }
}