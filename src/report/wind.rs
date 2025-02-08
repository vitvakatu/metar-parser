use crate::Annotated;
use crate::{parser::Parse, units::Knots};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq)]
pub struct Wind {
    pub direction: u32,
    pub speed: Knots,
    pub gust: Option<Knots>,
    pub variable: Option<RangeInclusive<u32>>,
}

impl<'a> Parse<'a> for Annotated<'a, Wind> {
    type Err = Annotated<'a, ()>;

    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
        let wind = Wind {
            direction: 180,
            speed: Knots(10),
            gust: None,
            variable: None,
        };

        Ok(Annotated::new(wind, s))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_wind() {
        let input = "18010KT";
        let wind: Annotated<Wind> = Parse::from_str(input).unwrap();
        assert_eq!(
            wind,
            Annotated::new(
                Wind {
                    direction: 180,
                    speed: Knots(10),
                    gust: None,
                    variable: None,
                },
                input,
            )
        );
    }
}
