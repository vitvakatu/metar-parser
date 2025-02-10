use crate::Annotated;
use frunk::Generic;
use frunk::LabelledGeneric;

pub mod clouds;
pub mod kind;
pub mod percipitation;
pub mod pressure;
pub mod station;
pub mod temperature;
pub mod time;
pub mod visibility;
pub mod wind;

pub use clouds::CloudLayer;
pub use clouds::CloudSignificant;
pub use clouds::Cover;
pub use kind::ReportKind;
pub use percipitation::Intensity;
pub use percipitation::Percipitation;
pub use pressure::Pressure;
pub use station::Station;
pub use temperature::Temperature;
pub use time::Time;
pub use visibility::Visibility;
pub use wind::Wind;

#[derive(Debug, PartialEq, Eq, Generic, LabelledGeneric)]
pub struct Report<'a> {
    pub origin: &'a str,
    pub kind: Annotated<'a, ReportKind>,
    pub station: Annotated<'a, Station<'a>>,
    pub time: Annotated<'a, Time>,
    pub wind: Annotated<'a, Wind>,
    pub visibility: Annotated<'a, Visibility>,
    pub percipitation: Annotated<'a, Percipitation>,
    pub clouds: Annotated<'a, CloudLayer>,
    pub temperature: Annotated<'a, Temperature>,
    pub pressure: Annotated<'a, Pressure>,
}
