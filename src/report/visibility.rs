use std::{
    fmt::{self, Display},
    num::ParseIntError,
};

use crate::{
    Annotated, ResultExt,
    parser::{Context, Parse},
    units::Meters,
};
use snafu::prelude::*;

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Invalid visibility (0-9999)"))]
    InvalidVisibility,
    #[snafu(display("Not an integer: {source}"))]
    NotAnInteger { source: ParseIntError },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Visibility {
    Horizontal(Meters),
}

impl Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Horizontal(distance) => write!(f, "visibility {} m", distance.0)?,
        }
        Ok(())
    }
}

pub struct Parser;

impl<'a> Parse<'a> for Parser {
    type Output = Annotated<'a, Visibility>;
    type Err = Annotated<'a, Error>;

    fn from_str(&self, context: &Context<'a>) -> Result<Self::Output, Self::Err> {
        parse_visibility(context.current()).annotate(context)
    }
}

fn parse_visibility(value: &str) -> Result<Visibility, Error> {
    let value = value.parse::<u32>().context(NotAnIntegerSnafu)?;
    ensure!(value <= 9999, InvalidVisibilitySnafu);
    Ok(Visibility::Horizontal(Meters(value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility() {
        let input = "1000";
        let parser = Parser;
        let context = Context::new(input);
        let visibility: Annotated<Visibility> = parser.from_str(&context).unwrap();
        assert_eq!(
            visibility,
            Annotated::new(Visibility::Horizontal(Meters(1000)), input)
        );
    }
}
