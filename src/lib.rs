use std::ops::Range;

use parser::Context;
use serde::Serialize;
pub mod parser;
pub mod report;
pub mod units;

pub mod prelude {
    pub use super::{
        Annotated,
        parser::Parser,
        report::{
            CloudLayer, CloudSignificant, Cover, Intensity, Percipitation, Pressure, Report,
            ReportKind, Station, Temperature, Time, Visibility, Wind,
        },
        units::{Hectopascal, Knots, Meters},
    };
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Annotated<'a, T> {
    pub inner: T,
    pub origin: &'a str,
    pub start: usize,
    pub end: usize,
}

pub(crate) trait ResultExt<'a> {
    type FullyAnnotated;
    type ErrorAnnotated;
    fn annotate(self, context: &Context<'a>) -> Self::FullyAnnotated;
    fn annotate_err(self, context: &Context<'a>) -> Self::ErrorAnnotated;
}

impl<'a, T, E> ResultExt<'a> for Result<T, E> {
    type FullyAnnotated = Result<Annotated<'a, T>, Annotated<'a, E>>;
    type ErrorAnnotated = Result<T, Annotated<'a, E>>;

    fn annotate(self, context: &Context<'a>) -> Self::FullyAnnotated {
        match self {
            Ok(value) => Ok(context.annotate(value)),
            Err(error) => Err(context.annotate(error)),
        }
    }

    fn annotate_err(self, context: &Context<'a>) -> Self::ErrorAnnotated {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(context.annotate(error)),
        }
    }
}

impl<'a, T> Annotated<'a, T> {
    pub fn new(inner: T, origin: &'a str) -> Self {
        Self {
            inner,
            origin,
            start: 0,
            end: origin.len(),
        }
    }

    pub fn with_range(inner: T, origin: &'a str, range: Range<usize>) -> Self {
        Self {
            inner,
            origin,
            start: range.start,
            end: range.end,
        }
    }

    pub fn unit(&self) -> Annotated<'a, ()> {
        Annotated::with_range((), self.origin, self.start..self.end)
    }
}
