use std::ops::RangeInclusive;

use crate::units::{Hectopascal, Knots};

#[derive(Debug, PartialEq, Eq)]
pub struct Report<'a> {
    pub origin: &'a str,
    pub kind: ReportKind,
    pub station: Station<'a>,
    pub time: Time,
    pub is_correction: bool,
    pub is_auto: bool,
    pub wind: Wind,
    pub visibility: u32,
    pub percipitation: Option<Percipitation>,
    pub clouds: Vec<CloudLayer>,
    pub temperature: i32,
    pub dew_point: i32,
    pub pressure: Hectopascal,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReportKind {
    Metar,
    Speci,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Station<'a> {
    pub icao_code: &'a str,
    pub name: String,
    pub country: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Time {
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Wind {
    pub direction: u32,
    pub speed: Knots,
    pub gust: Option<Knots>,
    pub variable: Option<RangeInclusive<u32>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Percipitation {
    Rain { intensity: Option<Intensity> },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Intensity {
    Light,
    Heavy,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct CloudLayer {
    pub ceiling: u32,
    pub cover: Cover,
    pub significant: Option<CloudSignificant>,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Cover {
    #[default]
    Clear,
    Few,
    Scattered,
    Broken,
    Overcast,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CloudSignificant {
    Cumulonimbus,
}
