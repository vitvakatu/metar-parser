use crate::Annotated;

mod kind;
mod percipitation;
mod pressure;
mod station;
mod temperature;
mod time;
mod visibility;
mod wind;

pub use kind::ReportKind;
pub use percipitation::Intensity;
pub use percipitation::Percipitation;
pub use pressure::Pressure;
pub use station::Station;
pub use temperature::Temperature;
pub use time::Time;
pub use visibility::Visibility;
pub use wind::Wind;

#[derive(Debug, PartialEq, Eq)]
pub struct Report<'a> {
    pub origin: &'a str,
    pub kind: Annotated<'a, ReportKind>,
    pub station: Annotated<'a, Station<'a>>,
    pub time: Annotated<'a, Time>,
    pub wind: Annotated<'a, Wind>,
    pub visibility: Annotated<'a, Visibility>,
    pub percipitation: Option<Annotated<'a, Percipitation>>,
    pub clouds: Vec<Annotated<'a, CloudLayer>>,
    pub temperature: Annotated<'a, Temperature>,
    pub pressure: Annotated<'a, Pressure>,
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
