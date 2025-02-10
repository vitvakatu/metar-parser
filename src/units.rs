use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Hectopascal(pub u32);

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Knots(pub u32);

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Meters(pub u32);
