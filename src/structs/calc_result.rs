use std::f64::consts::PI;

use nalgebra::{ OMatrix, Dyn, OVector };
use num_complex::{ Complex64, ComplexFloat };
use serde::{ Deserialize, Serialize };

use crate::enums::LineType;

use super::{ Route, BundleSpec };

type DMatrix = OMatrix<Complex64, Dyn, Dyn>;

type VoltageVector = OVector<Complex64, Dyn>;

#[derive(Serialize, Deserialize, Debug)]
pub struct CalcResult {
    pub x_range: [f64; 3],
    pub y_range: [f64; 3],
    pub b_field: Vec<Vec<f64>>,
    pub e_field: Vec<Vec<f64>>,
    pub current_y: i64,
}

impl Default for CalcResult {
    fn default() -> Self {
        Self {
            x_range: [-10.0, 0.2, 10.0],
            y_range: [-10.0, 0.2, 10.0],
            b_field: vec![],
            e_field: vec![],
            current_y: 0,
        }
    }
}

impl<'a> CalcResult {
    pub fn new(
        &mut self,
        [x_min, dx, x_max]: [f64; 3],
        [y_min, dy, y_max]: [f64; 3],
        routes: &mut Vec<Route>
    ) {
        self.b_field = vec![];
        self.e_field = vec![];

        let x_length = ((x_max - x_min) / dx).floor() as usize;
        let y_length = ((y_max - y_min) / dy).floor() as usize;

        self.x_range = [x_min, dx, x_max];
        self.y_range = [y_min, dy, y_max];

        let power_lines: Vec<&Route> = routes
            .iter()
            .filter(|route| { route.line_type == LineType::PowerLine })
            .collect();
        let conductors_numbers: usize = power_lines
            .iter()
            .map(|line| { line.conductors.len() })
            .sum();

        let mut c_matrix = DMatrix::zeros(conductors_numbers, conductors_numbers);

        let mut voltage_matrix = VoltageVector::zeros(conductors_numbers);

        for line in power_lines {
            let Route { bundle_spec: BundleSpec { spacing, thickness, number_per_bundle }, .. } =
                line;
            let bundle_number = *number_per_bundle as f64;

            let mut gmr =
                thickness + spacing / ((PI * (bundle_number - 2.0)) / (2.0 * bundle_number)).cos();

            gmr = gmr * ((bundle_number * thickness) / gmr).powf(1.0 / bundle_number);

            println!("GMR: {}", gmr);
            for i in 0..line.conductors.len() {
                c_matrix[(i, i)] = Complex64::new((4.0 * line.conductors[i].y) / gmr, 0.0).ln();
                voltage_matrix[i] = line.conductors[i].voltage_value;

                for j in i + 1..line.conductors.len() {
                    let x_diff_pow = (line.conductors[j].x - line.conductors[i].x).powf(2.0);
                    c_matrix[(i, j)] = Complex64::new(
                        (x_diff_pow + (line.conductors[j].y + line.conductors[i].y).powf(2.0)) /
                            (x_diff_pow + (line.conductors[j].y - line.conductors[i].y).powf(2.0)),
                        0.0
                    )
                        .sqrt()
                        .ln();
                    c_matrix[(j, i)] = c_matrix[(i, j)];
                }
            }
        }

        println!("Matrix: {}", c_matrix);

        println!("Vector: {}", voltage_matrix);

        c_matrix.try_inverse_mut();

        let q_matrix = c_matrix * voltage_matrix;
        println!("Result: {}", q_matrix);

        for y_cell in 0..y_length {
            self.b_field.push(vec![]);
            if conductors_numbers > 0 {
                self.e_field.push(vec![]);
            }

            for x_cell in 0..x_length {
                let x = x_min + (x_cell as f64) * dx;
                let y = y_min + (y_cell as f64) * dy;

                let mut bx_value = Complex64::new(0.0, 0.0);
                let mut by_value = Complex64::new(0.0, 0.0);

                let mut ex_value = Complex64::new(0.0, 0.0);
                let mut ey_value = Complex64::new(0.0, 0.0);
                for route in &mut *routes {
                    for conductor_number in 0..route.conductors.len() {
                        let delta_x = x - route.conductors[conductor_number].x;
                        let delta_y = y - route.conductors[conductor_number].y;

                        let main_factor = 1.0 / (delta_x * delta_x + delta_y * delta_y);

                        bx_value +=
                            route.conductors[conductor_number].current_value *
                            main_factor *
                            delta_y;
                        by_value +=
                            route.conductors[conductor_number].current_value *
                            main_factor *
                            delta_x;

                        if matches!(route.line_type, LineType::PowerLine) {
                            let dy_image = y + route.conductors[conductor_number].y;

                            let e_denominator = 1.0 / (delta_x * delta_x + dy_image * dy_image);

                            ex_value += q_matrix[0] * delta_x * (main_factor - e_denominator);
                            ey_value +=
                                q_matrix[0] * (delta_y * main_factor - dy_image * e_denominator);
                        }
                    }
                }
                const I: Complex64 = Complex64::new(0.0, 1.0);

                let b1_value = (by_value + I * bx_value) * 0.25;
                let b2_value = Complex64::conj(&(by_value - I * bx_value)) * 0.25;
                let mut b_value = 0.0;

                let e1_value = (ey_value + I * ex_value) * 0.5;
                let e2_value = Complex64::conj(&(ey_value - I * ex_value)) * 0.5;
                let mut e_value = 0.0;

                for t in 0..1000 {
                    let step = (I * (t as f64) * PI) / 1000.0;
                    let b_new = (b1_value * step.exp() + b2_value * (-step).exp()).abs();

                    if b_new > b_value {
                        b_value = b_new;
                    }
                    if conductors_numbers > 0 {
                        let e_new = (e1_value * step.exp() + e2_value * (-step).exp()).abs();
                        if e_new > e_value {
                            e_value = e_new;
                        }
                    }
                }
                self.b_field[y_cell].push(b_value / PI);

                if conductors_numbers > 0 {
                    self.e_field[y_cell].push(e_value * 0.57735026919);
                }
            }
        }
    }
}
