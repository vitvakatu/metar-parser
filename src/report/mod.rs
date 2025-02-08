use std::ops::RangeInclusive;

use crate::{
    Annotated,
    units::{Hectopascal, Knots, Meters},
};

mod kind;
mod station;
mod time;

pub use kind::ReportKind;
pub use station::Station;
pub use time::Time;

#[derive(Debug, PartialEq, Eq)]
pub struct Report<'a> {
    pub origin: &'a str,
    pub kind: Annotated<'a, ReportKind>,
    pub station: Annotated<'a, Station<'a>>,
    pub time: Annotated<'a, Time>,
    pub is_correction: bool,
    pub is_auto: bool,
    pub wind: Annotated<'a, Wind>,
    pub visibility: Annotated<'a, Meters>,
    pub percipitation: Option<Annotated<'a, Percipitation>>,
    pub clouds: Vec<Annotated<'a, CloudLayer>>,
    pub temperature: Annotated<'a, i32>,
    pub dew_point: Annotated<'a, i32>,
    pub pressure: Annotated<'a, Hectopascal>,
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
