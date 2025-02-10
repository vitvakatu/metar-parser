use crate::units::Knots;
use crate::{
    Annotated, ResultExt,
    parser::{Context, Parse},
};
use snafu::prelude::*;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::ops::RangeInclusive;

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Invalid format, expected <direction><speed>KT"))]
    InvalidFormat,
    #[snafu(display("Invalid direction (0-360): {value}"))]
    InvalidDirection { value: u32 },
    #[snafu(display("Invalid speed (0-49): {value}"))]
    InvalidSpeed { value: u32 },
    #[snafu(display("Invalid gust (0-49): {value}"))]
    InvalidGust { value: u32 },
    #[snafu(display("Not an integer: {source}"))]
    NotAnInteger { source: ParseIntError },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Wind {
    pub direction: u32,
    pub speed: Knots,
    pub gust: Option<Knots>,
    pub variable: Option<RangeInclusive<u32>>,
}

impl Display for Wind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "wind {}° {}KT", self.direction, self.speed.0)?;
        if let Some(gust) = &self.gust {
            write!(f, " gust {}KT", gust.0)?;
        }
        if let Some(variable) = &self.variable {
            write!(
                f,
                " variable between {}° and {}°",
                variable.start(),
                variable.end()
            )?;
        }
        Ok(())
    }
}

pub struct Parser;

impl<'a> Parse<'a> for Parser {
    type Output = Annotated<'a, Wind>;
    type Err = Annotated<'a, Error>;

    fn from_str(&self, context: &Context<'a>) -> Result<Self::Output, Self::Err> {
        parse_wind_internal(context.current()).annotate(context)
    }
}

fn parse_wind_internal(value: &str) -> Result<Wind, Error> {
    ensure!(value.len() >= 7, InvalidFormatSnafu);
    ensure!(value.ends_with("KT"), InvalidFormatSnafu);

    let direction = value[0..3].parse::<u32>().context(NotAnIntegerSnafu)?;
    let speed = value[3..5].parse::<u32>().context(NotAnIntegerSnafu)?;

    ensure!(direction <= 360, InvalidDirectionSnafu { value: direction });
    ensure!(speed <= 49, InvalidSpeedSnafu { value: speed });

    let gust = if value.len() == 10 {
        let gust = value[6..8].parse::<u32>().context(NotAnIntegerSnafu)?;
        ensure!(gust <= 49, InvalidGustSnafu { value: gust });
        Some(Knots(gust))
    } else {
        None
    };

    Ok(Wind {
        direction,
        speed: Knots(speed),
        gust,
        variable: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wind() {
        let input = "18010KT";
        let parser = Parser;
        let context = Context::new(input);
        let wind: Annotated<Wind> = parser.from_str(&context).unwrap();
        assert_eq!(
            wind,
            Annotated::new(
                Wind {
                    direction: 180,
                    speed: Knots(10),
                    gust: None,
                    variable: None,
                },
                input,
            )
        );
    }
}
