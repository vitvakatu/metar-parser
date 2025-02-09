use crate::{
    Annotated,
    report::{
        CloudLayer, CloudSignificant, Cover, Intensity, Percipitation, Pressure, Report,
        ReportKind, Station, Temperature, Time, Visibility, Wind,
    },
    units::{Hectopascal, Knots, Meters},
};

pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn parse(&self) -> Result<Report<'a>, ()> {
        let report = Report {
            origin: self.input,
            kind: Annotated::with_range(ReportKind::Metar, self.input, 0..4),
            station: Annotated::with_range(
                Station {
                    icao_code: "EHLE",
                    name: "Lelystad Airport".to_owned(),
                    country: "Germany".to_owned(),
                },
                self.input,
                0..4,
            ),
            time: Annotated::with_range(
                Time {
                    day: 28,
                    hour: 9,
                    minute: 25,
                },
                self.input,
                4..10,
            ),
            wind: Annotated::with_range(
                Wind {
                    direction: 210,
                    speed: Knots(9),
                    gust: Some(Knots(19)),
                    variable: Some(60..=130),
                },
                self.input,
                10..20,
            ),
            visibility: Annotated::with_range(
                Visibility::Horizontal(Meters(5000)),
                self.input,
                20..25,
            ),
            percipitation: Some(Annotated::with_range(
                Percipitation::Rain {
                    intensity: Some(Intensity::Light),
                },
                self.input,
                25..30,
            )),
            clouds: vec![
                Annotated::with_range(
                    CloudLayer {
                        ceiling: 700,
                        cover: Cover::Few,
                        ..Default::default()
                    },
                    self.input,
                    30..35,
                ),
                Annotated::with_range(
                    CloudLayer {
                        ceiling: 1400,
                        cover: Cover::Broken,
                        significant: Some(CloudSignificant::Cumulonimbus),
                    },
                    self.input,
                    35..40,
                ),
                Annotated::with_range(
                    CloudLayer {
                        ceiling: 1700,
                        cover: Cover::Broken,
                        ..Default::default()
                    },
                    self.input,
                    40..45,
                ),
            ],
            temperature: Annotated::with_range(
                Temperature {
                    value: 2,
                    dew_point: -1,
                },
                self.input,
                45..47,
            ),
            pressure: Annotated::with_range(
                Pressure {
                    value: Hectopascal(1001),
                },
                self.input,
                49..54,
            ),
        };
        Ok(report)
    }
}

pub trait Parse<'a>: Sized {
    type Err;
    fn from_str(s: &'a str) -> Result<Self, Self::Err>;
}
