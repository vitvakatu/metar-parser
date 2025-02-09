use crate::{
    Annotated,
    parser::{Context, Parse},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Time {
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

impl<'a> Parse<'a> for Annotated<'a, Time> {
    type Err = Annotated<'a, ()>;

    fn from_str(context: &Context<'a>) -> Result<Self, Self::Err> {
        if !context.current().ends_with('Z') {
            return Err(context.annotate(()));
        }

        if context.current().len() != 7 {
            return Err(context.annotate(()));
        }

        let day = context.current()[0..2].parse().unwrap();
        if day < 1 || day > 31 {
            return Err(context.annotate(()));
        }

        let hour = context.current()[2..4].parse().unwrap();
        if hour > 23 {
            return Err(context.annotate(()));
        }

        let minute = context.current()[4..6].parse().unwrap();
        if minute > 59 {
            return Err(context.annotate(()));
        }

        Ok(context.annotate(Time { day, hour, minute }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        let input = "061235Z";
        let context = Context::new(input);
        let time: Annotated<Time> = Parse::from_str(&context).unwrap();
        assert_eq!(time, Annotated {
            inner: Time {
                day: 6,
                hour: 12,
                minute: 35
            },
            origin: input,
            start: 0,
            end: 7
        });
    }

    #[test]
    fn test_time_invalid_day() {
        let input = "321235Z";
        let context = Context::new(input);
        let time: Result<Annotated<Time>, _> = Parse::from_str(&context);
        assert!(time.is_err());
        assert_eq!(time.unwrap_err(), Annotated {
            inner: (),
            origin: input,
            start: 0,
            end: 7
        });
    }
    #[test]
    fn test_time_missing_zulu() {
        let input = "061235";
        let context = Context::new(input);
        let time: Result<Annotated<Time>, _> = Parse::from_str(&context);
        assert!(time.is_err());
        assert_eq!(time.unwrap_err(), Annotated {
            inner: (),
            origin: input,
            start: 0,
            end: 6
        });
    }
}
