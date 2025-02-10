use std::fmt::{self, Display};

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

pub struct Parser;

#[derive(Debug, PartialEq, Eq)]
pub enum Percipitation {
    Rain { intensity: Option<Intensity> },
}

impl Display for Percipitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rain { intensity } => {
                if let Some(intensity) = intensity {
                    match intensity {
                        Intensity::Light => write!(f, "light ")?,
                        Intensity::Heavy => write!(f, "heavy ")?,
                    }
                }
                write!(f, "rain")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Intensity {
    Light,
    Heavy,
}

impl<'a> Parse<'a> for Parser {
    type Output = Annotated<'a, Percipitation>;
    type Err = Annotated<'a, Error>;

    fn from_str(&self, context: &Context<'a>) -> Result<Self::Output, Self::Err> {
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
        let parser = Parser;
        let context = Context::new(input);
        let percipitation: Annotated<Percipitation> = parser.from_str(&context).unwrap();
        assert_eq!(percipitation.inner, Percipitation::Rain { intensity: None });
    }
}
