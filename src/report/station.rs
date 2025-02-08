use std::cell::LazyCell;
use std::collections::HashMap;

use crate::{Annotated, parser::Parse};

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
    type Err = Annotated<'a, ()>;

    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
        let stations = KNOWN_STATIONS;
        let station = stations.get(s);
        if let Some(station) = station {
            return Ok(Annotated::new(station.clone(), s));
        }
        Err(Annotated::new((), s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_ulli() {
        let input = "ULLI";
        let station: Annotated<Station> = Parse::from_str(input).unwrap();
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
        let station: Annotated<Station> = Parse::from_str(input).unwrap();
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
        let station: Result<Annotated<Station>, _> = Parse::from_str(input);
        assert!(station.is_err());
        assert_eq!(station.unwrap_err(), Annotated {
            inner: (),
            origin: input,
            start: 0,
            end: 4,
        });
    }
}
