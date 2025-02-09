use crate::{
    Annotated,
    parser::{Context, Parse},
};

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

    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err> {
        let percipitation = Percipitation::Rain { intensity: None };
        Ok(context.annotate(percipitation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rain() {
        let input = "RA";
        let context = Context::new(input);
        let percipitation: Annotated<Percipitation> = Parse::from_str(&context).unwrap();
        assert_eq!(percipitation.inner, Percipitation::Rain { intensity: None });
    }
}
