use crate::{
    Annotated, ResultExt,
    parser::{Context, Parse},
    units::Hectopascal,
};
use snafu::prelude::*;
use std::num::ParseIntError;

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Invalid format, expected Q<value>"))]
    InvalidFormat,
    #[snafu(display("Not an integer: {source}"))]
    NotAnInteger { source: ParseIntError },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pressure {
    pub value: Hectopascal,
}

impl<'a> Parse<'a> for Annotated<'a, Pressure> {
    type Err = Annotated<'a, Error>;

    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err> {
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
        let context = Context::new(input);
        let pressure: Annotated<Pressure> = Parse::from_str(&context).unwrap();
        assert_eq!(pressure.inner.value, Hectopascal(1013));
    }
}
