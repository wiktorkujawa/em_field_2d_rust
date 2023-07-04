use egui::Ui;

use crate::{components::atoms::heading, structs::Interface, enums::LINE_TYPE_LIST};

pub fn line_choice(ui: &mut Ui, interface: &mut Interface) {
  heading(ui, "Line type");

  ui.horizontal(|ui| {
      for (value, label) in LINE_TYPE_LIST {
          let matching = interface.line_type == value;
          if ui.radio(matching, label).clicked() {
              interface.line_type = value;
          }
      }
  });
}