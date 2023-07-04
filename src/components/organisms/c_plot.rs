use std::ops::RangeInclusive;

use egui::{ Ui, plot::{ Plot, Legend, Line, LineStyle, PlotPoints }, Color32, Slider };

use crate::{ components::atoms::heading, structs::CalcResult };

pub fn plot(ui: &mut Ui, result: &mut CalcResult) {
    heading(ui, "Plot");

    // let size = frame.info();

    let x_fmt = |x: f64, _range: &RangeInclusive<f64>| { format!("{:.0}m", x) };

    let plot = Plot::new("lines_demo")
        .legend(Legend::default())
        .auto_bounds_x()
        .auto_bounds_y()
        // .height(ui.available_height()-100.0)
        .x_axis_formatter(x_fmt);

    let [x_min, dx, x_max] = result.x_range;
    let [y_min, dy, y_max] = result.y_range;

    let x_length = ((x_max - x_min) / dx).floor() as usize;
    let y_length = ((y_max - y_min) / dy).floor() as usize;

    // if result.b_field.len()>0 {
    // b_field = (0..x_length)
    //     .map(|i| {
    //         [
    //             ((x_min as f64) + (i as f64) * (dx as f64)) as f64,
    //             result.b_field[result.current_y as usize][i] as f64,
    //         ]
    //     })
    //     .collect();

    // line = Line::new(b_field)
    //     .color(Color32::RED)
    //     .style(LineStyle::Solid)
    //     .name("Magnetic field");
    // }

    //     let e_field: PlotPoints = (0..x_length)
    //     .map(|i| {
    //         [
    //             ((x_min as f64) + (i as f64) * (dx as f64)) as f64,
    //             result.e_field[result.current_y as usize][i] as f64,
    //         ]
    //     })
    //     .collect();

    // let line2 = Line::new(e_field)
    //     .color(Color32::RED)
    //     .style(LineStyle::Solid)
    //     .name("Electric field");

    ui.horizontal_centered(|ui| {
        if result.b_field.len() + result.e_field.len() > 0 {
            ui.spacing_mut().slider_width = ui.available_height() - 90.0;
            ui.add(
                Slider::new(&mut result.current_y, 0..=(y_length - 1) as i64)
                    .vertical()
                    .custom_formatter(|n, _| { format!("{:.2} m", y_min + n * dy) })
                    .text("Line height:")
            );
        }

        ui.set_height(ui.available_height() - 120.0);
        plot.show(ui, |plot_ui| {
            if result.b_field.len() > 0 && result.b_field[0].len()>0 {
                let b_field: PlotPoints = (0..x_length)
                    .map(|i| {
                        [
                            ((x_min as f64) + (i as f64) * (dx as f64)) as f64,
                            result.b_field[result.current_y as usize][i] as f64,
                        ]
                    })
                    .collect();

                plot_ui.line(
                    Line::new(b_field)
                        .color(Color32::RED)
                        .style(LineStyle::Solid)
                        .name("Magnetic field")
                );
            }

            if result.e_field.len() > 0 && result.e_field[0].len()>0  {
                let e_field: PlotPoints = (0..x_length)
                    .map(|i| {
                        [
                            ((x_min as f64) + (i as f64) * (dx as f64)) as f64,
                            result.e_field[result.current_y as usize][i] as f64,
                        ]
                    })
                    .collect();

                plot_ui.line(
                    Line::new(e_field)
                        .color(Color32::BLUE)
                        .style(LineStyle::Solid)
                        .name("Electric field")
                );
            }
        });
    });
}
