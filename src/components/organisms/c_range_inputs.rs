use egui::{ Ui };

pub fn range_inputs(ui: &mut Ui, mut x_range: [f64;3], mut y_range: [f64;3]) {
    // heading(ui, "Calculation area");

    let [x_min, dx, x_max] = &mut x_range;
    let [y_min, dy, y_max] = &mut y_range;

    // ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(x_min).speed(0.1));
                ui.add(egui::DragValue::new(dx).speed(0.1));
                ui.add(egui::DragValue::new(x_max).speed(0.1));
                ui.label("Xmin:dX:Xmax");
            });

            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(y_min).speed(0.1));
                ui.add(egui::DragValue::new(dy).speed(0.1));
                ui.add(egui::DragValue::new(y_max).speed(0.1));
                ui.label("Ymin:dY:Ymax");
            });
        });
        // ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        //     if ui.add(egui::Button::new(RichText::new("Generate plot").size(20.0))).clicked() {
        //         let x_grid = ((*x_max - *x_min) / (*dx)).floor() as usize;
        //         let y_grid = ((*y_max - *y_min) / (*dy)).floor() as usize;

        //         let Bx = vec![vec![Complex::new(0.0,0.0); x_grid]; y_grid];
        //         let By = vec![vec![Complex::new(0.0,0.0); x_grid]; y_grid];
            
        //         let B1 = vec![vec![Complex::new(0.0,0.0); x_grid]; y_grid];
        //         let B2 = vec![vec![Complex::new(0.0,0.0); x_grid]; y_grid];
        //     }
        // });
    // });
}
