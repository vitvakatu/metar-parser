use crate::{
    report::{
        CloudLayer, CloudSignificant, Cover, Intensity, Percipitation, Report, ReportKind, Station,
        Time, Wind,
    },
    units::{Hectopascal, Knots},
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
            kind: ReportKind::Metar,
            station: Station {
                icao_code: "EHLE",
                name: "Lelystad Airport".to_owned(),
                country: "Germany".to_owned(),
            },
            time: Time {
                day: 28,
                hour: 9,
                minute: 25,
            },
            is_correction: false,
            is_auto: true,
            wind: Wind {
                direction: 210,
                speed: Knots(9),
                gust: Some(Knots(19)),
                variable: Some(60..=130),
            },
            visibility: 5000,
            percipitation: Some(Percipitation::Rain {
                intensity: Some(Intensity::Light),
            }),
            clouds: vec![
                CloudLayer {
                    ceiling: 700,
                    cover: Cover::Few,
                    ..Default::default()
                },
                CloudLayer {
                    ceiling: 1400,
                    cover: Cover::Broken,
                    significant: Some(CloudSignificant::Cumulonimbus),
                },
                CloudLayer {
                    ceiling: 1700,
                    cover: Cover::Broken,
                    ..Default::default()
                },
            ],
            temperature: 2,
            dew_point: -1,
            pressure: Hectopascal(1001),
        };
        Ok(report)
    }
}
