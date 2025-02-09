use std::cell::LazyCell;
use std::collections::HashMap;

use snafu::Snafu;

use crate::{
    Annotated,
    parser::{Context, Parse},
};

#[derive(Debug, Snafu, PartialEq)]
#[snafu(display("Unknown station"))]
pub struct UnknownStation;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Station<'a> {
    pub icao_code: &'a str,
    pub name: String,
    pub country: String,
}

pub const KNOWN_STATIONS: LazyCell<HashMap<&'static str, Station>> = LazyCell::new(|| {
    HashMap::from([
        ("ULLI", Station {
            icao_code: "ULLI",
            name: "Pulkovo Airport".to_owned(),
            country: "Russia".to_owned(),
        }),
        ("EHLE", Station {
            icao_code: "EHLE",
            name: "Lelystad Airport".to_owned(),
            country: "Netherlands".to_owned(),
        }),
    ])
});

impl<'a> Parse<'a> for Annotated<'a, Station<'a>> {
    type Err = Annotated<'a, UnknownStation>;

    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err> {
        let stations = KNOWN_STATIONS;
        let station = stations.get(context.current());
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
        let context = Context::new(input);
        let station: Annotated<Station> = Parse::from_str(&context).unwrap();
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
        let context = Context::new(input);
        let station: Annotated<Station> = Parse::from_str(&context).unwrap();
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
        let context = Context::new(input);
        let station: Result<Annotated<Station>, _> = Parse::from_str(&context);
        assert!(station.is_err());
        assert_eq!(station.unwrap_err(), Annotated {
            inner: UnknownStation,
            origin: input,
            start: 0,
            end: 4,
        });
    }
}
