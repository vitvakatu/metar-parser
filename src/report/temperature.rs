use crate::{
    Annotated,
    parser::{Context, Parse},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Temperature {
    pub value: i32,
    pub dew_point: i32,
}

impl<'a> Parse<'a> for Annotated<'a, Temperature> {
    type Err = Annotated<'a, ()>;

    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err> {
        let mut parts = context.current().split('/');
        let temperature = Temperature {
            value: parts.next().unwrap().parse().unwrap(),
            dew_point: parts.next().unwrap().parse().unwrap(),
        };
        Ok(context.annotate(temperature))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_temperature() {
        let input = "10/05";
        let context = Context::new(input);
        let temperature: Annotated<Temperature> = Parse::from_str(&context).unwrap();
        assert_eq!(
            temperature,
            Annotated::new(
                Temperature {
                    value: 10,
                    dew_point: 5
                },
                input
            )
        );
    }
}
