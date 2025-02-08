use crate::{
    Annotated,
    report::{
        CloudLayer, CloudSignificant, Cover, Intensity, Percipitation, Report, ReportKind, Station,
        Time, Wind,
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
            kind: Annotated::new(ReportKind::Metar, self.input, 0, 4),
            station: Annotated::new(
                Station {
                    icao_code: "EHLE",
                    name: "Lelystad Airport".to_owned(),
                    country: "Germany".to_owned(),
                },
                self.input,
                0,
                4,
            ),
            time: Annotated::new(
                Time {
                    day: 28,
                    hour: 9,
                    minute: 25,
                },
                self.input,
                4,
                10,
            ),
            is_correction: false,
            is_auto: true,
            wind: Annotated::new(
                Wind {
                    direction: 210,
                    speed: Knots(9),
                    gust: Some(Knots(19)),
                    variable: Some(60..=130),
                },
                self.input,
                10,
                20,
            ),
            visibility: Annotated::new(Meters(5000), self.input, 20, 25),
            percipitation: Some(Annotated::new(
                Percipitation::Rain {
                    intensity: Some(Intensity::Light),
                },
                self.input,
                25,
                30,
            )),
            clouds: vec![
                Annotated::new(
                    CloudLayer {
                        ceiling: 700,
                        cover: Cover::Few,
                        ..Default::default()
                    },
                    self.input,
                    30,
                    35,
                ),
                Annotated::new(
                    CloudLayer {
                        ceiling: 1400,
                        cover: Cover::Broken,
                        significant: Some(CloudSignificant::Cumulonimbus),
                    },
                    self.input,
                    35,
                    40,
                ),
                Annotated::new(
                    CloudLayer {
                        ceiling: 1700,
                        cover: Cover::Broken,
                        ..Default::default()
                    },
                    self.input,
                    40,
                    45,
                ),
            ],
            temperature: Annotated::new(2, self.input, 45, 47),
            dew_point: Annotated::new(-1, self.input, 47, 49),
            pressure: Annotated::new(Hectopascal(1001), self.input, 49, 54),
        };
        Ok(report)
    }
}
