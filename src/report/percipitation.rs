use crate::{Annotated, parser::Parse};

#[derive(Debug, PartialEq, Eq)]
pub enum Percipitation {
    Rain { intensity: Option<Intensity> },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Intensity {
    Light,
    Heavy,
}

impl<'a> Parse<'a> for Annotated<'a, Percipitation> {
    type Err = Annotated<'a, ()>;

    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
        Ok(Annotated::new(Percipitation::Rain { intensity: None }, s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rain() {
        let input = "RA";
        let percipitation: Annotated<Percipitation> = Parse::from_str(input).unwrap();
        assert_eq!(percipitation.inner, Percipitation::Rain { intensity: None });
    }
}
