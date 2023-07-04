use egui::Ui;

use crate::{structs::Interface, components::atoms::heading};

pub fn bundle_specification(ui: &mut Ui, interface: &mut Interface) {
  heading(ui, "Bundle specification");

  ui.horizontal(|ui| {
      ui.label("Conductor thickness[cm]:");
      ui.add(egui::DragValue::new(&mut interface.bundle_spec.thickness).speed(0.1));
  });

  ui.add(
      egui::Slider
          ::new(&mut interface.bundle_spec.number_per_bundle, 1..=10)
          .text("Number of bundle conductors")
  );

  if interface.bundle_spec.number_per_bundle > 1 {
      ui.horizontal(|ui| {
          ui.label("Spacing between conductors[cm]:");
          ui.add(egui::DragValue::new(&mut interface.bundle_spec.spacing).speed(0.1));
      });
  }
}