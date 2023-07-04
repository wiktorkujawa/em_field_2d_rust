// use num_complex::Complex;

use std::f64::consts::{SQRT_2, PI};

use crate::enums::{ LineType, ArrangementMethod, PhaseOrder };

use super::{ Conductor, BundleSpec, ElectricInfo, Interface };
use num_complex::{Complex, Complex64};
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    pub line_type: LineType,
    pub conductors: Vec<Conductor>,
    pub electric_info: ElectricInfo,
    pub bundle_spec: BundleSpec,
}

impl Route {
    pub fn new(Interface {
        electric_info: ElectricInfo { current_value, phase_order, phase_shift, voltage_value },
        bundle_spec,
        arrangement_method,
        center_position,
        route_coor,
        lines_gap,
        line_type,
        ..
    }: &mut Interface) -> Route {
        let mut route: Route = Route {
            line_type: *line_type,
            conductors: vec![],
            electric_info: ElectricInfo {
                current_value: *current_value,
                phase_order: *phase_order,
                phase_shift: *phase_shift,
                voltage_value: *voltage_value,
            },
            bundle_spec: *bundle_spec,
        };

        const PHASE_DIF: f64 = 2.0*PI/3.0;
        let phase_multiplier = match &phase_order {
            PhaseOrder::ABC => [0.0, -PHASE_DIF, PHASE_DIF],
            PhaseOrder::ACB => [0.0, PHASE_DIF, -PHASE_DIF],
            PhaseOrder::BAC => [-PHASE_DIF, 0.0, PHASE_DIF],
            PhaseOrder::BCA => [-PHASE_DIF, PHASE_DIF, 0.0],
            PhaseOrder::CAB => [PHASE_DIF, 0.0, -PHASE_DIF],
            PhaseOrder::CBA => [PHASE_DIF, -PHASE_DIF, 0.0],
        };
        let [first_phase, second_phase, third_phase] = phase_multiplier;
        let phase_shift_in_radians= *phase_shift as f64*PI/180.0;

        let shift_first_phase = phase_shift_in_radians + first_phase;
        let shift_second_phase = phase_shift_in_radians + second_phase;
        let shift_third_phase = phase_shift_in_radians + third_phase;


        match arrangement_method {
            ArrangementMethod::Various => {
                for [x, y] in route_coor {
                    route.conductors.push(Conductor {
                        x: *x,
                        y: *y,
                        current_value: Complex::new(640.0, 0.0),
                        voltage_value: Complex::new(640.0, 0.0),
                    });
                }
            }

            ArrangementMethod::Flat => {
                let [x, y] = *center_position;

                route.conductors.push(Conductor {
                    x: x - *lines_gap,
                    y,
                    current_value: *current_value * Complex64::cis(shift_first_phase),
                    voltage_value: *voltage_value * Complex64::cis(shift_first_phase),
                });
                route.conductors.push(Conductor {
                    x,
                    y,
                    current_value: *current_value * Complex::cis(shift_second_phase),
                    voltage_value: *voltage_value * Complex::cis(shift_second_phase),
                });
                route.conductors.push(Conductor {
                    x: x + *lines_gap,
                    y,
                    current_value: *current_value * Complex::cis(shift_third_phase),
                    voltage_value: *voltage_value * Complex::cis(shift_third_phase),
                });
            }
            ArrangementMethod::Trefoil => {
                let [x, y] = *center_position;
                route.conductors.push(Conductor {
                    x: x - *lines_gap * 0.5,
                    y: y - (*lines_gap * SQRT_2) / 6.0,
                    current_value: *current_value * Complex::cis(shift_first_phase),
                    voltage_value: *voltage_value * Complex::cis(shift_first_phase),
                });
                route.conductors.push(Conductor {
                    x,
                    y: y + (*lines_gap * SQRT_2) / 3.0,
                    current_value: *current_value * Complex::cis(shift_second_phase),
                    voltage_value: *voltage_value * Complex::cis(shift_second_phase),
                });
                route.conductors.push(Conductor {
                    x: x + *lines_gap * 0.5,
                    y: y - (*lines_gap * SQRT_2) / 6.0,
                    current_value: *current_value * Complex::cis(shift_third_phase),
                    voltage_value: *voltage_value * Complex::cis(shift_third_phase),
                });
            }
        }

        return route;
    }
}

impl Default for Route {
    fn default() -> Self {
        Self {
            electric_info: ElectricInfo::default(),
            line_type: LineType::PowerLine,
            conductors: vec![],
            bundle_spec: BundleSpec::default(),
        }
    }
}
