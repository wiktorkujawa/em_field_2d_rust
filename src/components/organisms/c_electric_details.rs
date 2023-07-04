use egui::Ui;

use crate::{structs::Interface, components::atoms::heading, enums::LineType};

pub fn electric_details(ui: &mut Ui, interface: &mut Interface) {
  heading(ui, "Electric Details");

  ui.horizontal(|ui| {
      ui.label("Current value[A]:");
      ui.add(egui::DragValue::new(&mut interface.electric_info.current_value).speed(0.1));
  });

  if matches!(interface.line_type, LineType::PowerLine) {
      ui.horizontal(|ui| {
          ui.label("Voltage value[kV]:");
          ui.add(egui::DragValue::new(&mut interface.electric_info.voltage_value).speed(0.1));
      });
  }

  ui.horizontal(|ui| {
      ui.label("Phase shift angle:");
      ui.drag_angle(&mut interface.electric_info.phase_shift);
  });
}