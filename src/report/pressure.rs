use crate::{Annotated, parser::Parse, units::Hectopascal};

#[derive(Debug, PartialEq, Eq)]
pub struct Pressure {
    pub value: Hectopascal,
}

impl<'a> Parse<'a> for Annotated<'a, Pressure> {
    type Err = Annotated<'a, ()>;

    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
        let value = s[1..].parse().unwrap();
        Ok(Annotated::new(
            Pressure {
                value: Hectopascal(value),
            },
            s,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure() {
        let input = "Q1013";
        let pressure: Annotated<Pressure> = Parse::from_str(input).unwrap();
        assert_eq!(pressure.inner.value, Hectopascal(1013));
    }
}
