use std::fmt;
use std::fmt::Display;

use serde::Serialize;
use snafu::Snafu;

use crate::{
    Annotated,
    parser::{Context, Parse},
};

#[derive(Debug, Snafu, PartialEq, Serialize)]
#[snafu(display("Unknown station"))]
pub struct UnknownStation;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Station<'a> {
    pub icao_code: &'a str,
    pub name: String,
    pub country: String,
}

pub struct Parser;

read_stations::read_stations!();

impl Display for Station<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {})", self.icao_code, self.name, self.country)
    }
}

impl<'a> Parse<'a> for Parser {
    type Output = Annotated<'a, Station<'a>>;
    type Err = Annotated<'a, UnknownStation>;

    fn from_str(&self, context: &Context<'a>) -> Result<Self::Output, Self::Err> {
        let station = KNOWN_STATIONS.get(context.current());
        if let Some(station) = station {
            return Ok(context.annotate(station.clone()));
        }
        Err(context.annotate(UnknownStation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_ulli() {
        let input = "ULLI";
        let parser = Parser;
        let context = Context::new(input);
        let station: Annotated<Station> = parser.from_str(&context).unwrap();
        assert_eq!(station, Annotated {
            inner: Station {
                icao_code: input,
                name: "Pulkovo Airport".to_owned(),
                country: "Russia".to_owned(),
            },
            origin: input,
            start: 0,
            end: 4,
        });
    }

    #[test]
    fn test_station_ehle() {
        let input = "EHLE";
        let parser = Parser;
        let context = Context::new(input);
        let station: Annotated<Station> = parser.from_str(&context).unwrap();
        assert_eq!(station, Annotated {
            inner: Station {
                icao_code: input,
                name: "Lelystad Airport".to_owned(),
                country: "Netherlands".to_owned(),
            },
            origin: input,
            start: 0,
            end: 4,
        });
    }

    #[test]
    fn test_station_unknown() {
        let input = "ZZZZ";
        let parser = Parser;
        let context = Context::new(input);
        let station: Result<Annotated<Station>, _> = parser.from_str(&context);
        assert!(station.is_err());
        assert_eq!(station.unwrap_err(), Annotated {
            inner: UnknownStation,
            origin: input,
            start: 0,
            end: 4,
        });
    }
}
