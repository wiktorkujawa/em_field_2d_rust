use egui::{Ui, FontId, RichText, Color32};

use crate::{structs::{Route, Conductor}, components::atoms::heading, enums::LineType};

pub fn routes(ui: &mut Ui, routes: &mut Vec<Route>) {
  heading(ui, "Routes");

  let mut index = 0 as usize;
  while index < routes.len() {
      ui.group(|ui| {
          ui.horizontal(|ui| {
              ui.label(
                  RichText::new(format!("Route nr {}:", index + 1)).font(
                      FontId::proportional(16.0)
                  )
              );

              ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                  if
                      ui
                          .add(
                              egui::Button
                                  ::new(RichText::new("X").color(Color32::WHITE))
                                  .small()
                                  .fill(Color32::DARK_RED)
                          )
                          .clicked()
                  {
                      routes.remove(index);
                  }
              });
          });
          if routes.len() != 0 && index < routes.len() {
              let Route { line_type, bundle_spec, electric_info, conductors } = &routes[index];

              match line_type {
                  LineType::PowerLine => ui.label("Line type: Power line"),
                  LineType::UndergroundCable => ui.label("Line type: Underground Cable"),
              };

              ui.label(format!("Current value: {}A", electric_info.current_value));
              if matches!(line_type, LineType::PowerLine) {
                  ui.label(format!("Voltage value: {}kV", electric_info.voltage_value));
              }
              ui.label(format!("Phase shift: {} degrees", electric_info.phase_shift));

              if matches!(line_type, LineType::PowerLine) {
                  ui.label(RichText::new("Bundle spec: ").font(FontId::proportional(16.0)));

                  ui.label(format!("Conductor thickness: {}cm", bundle_spec.thickness));
                  ui.label(
                      format!(
                          "Number of conductors per bundle: {}",
                          bundle_spec.number_per_bundle
                      )
                  );
                  if bundle_spec.number_per_bundle > 1 {
                      ui.label(format!("Spacing: {}cm", bundle_spec.spacing));
                  }
              }
              ui.label(RichText::new("Conductors: ").font(FontId::proportional(16.0)));

              let phase_order_letters: Vec<char> = electric_info.phase_order
                  .to_string()
                  .chars()
                  .collect();

              for index in 0..conductors.len() {
                  let Conductor { x, y, .. } = conductors[index];
                  if conductors.len() == 3 {
                      ui.label(
                          format!(
                              "{} line: [X, Y]=[{:.2},{:.2}] m Phase {}",
                              index+1,
                              x,
                              y,
                              phase_order_letters[index]
                          )
                      );
                  }
              }
          }
      });
      index += 1;
  }
}