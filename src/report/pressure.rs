use crate::{
    Annotated, ResultExt,
    parser::{Context, Parse},
    units::Hectopascal,
};
use serde::Serialize;
use snafu::prelude::*;
use std::{
    fmt::{self, Display},
    num::ParseIntError,
};

#[derive(Debug, Snafu, PartialEq, Serialize)]
pub enum Error {
    #[snafu(display("Invalid format, expected Q<value>"))]
    InvalidFormat,
    #[snafu(display("Not an integer: {source}"))]
    NotAnInteger {
        #[serde(skip)]
        source: ParseIntError,
    },
}

pub struct Parser;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Pressure {
    pub value: Hectopascal,
}

impl Display for Pressure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QNH {} hpa", self.value.0)
    }
}

impl<'a> Parse<'a> for Parser {
    type Output = Annotated<'a, Pressure>;
    type Err = Annotated<'a, Error>;

    fn from_str(&self, context: &Context<'a>) -> Result<Self::Output, Self::Err> {
        parse_pressure(context.current()).annotate(context)
    }
}

fn parse_pressure(value: &str) -> Result<Pressure, Error> {
    ensure!(value.starts_with('Q'), InvalidFormatSnafu);
    ensure!(value.len() == 5, InvalidFormatSnafu);
    let value = value[1..].parse::<u32>().context(NotAnIntegerSnafu)?;
    Ok(Pressure {
        value: Hectopascal(value),
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure() {
        let input = "Q1013";
        let parser = Parser;
        let context = Context::new(input);
        let pressure: Annotated<Pressure> = parser.from_str(&context).unwrap();
        assert_eq!(pressure.inner.value, Hectopascal(1013));
    }
}
