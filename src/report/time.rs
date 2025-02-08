use crate::{Annotated, parser::Parse};

#[derive(Debug, PartialEq, Eq)]
pub struct Time {
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

impl<'a> Parse<'a> for Annotated<'a, Time> {
    type Err = Annotated<'a, ()>;

    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
        if !s.ends_with('Z') {
            return Err(Annotated::new((), s));
        }

        if s.len() != 7 {
            return Err(Annotated::new((), s));
        }

        let day = s[0..2].parse().unwrap();
        if day < 1 || day > 31 {
            return Err(Annotated::new((), s));
        }

        let hour = s[2..4].parse().unwrap();
        if hour > 23 {
            return Err(Annotated::new((), s));
        }

        let minute = s[4..6].parse().unwrap();
        if minute > 59 {
            return Err(Annotated::new((), s));
        }

        Ok(Annotated::new(Time { day, hour, minute }, s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        let input = "061235Z";
        let time: Annotated<Time> = Parse::from_str(input).unwrap();
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
        let time: Result<Annotated<Time>, _> = Parse::from_str(input);
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
        let time: Result<Annotated<Time>, _> = Parse::from_str(input);
        assert!(time.is_err());
        assert_eq!(time.unwrap_err(), Annotated {
            inner: (),
            origin: input,
            start: 0,
            end: 6
        });
    }
}
