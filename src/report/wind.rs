use crate::units::Knots;
use crate::{
    Annotated,
    parser::{Context, Parse},
};
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

    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err> {
        if context.current().len() < 5 {
            return Err(context.annotate(()));
        }
        let direction = context.current()[0..3].parse().unwrap();
        let speed = context.current()[3..5].parse().unwrap();
        let wind = Wind {
            direction,
            speed: Knots(speed),
            gust: None,
            variable: None,
        };

        Ok(context.annotate(wind))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_wind() {
        let input = "18010KT";
        let context = Context::new(input);
        let wind: Annotated<Wind> = Parse::from_str(&context).unwrap();
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
