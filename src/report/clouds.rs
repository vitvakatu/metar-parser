use std::{
    fmt::{self, Display},
    num::ParseIntError,
};

use crate::{
    Annotated, ResultExt,
    parser::{Context, Parse},
};
use serde::Serialize;
use snafu::prelude::*;

#[derive(Debug, Snafu, PartialEq, Serialize)]
pub enum Error {
    #[snafu(display("Invalid cloud layer format"))]
    InvalidFormat,
    #[snafu(display("Not an integer: {source}"))]
    NotAnInteger {
        #[serde(skip)]
        source: ParseIntError,
    },
}

pub struct Parser;

#[derive(Default, Debug, PartialEq, Eq, Serialize)]
pub struct CloudLayer {
    pub ceiling: u32,
    pub cover: Cover,
    pub significant: Option<CloudSignificant>,
}

impl Display for CloudLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.cover {
            Cover::Clear => write!(f, "no significant clouds")?,
            Cover::Few => write!(f, "few clouds")?,
            Cover::Scattered => write!(f, "scattered clouds")?,
            Cover::Broken => write!(f, "broken clouds")?,
            Cover::Overcast => write!(f, "overcast")?,
        }
        write!(f, " at {}ft", self.ceiling)?;
        if let Some(significant) = &self.significant {
            match significant {
                CloudSignificant::Cumulonimbus => write!(f, ", cumulonimbus")?,
                CloudSignificant::Thunderstorm => write!(f, ", thunderstorm")?,
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug, PartialEq, Eq, Serialize)]
pub enum Cover {
    #[default]
    Clear,
    Few,
    Scattered,
    Broken,
    Overcast,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum CloudSignificant {
    Cumulonimbus,
    Thunderstorm,
}

impl<'a> Parse<'a> for Parser {
    type Output = Annotated<'a, CloudLayer>;
    type Err = Annotated<'a, Error>;

    fn from_str(&self, context: &Context<'a>) -> Result<Self::Output, Self::Err> {
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
        let parser = Parser;
        let context = Context::new(input);
        let cloud_layer = parser.from_str(&context).unwrap();
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
