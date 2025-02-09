use crate::{Annotated, parser::Parse};

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
    type Err = Annotated<'a, ()>;

    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
        Ok(Annotated::new(
            CloudLayer {
                ceiling: 1400,
                cover: Cover::Broken,
                significant: None,
            },
            s,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_layer() {
        let input = "BKN014CB";
        let cloud_layer: Annotated<CloudLayer> = Parse::from_str(input).unwrap();
        assert_eq!(
            cloud_layer,
            Annotated::with_range(
                CloudLayer {
                    ceiling: 1400,
                    cover: Cover::Broken,
                    significant: None,
                },
                input,
                0..8
            )
        );
    }
}
