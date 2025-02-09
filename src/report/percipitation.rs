use crate::{
    Annotated, ResultExt,
    parser::{Context, Parse},
};
use snafu::prelude::*;

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Invalid percpitation format"))]
    InvalidFormat,
    #[snafu(display("Unknown percpitation"))]
    UnknownPercipitation,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Percipitation {
    Rain { intensity: Option<Intensity> },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Intensity {
    Light,
    Heavy,
}

impl<'a> Parse<'a> for Annotated<'a, Percipitation> {
    type Err = Annotated<'a, Error>;

    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err> {
        parse_rain(context.current()).annotate(context)
    }
}

fn parse_rain(value: &str) -> Result<Percipitation, Error> {
    ensure!(value.len() >= 2, InvalidFormatSnafu);
    let intensity = match value.chars().next().unwrap() {
        '-' => Some(Intensity::Light),
        '+' => Some(Intensity::Heavy),
        _ => None,
    };
    match &value[value.len() - 2..] {
        "RA" => Ok(Percipitation::Rain { intensity }),
        _ => UnknownPercipitationSnafu.fail(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rain() {
        let input = "RA";
        let context = Context::new(input);
        let percipitation: Annotated<Percipitation> = Parse::from_str(&context).unwrap();
        assert_eq!(percipitation.inner, Percipitation::Rain { intensity: None });
    }
}
