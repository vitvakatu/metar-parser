use crate::Annotated;

mod kind;
mod percipitation;
mod pressure;
mod station;
mod temperature;
mod time;
mod visibility;
mod wind;
mod clouds;

pub use kind::ReportKind;
pub use percipitation::Intensity;
pub use percipitation::Percipitation;
pub use pressure::Pressure;
pub use station::Station;
pub use temperature::Temperature;
pub use time::Time;
pub use visibility::Visibility;
pub use wind::Wind;
pub use clouds::CloudLayer;
pub use clouds::Cover;
pub use clouds::CloudSignificant;

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
