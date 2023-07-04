use num_complex::Complex;
// use num_complex::Complex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Conductor {
  pub x: f64,
  pub y: f64,
  pub current_value: Complex<f64>,
  pub voltage_value: Complex<f64>
}

impl Default for Conductor {
  fn default() -> Self {
      Self {
          x: 3.0,
          y: 3.0,
          current_value: Complex::new(640.0, 0.0),
          voltage_value: Complex::new(640.0, 0.0),
      }
  }
}