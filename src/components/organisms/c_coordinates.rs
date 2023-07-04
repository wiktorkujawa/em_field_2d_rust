use std::{ops::RangeInclusive, f64::MAX};

use egui::Ui;

use crate::{enums::ArrangementMethod, structs::Interface, components::atoms::heading};

pub fn coordinates(ui: &mut Ui, interface: &mut Interface) {
  heading(ui, "Coordinates");

  match interface.arrangement_method {
      ArrangementMethod::Various => {
          if interface.route_coor.len() != 3 {
              interface.route_coor = vec![[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
          }
          for [x_coor, y_coor] in interface.route_coor.iter_mut() {
              // ui.vertical_centered(|ui| {
              ui.horizontal(|ui| {
                  ui.label("X[m]:");
                  ui.add(egui::DragValue::new(x_coor).speed(0.1));

                  ui.label("Y[m]:");
                  match interface.line_type {
                    crate::enums::LineType::PowerLine => ui.add(egui::DragValue::new(y_coor).speed(0.1).clamp_range(RangeInclusive::new(0.0, MAX))),
                    crate::enums::LineType::UndergroundCable => ui.add(egui::DragValue::new(y_coor).speed(0.1).clamp_range(RangeInclusive::new(0.0, -MAX))),
                 }
                  
              });
          }
      }
      ArrangementMethod::Flat => {
          ui.horizontal(|ui| {
              let [x, y] = &mut interface.center_position;

              ui.label("X[m]:");
              ui.add(egui::DragValue::new(x).speed(0.1));

              ui.label("Y[m]:");
              match interface.line_type {
                crate::enums::LineType::PowerLine => ui.add(egui::DragValue::new(y).speed(0.1).clamp_range(RangeInclusive::new(0.0, MAX))),
                crate::enums::LineType::UndergroundCable => ui.add(egui::DragValue::new(y).speed(0.1).clamp_range(RangeInclusive::new(0.0, -MAX))),
             };
          });
          ui.horizontal(|ui| {
              ui.label("Gap between lines[m]:");
              ui.add(egui::DragValue::new(&mut interface.lines_gap).speed(0.1).clamp_range(RangeInclusive::new(0.0, MAX)));
          });
      }
      ArrangementMethod::Trefoil => {
          ui.horizontal(|ui| {
            let [x, y] = &mut interface.center_position;

              ui.label("X[m]:");
              ui.add(egui::DragValue::new(x).speed(0.1));

              ui.label("Y[m]:");


            //   y + (*lines_gap * SQRT_2) / 3.0
              match interface.line_type {
                crate::enums::LineType::PowerLine => ui.add(egui::DragValue::new(y).speed(0.1).clamp_range(RangeInclusive::new(0.0, MAX))),
                crate::enums::LineType::UndergroundCable => ui.add(egui::DragValue::new(y).speed(0.1).clamp_range(RangeInclusive::new(0.0, -MAX))),
             };
            //   ui.add(egui::DragValue::new(y).speed(0.1));
          });
          ui.horizontal(|ui| {
              ui.label("Spacing between lines:");
              ui.add(egui::DragValue::new(&mut interface.lines_gap).speed(0.1).clamp_range(RangeInclusive::new(0.0, MAX)));
          });
      }
  }
}