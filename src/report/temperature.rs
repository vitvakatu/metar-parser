use std::fmt::{self, Display};
use std::num::ParseIntError;

use crate::ResultExt;
use crate::{
    Annotated,
    parser::{Context, Parse},
};
use snafu::prelude::*;

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Invalid format, expected <value>/<dew_point>"))]
    InvalidFormat,
    #[snafu(display("Invalid temperature: {source}"))]
    InvalidTemperature { source: ParsingError },
    #[snafu(display("Invalid dew point: {source}"))]
    InvalidDewPoint { source: ParsingError },
}

#[derive(Debug, Snafu, PartialEq)]
pub enum ParsingError {
    #[snafu(display("Negative values must be prefixed with 'M'"))]
    InvalidNegativeValue,
    #[snafu(display("Invalid digits count, expected 2"))]
    InvalidDigitsCount,
    #[snafu(display("Not an integer: {source}"))]
    NotAnInteger { source: ParseIntError },
}

pub struct Parser;

#[derive(Debug, PartialEq, Eq)]
pub struct Temperature {
    pub value: i32,
    pub dew_point: i32,
}

impl Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "temperature {}°C, dew point {}°C",
            self.value, self.dew_point
        )
    }
}

impl<'a> Parse<'a> for Parser {
    type Output = Annotated<'a, Temperature>;
    type Err = Annotated<'a, Error>;

    fn from_str(&self, context: &Context<'a>) -> Result<Self::Output, Self::Err> {
        let mut parts = context.current().split('/');
        let value = parts
            .next()
            .context(InvalidFormatSnafu)
            .annotate_err(context)?;
        let dew_point = parts
            .next()
            .context(InvalidFormatSnafu)
            .annotate_err(context)?;

        let temperature = parse_individual_temperature(value)
            .context(InvalidTemperatureSnafu)
            .annotate_err(context)?;
        let dew_point = parse_individual_temperature(dew_point)
            .context(InvalidDewPointSnafu)
            .annotate_err(context)?;

        Ok(context.annotate(Temperature {
            value: temperature,
            dew_point,
        }))
    }
}

fn parse_individual_temperature(value: &str) -> Result<i32, ParsingError> {
    let value = if value.starts_with('M') {
        &value[1..]
    } else {
        value
    };
    ensure!(!value.contains('-'), InvalidNegativeValueSnafu);
    ensure!(value.len() == 2, InvalidDigitsCountSnafu);
    let value = value.parse::<i32>().context(NotAnIntegerSnafu)?;
    Ok(value)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_temperature() {
        let input = "10/05";
        let parser = Parser;
        let context = Context::new(input);
        let temperature: Annotated<Temperature> = parser.from_str(&context).unwrap();
        assert_eq!(
            temperature,
            Annotated::new(
                Temperature {
                    value: 10,
                    dew_point: 5
                },
                input
            )
        );
    }
}
