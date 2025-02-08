use std::ops::RangeInclusive;

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

#[derive(Debug, PartialEq, Eq)]
pub struct Report<'a> {
    pub origin: &'a str,
    pub kind: ReportKind,
    pub station: Station<'a>,
    pub time: Time,
    pub is_correction: bool,
    pub is_auto: bool,
    pub wind: Wind,
    pub visibility: u32,
    pub percipitation: Option<Percipitation>,
    pub clouds: Vec<CloudLayer>,
    pub temperature: i32,
    pub dew_point: i32,
    pub pressure: Hectopascal,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Knots(pub u32);

#[derive(Debug, PartialEq, Eq)]
pub enum ReportKind {
    Metar,
    Speci,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Station<'a> {
    pub icao_code: &'a str,
    pub name: String,
    pub country: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Time {
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Wind {
    pub direction: u32,
    pub speed: Knots,
    pub gust: Option<Knots>,
    pub variable: Option<RangeInclusive<u32>>,
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

#[derive(Default, Debug, PartialEq, Eq)]
pub struct CloudLayer {
    pub ceiling: u32,
    pub cover: Cover,
    pub significant: Option<CloudSignificant>,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Cover {
    #[default]
    Clear,
    Few,
    Scattered,
    Broken,
    Overcast,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CloudSignificant {
    Cumulonimbus,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hectopascal(pub u32);
