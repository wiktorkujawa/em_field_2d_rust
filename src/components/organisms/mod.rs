mod c_electric_details;
mod c_line_choice;
mod c_phase_order_choice;
mod c_bundle_specification;
mod c_arrangement_method;
mod c_coordinates;
mod c_routes;
mod c_range_inputs;
mod c_plot;

pub use crate::components::organisms::{
  c_electric_details::electric_details,
  c_line_choice::line_choice,
  c_phase_order_choice::phase_order_choice,
  c_bundle_specification::bundle_specification,
  c_arrangement_method::arrangement_method,
  c_coordinates::coordinates,
  c_routes::routes,
  c_range_inputs::range_inputs,
  c_plot::plot
};
