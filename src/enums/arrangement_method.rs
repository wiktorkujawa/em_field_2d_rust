use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ArrangementMethod {
  Various,
  Flat,
  Trefoil
}

pub const ARRANGE_METHODS_LIST: [(ArrangementMethod, &str); 3] = [
  (ArrangementMethod::Various, "Various positions"),
  (ArrangementMethod:: Flat, "Flat formation"),
  (ArrangementMethod::Trefoil, "Trefoil formation")
];