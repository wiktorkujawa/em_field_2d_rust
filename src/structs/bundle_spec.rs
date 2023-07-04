use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct BundleSpec { // optional
  pub thickness: f64,
  pub number_per_bundle: u32,
  pub spacing: f64
}

impl Default for BundleSpec {
  fn default() -> Self {
      Self {
        thickness: 4.0,
        number_per_bundle: 1,
        spacing: 4.0
      }
  }
}