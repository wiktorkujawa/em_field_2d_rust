use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum PhaseOrder {
  ABC,
  ACB,
  BAC,
  BCA,
  CAB,
  CBA
}

impl fmt::Display for PhaseOrder {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
      // or, alternatively:
      // fmt::Debug::fmt(self, f)
  }
}

pub const PHASE_ORDER_LIST: [(PhaseOrder, &str); 6] = [
  (PhaseOrder::ABC, "ABC"),
  (PhaseOrder::ACB, "ACB"),
  (PhaseOrder::BAC, "BAC"),
  (PhaseOrder::BCA, "BCA"),
  (PhaseOrder::CAB, "CAB"),
  (PhaseOrder::CBA, "CBA")
];