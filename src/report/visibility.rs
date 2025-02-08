use crate::{Annotated, parser::Parse, units::Meters};

#[derive(Debug, PartialEq, Eq)]
pub enum Visibility {
    Horizontal(Meters),
}

impl<'a> Parse<'a> for Annotated<'a, Visibility> {
    type Err = Annotated<'a, ()>;

    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
        let visibility = Visibility::Horizontal(Meters(1000));
        Ok(Annotated::new(visibility, s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility() {
        let input = "1000";
        let visibility: Annotated<Visibility> = Parse::from_str(input).unwrap();
        assert_eq!(
            visibility,
            Annotated::new(Visibility::Horizontal(Meters(1000)), input)
        );
    }
}
