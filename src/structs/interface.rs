use crate::enums::{ ArrangementMethod, LineType };

use super::{ BundleSpec, ElectricInfo };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Interface {
    pub electric_info: ElectricInfo,
    pub bundle_spec: BundleSpec,
    pub arrangement_method: ArrangementMethod,
    pub center_position: [f64; 2],
    pub route_coor: Vec<[f64; 2]>,
    pub lines_gap: f64,
    pub line_type: LineType,
    pub x_range: [f64; 3],
    pub y_range: [f64; 3],
}

impl Default for Interface {
    fn default() -> Self {
        Self {
            bundle_spec: BundleSpec::default(),
            electric_info: ElectricInfo::default(),
            arrangement_method: ArrangementMethod::Various,
            line_type: LineType::PowerLine,
            center_position: [0.0, 0.0],
            lines_gap: 4.0,
            route_coor: vec![],
            x_range: [-10.0, 0.2, 10.0],
            y_range: [-10.0, 0.2, 10.0],
        }
    }
}
