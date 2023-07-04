use egui::Ui;

pub fn heading(ui: &mut Ui, text: &str) {
  ui.separator();
  ui.heading(text);
  ui.separator();
}