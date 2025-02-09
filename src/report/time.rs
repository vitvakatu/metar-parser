use std::num::ParseIntError;

use crate::{
    Annotated, ResultExt,
    parser::{Context, Parse},
};
use snafu::prelude::*;

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Invalid format, expected <day><hour><minute>Z"))]
    InvalidFormat,
    #[snafu(display("No Zulu designator at the end"))]
    NoZuluDesignator,
    #[snafu(display("Not an integer: {source}"))]
    NotAnInteger { source: ParseIntError },
    #[snafu(display("Day not in range (1-31): {value}"))]
    DayNotInRange { value: u8 },
    #[snafu(display("Hour not in range (0-23): {value}"))]
    HourNotInRange { value: u8 },
    #[snafu(display("Minute not in range (0-59): {value}"))]
    MinuteNotInRange { value: u8 },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Time {
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

impl<'a> Parse<'a> for Annotated<'a, Time> {
    type Err = Annotated<'a, Error>;

    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err> {
        parse_date_time_internal(context.current()).annotate(context)
    }
}

fn parse_date_time_internal(value: &str) -> Result<Time, Error> {
    ensure!(value.len() == 7, InvalidFormatSnafu);
    ensure!(value.ends_with('Z'), NoZuluDesignatorSnafu);

    let mut chunks = (0..(value.len() - 2)).step_by(2).map(|i| &value[i..i + 2]);
    let day = chunks.next().context(InvalidFormatSnafu)?;
    let hour = chunks.next().context(InvalidFormatSnafu)?;
    let minute = chunks.next().context(InvalidFormatSnafu)?;
    let day = day.parse::<u8>().context(NotAnIntegerSnafu)?;
    let hour = hour.parse::<u8>().context(NotAnIntegerSnafu)?;
    let minute = minute.parse::<u8>().context(NotAnIntegerSnafu)?;

    ensure!(day >= 1 && day <= 31, DayNotInRangeSnafu { value: day });
    ensure!(hour <= 23, HourNotInRangeSnafu { value: hour });
    ensure!(minute <= 59, MinuteNotInRangeSnafu { value: minute });

    Ok(Time { day, hour, minute })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        let input = "061235Z";
        let context = Context::new(input);
        let time: Annotated<Time> = Parse::from_str(&context).unwrap();
        assert_eq!(time, Annotated {
            inner: Time {
                day: 6,
                hour: 12,
                minute: 35
            },
            origin: input,
            start: 0,
            end: 7
        });
    }

    #[test]
    fn test_time_invalid_day() {
        let input = "321235Z";
        let context = Context::new(input);
        let time: Result<Annotated<Time>, _> = Parse::from_str(&context);
        assert!(time.is_err());
        assert_eq!(
            time.unwrap_err().inner.to_string(),
            "Day not in range (1-31): 32"
        );
    }
    #[test]
    fn test_time_missing_zulu() {
        let input = "061235L";
        let context = Context::new(input);
        let time: Result<Annotated<Time>, _> = Parse::from_str(&context);
        assert!(time.is_err());
        assert_eq!(
            time.unwrap_err().inner.to_string(),
            "No Zulu designator at the end"
        );
    }
}
