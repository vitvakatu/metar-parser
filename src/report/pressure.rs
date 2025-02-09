use crate::{
    Annotated,
    parser::{Context, Parse},
    units::Hectopascal,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Pressure {
    pub value: Hectopascal,
}

impl<'a> Parse<'a> for Annotated<'a, Pressure> {
    type Err = Annotated<'a, ()>;

    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err> {
        let value = context.current()[1..].parse().unwrap();
        Ok(context.annotate(Pressure {
            value: Hectopascal(value),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure() {
        let input = "Q1013";
        let context = Context::new(input);
        let pressure: Annotated<Pressure> = Parse::from_str(&context).unwrap();
        assert_eq!(pressure.inner.value, Hectopascal(1013));
    }
}
