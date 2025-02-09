use std::{ops::Range, str::SplitWhitespace};

use crate::{
    Annotated,
    report::{
        CloudLayer, Percipitation, Pressure, Report, ReportKind, Station, Temperature, Time,
        Visibility, Wind,
    },
};

pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn parse(&self) -> Result<Report<'a>, ()> {
        let mut context = Context::new(self.input);
        let kind: Annotated<ReportKind> = Parse::from_str(&context).unwrap();
        context.advance();
        let station: Annotated<Station> = Parse::from_str(&context).unwrap();
        context.advance();
        let time: Annotated<Time> = Parse::from_str(&context).unwrap();
        context.advance();
        let wind: Annotated<Wind> = Parse::from_str(&context).unwrap();
        context.advance();
        let visibility: Annotated<Visibility> = Parse::from_str(&context).unwrap();
        context.advance();
        let percipitation: Annotated<Percipitation> = Parse::from_str(&context).unwrap();
        context.advance();
        let clouds: Annotated<CloudLayer> = Parse::from_str(&context).unwrap();
        context.advance();
        let temperature: Annotated<Temperature> = Parse::from_str(&context).unwrap();
        context.advance();
        let pressure: Annotated<Pressure> = Parse::from_str(&context).unwrap();
        let report = Report {
            kind,
            station,
            time,
            wind,
            visibility,
            origin: self.input,
            percipitation: Some(percipitation),
            clouds: vec![clouds],
            temperature,
            pressure,
        };
        Ok(report)
    }
}

pub(crate) struct Context<'a> {
    input: &'a str,
    parts: SplitWhitespace<'a>,
    current: Range<usize>,
    is_started: bool,
}

impl<'a> Context<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut context = Self {
            input,
            parts: input.split_whitespace(),
            current: 0..0,
            is_started: false,
        };
        context.advance();
        context
    }

    pub fn advance(&mut self) -> bool {
        let current_len = self.current.len();
        let space_width = if self.is_started { 1 } else { 0 };
        self.is_started = true;
        let Some(next_part) = self.parts.next() else {
            return false;
        };
        self.current = (self.current.start + current_len + space_width)
            ..(self.current.end + space_width + next_part.len());
        true
    }

    pub fn current(&self) -> &'a str {
        &self.input[self.current.clone()]
    }

    pub fn annotate<T>(&self, value: T) -> Annotated<'a, T> {
        Annotated::with_range(value, self.input, self.current.clone())
    }
}

pub(crate) trait Parse<'a>: Sized {
    type Err;
    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let mut context = Context::new("Hello world");
        assert_eq!(context.current(), "Hello");
        assert!(context.advance());
        assert_eq!(context.current(), "world");
        assert!(!context.advance());
    }
}
