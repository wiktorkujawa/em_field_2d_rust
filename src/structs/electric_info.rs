use serde::{ Deserialize, Serialize };

use crate::enums::PhaseOrder;

#[derive(Serialize, Deserialize, Debug)]
pub struct ElectricInfo {
    pub current_value: f64,
    pub voltage_value: f64, // optional
    pub phase_shift: f32,
    pub phase_order: PhaseOrder,
}

impl Default for ElectricInfo {
    fn default() -> Self {
        Self {
            current_value: 640.0,
            voltage_value: 123.0,
            phase_shift: 0.0,
            phase_order: PhaseOrder::ABC,
        }
    }
}
