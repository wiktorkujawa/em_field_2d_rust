use egui::Ui;

use crate::{enums::ARRANGE_METHODS_LIST, structs::Interface, components::atoms::heading};

pub fn arrangement_method(ui: &mut Ui, interface: &mut Interface) {
  heading(ui, "Arrangement method");

  ui.horizontal(|ui| {
      for (value, label) in ARRANGE_METHODS_LIST {
          let matching = interface.arrangement_method == value;
          if ui.radio(matching, label).clicked() {
              interface.arrangement_method = value;
          }
      }
  });
}