use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum LineType {
  PowerLine,
  UndergroundCable
}

pub const LINE_TYPE_LIST: [(LineType, &str); 2] = [
  (LineType::PowerLine, "Power line"),
  (LineType::UndergroundCable, "Underground Cable"),
];