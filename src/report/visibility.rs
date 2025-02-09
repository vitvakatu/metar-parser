use crate::{
    Annotated,
    parser::{Context, Parse},
    units::Meters,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Visibility {
    Horizontal(Meters),
}

impl<'a> Parse<'a> for Annotated<'a, Visibility> {
    type Err = Annotated<'a, ()>;

    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err> {
        let value = context.current().parse().unwrap();
        let visibility = Visibility::Horizontal(Meters(value));
        Ok(context.annotate(visibility))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility() {
        let input = "1000";
        let context = Context::new(input);
        let visibility: Annotated<Visibility> = Parse::from_str(&context).unwrap();
        assert_eq!(
            visibility,
            Annotated::new(Visibility::Horizontal(Meters(1000)), input)
        );
    }
}
