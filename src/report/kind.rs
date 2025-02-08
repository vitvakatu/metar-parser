use crate::{Annotated, parser::Parse};

#[derive(Debug, PartialEq, Eq)]
pub enum ReportKind {
    Metar,
    Speci,
}

impl<'a> Parse<'a> for Annotated<'a, ReportKind> {
    type Err = Annotated<'a, ()>;

    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
        match s {
            "METAR" => Ok(Annotated::new(ReportKind::Metar, s)),
            "SPECI" => Ok(Annotated::new(ReportKind::Speci, s)),
            _ => Err(Annotated::new((), s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_kind_metar() {
        let input = "METAR";
        let kind: Annotated<ReportKind> = Parse::from_str(input).unwrap();
        assert_eq!(kind, Annotated {
            inner: ReportKind::Metar,
            origin: input,
            start: 0,
            end: 5
        });
    }

    #[test]
    fn parse_kind_speci() {
        let input = "SPECI";
        let kind: Annotated<ReportKind> = Parse::from_str(input).unwrap();
        assert_eq!(kind, Annotated {
            inner: ReportKind::Speci,
            origin: input,
            start: 0,
            end: 5
        });
    }

    #[test]
    fn parse_kind_invalid() {
        let input = "INVALID";
        let kind: Result<Annotated<ReportKind>, _> = Parse::from_str(input);
        assert!(kind.is_err());
        assert_eq!(kind.unwrap_err(), Annotated {
            inner: (),
            origin: input,
            start: 0,
            end: 7
        });
    }
}
