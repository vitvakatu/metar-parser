use std::num::ParseIntError;

use crate::{
    Annotated, ResultExt,
    parser::{Context, Parse},
};
use snafu::prelude::*;

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Invalid cloud layer format"))]
    InvalidFormat,
    #[snafu(display("Not an integer"))]
    NotAnInteger { source: ParseIntError },
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
    Thunderstorm,
}

impl<'a> Parse<'a> for Annotated<'a, CloudLayer> {
    type Err = Annotated<'a, Error>;

    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err> {
        parse_cloud_layer(context.current()).annotate(context)
    }
}

fn parse_cloud_layer(value: &str) -> Result<CloudLayer, Error> {
    ensure!(value.len() >= 6, InvalidFormatSnafu);
    let cover = &value[0..3];
    let ceiling = &value[3..6];
    let significant = match &value.get(6..) {
        Some("CB") => Some(CloudSignificant::Cumulonimbus),
        Some("TS") => Some(CloudSignificant::Thunderstorm),
        _ => None,
    };
    let cover = match cover {
        "FEW" => Cover::Few,
        "SCT" => Cover::Scattered,
        "BKN" => Cover::Broken,
        "OVC" => Cover::Overcast,
        _ => return InvalidFormatSnafu.fail(),
    };
    let ceiling = ceiling.parse::<u32>().context(NotAnIntegerSnafu)? * 100;
    let layer = CloudLayer {
        ceiling,
        cover,
        significant,
    };
    Ok(layer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_layer() {
        let input = "BKN014CB";
        let context = Context::new(input);
        let cloud_layer: Annotated<CloudLayer> = Parse::from_str(&context).unwrap();
        assert_eq!(
            cloud_layer,
            Annotated::with_range(
                CloudLayer {
                    ceiling: 1400,
                    cover: Cover::Broken,
                    significant: Some(CloudSignificant::Cumulonimbus),
                },
                input,
                0..8
            )
        );
    }
}
