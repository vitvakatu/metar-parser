use std::fmt::{self, Display};

use crate::{
    Annotated,
    parser::{Context, Parse},
};
use snafu::Snafu;

#[derive(Debug, Snafu, PartialEq)]
#[snafu(display("Invalid report kind, expected METAR/SPECI"))]
pub struct InvalidReportKind;

pub struct Parser;

#[derive(Debug, PartialEq, Eq)]
pub enum ReportKind {
    Metar,
    Speci,
}

impl Display for ReportKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReportKind::Metar => write!(f, "METAR"),
            ReportKind::Speci => write!(f, "SPECI"),
        }
    }
}

impl<'a> Parse<'a> for Parser {
    type Output = Annotated<'a, ReportKind>;
    type Err = Annotated<'a, InvalidReportKind>;

    fn from_str(&self, context: &Context<'a>) -> Result<Self::Output, Self::Err> {
        match context.current() {
            "METAR" => Ok(context.annotate(ReportKind::Metar)),
            "SPECI" => Ok(context.annotate(ReportKind::Speci)),
            _ => Err(context.annotate(InvalidReportKind)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_kind_metar() {
        let input = "METAR";
        let parser = Parser;
        let context = Context::new(input);
        let kind: Annotated<ReportKind> = parser.from_str(&context).unwrap();
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
        let parser = Parser;
        let context = Context::new(input);
        let kind: Annotated<ReportKind> = parser.from_str(&context).unwrap();
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
        let parser = Parser;
        let context = Context::new(input);
        let kind: Result<Annotated<ReportKind>, _> = parser.from_str(&context);
        assert!(kind.is_err());
        assert_eq!(kind.unwrap_err(), Annotated {
            inner: InvalidReportKind,
            origin: input,
            start: 0,
            end: 7
        });
    }
}
