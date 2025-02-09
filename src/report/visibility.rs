use std::num::ParseIntError;

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

impl<'a> Parse<'a> for Annotated<'a, Visibility> {
    type Err = Annotated<'a, Error>;

    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err> {
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
        let context = Context::new(input);
        let visibility: Annotated<Visibility> = Parse::from_str(&context).unwrap();
        assert_eq!(
            visibility,
            Annotated::new(Visibility::Horizontal(Meters(1000)), input)
        );
    }
}
