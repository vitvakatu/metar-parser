pub mod parser;
pub mod report;
pub mod units;

pub mod prelude {
    pub use super::{
        Annotated,
        parser::Parser,
        report::{
            CloudLayer, CloudSignificant, Cover, Intensity, Percipitation, Report, ReportKind,
            Station, Time, Wind,
        },
        units::{Hectopascal, Knots, Meters},
    };
}

#[derive(Debug, PartialEq, Eq)]
pub struct Annotated<'a, T> {
    pub inner: T,
    pub origin: &'a str,
    pub start: usize,
    pub end: usize,
}

impl<'a, T> Annotated<'a, T> {
    pub fn new(inner: T, origin: &'a str, start: usize, end: usize) -> Self {
        Self {
            inner,
            origin,
            start,
            end,
        }
    }
}
