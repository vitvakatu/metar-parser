use std::{
    fmt::Write,
    fmt::{self, Display},
    ops::Range,
    str::SplitWhitespace,
};

use frunk::hlist;
use snafu::ResultExt;

use crate::{
    Annotated,
    report::{self, Report},
};

pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn describe(&self, mut output: impl Write) -> Result<(), snafu::Whatever> {
        let mut context = Context::new(self.input);
        let parsers: Vec<(
            &'static str,
            Box<
                dyn Parse<
                        Output = Box<dyn AnnotatedDisplay<'a>>,
                        Err = Box<dyn AnnotatedDisplay<'a>>,
                    >,
            >,
        )> = vec![
            ("kind", EraseTypes::erase_types(report::kind::Parser)),
            ("station", EraseTypes::erase_types(report::station::Parser)),
            ("time", EraseTypes::erase_types(report::time::Parser)),
            ("wind", EraseTypes::erase_types(report::wind::Parser)),
            (
                "visibility",
                EraseTypes::erase_types(report::visibility::Parser),
            ),
            (
                "percipitation",
                EraseTypes::erase_types(report::percipitation::Parser),
            ),
            ("clouds", EraseTypes::erase_types(report::clouds::Parser)),
            (
                "temperature",
                EraseTypes::erase_types(report::temperature::Parser),
            ),
            (
                "pressure",
                EraseTypes::erase_types(report::pressure::Parser),
            ),
        ];
        let mut first = true;
        loop {
            for (_, parser) in parsers.iter() {
                let result = parser.from_str(&context);
                if let Ok(result) = result {
                    if !first {
                        output
                            .write_str(", ")
                            .with_whatever_context(|_| "Failed to write to output")?;
                    }
                    first = false;
                    output
                        .write_fmt(format_args!("{}", DisplayHelper(result)))
                        .with_whatever_context(|_| "Failed to write to output")?;
                }
            }
            if !context.advance() {
                break;
            }
        }
        Ok(())
    }

    pub fn parse(&self) -> Result<Report<'a>, Box<dyn AnnotatedDisplay<'a> + 'a>> {
        let mut context = Context::new(self.input);
        let parser_outputs = hlist![self.input];

        macro_rules! parse {
            ($parser:ident, $output:ident) => {
                let $parser = report::$parser::Parser
                    .from_str(&context)
                    .map_err(|e| Box::new(e) as Box<dyn AnnotatedDisplay<'a>>)?;
                let $output = $output + hlist![$parser];
                context.advance();
            };
        }

        parse!(kind, parser_outputs);
        parse!(station, parser_outputs);
        parse!(time, parser_outputs);
        parse!(wind, parser_outputs);
        parse!(visibility, parser_outputs);
        parse!(percipitation, parser_outputs);
        parse!(clouds, parser_outputs);
        parse!(temperature, parser_outputs);
        parse!(pressure, parser_outputs);

        let report: Report<'a> = frunk::from_generic(parser_outputs);
        Ok(report)
    }
}

pub(crate) struct Context<'a> {
    input: &'a str,
    parts: SplitWhitespace<'a>,
    current: Range<usize>,
    is_started: bool,
}

impl<'a> Context<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut context = Self {
            input,
            parts: input.split_whitespace(),
            current: 0..0,
            is_started: false,
        };
        context.advance();
        context
    }

    pub fn advance(&mut self) -> bool {
        let current_len = self.current.len();
        let space_width = if self.is_started { 1 } else { 0 };
        self.is_started = true;
        let Some(next_part) = self.parts.next() else {
            return false;
        };
        self.current = (self.current.start + current_len + space_width)
            ..(self.current.end + space_width + next_part.len());
        true
    }

    pub fn current(&self) -> &'a str {
        &self.input[self.current.clone()]
    }

    pub fn annotate<T>(&self, value: T) -> Annotated<'a, T> {
        Annotated::with_range(value, self.input, self.current.clone())
    }
}

pub(crate) trait Parse<'a> {
    type Output;
    type Err;
    fn from_str(&self, context: &Context<'a>) -> Result<Self::Output, Self::Err>;
}

pub(crate) trait EraseTypes<'a> {
    fn erase_types(
        self,
    ) -> Box<
        dyn Parse<
                'a,
                Output = Box<dyn AnnotatedDisplay<'a> + 'a>,
                Err = Box<dyn AnnotatedDisplay<'a> + 'a>,
            > + 'a,
    >;
}

impl<
    'a,
    P: Parse<'a, Output = T, Err = E> + 'a,
    T: AnnotatedDisplay<'a> + 'a,
    E: AnnotatedDisplay<'a> + 'a,
> EraseTypes<'a> for P
{
    fn erase_types(
        self,
    ) -> Box<
        dyn Parse<
                'a,
                Output = Box<dyn AnnotatedDisplay<'a> + 'a>,
                Err = Box<dyn AnnotatedDisplay<'a> + 'a>,
            > + 'a,
    > {
        Box::new(ErasedParser { inner: self })
    }
}

struct ErasedParser<P> {
    inner: P,
}
impl<
    'a,
    T: AnnotatedDisplay<'a> + 'a,
    E: AnnotatedDisplay<'a> + 'a,
    P: Parse<'a, Output = T, Err = E>,
> Parse<'a> for ErasedParser<P>
{
    type Output = Box<dyn AnnotatedDisplay<'a> + 'a>;
    type Err = Box<dyn AnnotatedDisplay<'a> + 'a>;
    fn from_str(&self, context: &Context<'a>) -> Result<Self::Output, Self::Err> {
        match self.inner.from_str(context) {
            Ok(output) => Ok(Box::new(output)),
            Err(err) => Err(Box::new(err)),
        }
    }
}

pub trait AnnotatedDisplay<'a>: std::fmt::Debug {
    fn display(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result;
    fn annotation(&self) -> Annotated<'a, ()>;
}

impl<'a, T: Display + std::fmt::Debug> AnnotatedDisplay<'a> for Annotated<'a, T> {
    fn display(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.inner, fmt)
    }

    fn annotation(&self) -> Annotated<'a, ()> {
        Annotated::unit(self)
    }
}

impl<'a, T: AnnotatedDisplay<'a> + ?Sized> AnnotatedDisplay<'a> for Box<T> {
    fn display(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        AnnotatedDisplay::display(&**self, fmt)
    }

    fn annotation(&self) -> Annotated<'a, ()> {
        AnnotatedDisplay::annotation(&**self)
    }
}

pub struct DisplayHelper<T>(T);

impl<'a, T: AnnotatedDisplay<'a>> Display for DisplayHelper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.display(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let mut context = Context::new("Hello world");
        assert_eq!(context.current(), "Hello");
        assert!(context.advance());
        assert_eq!(context.current(), "world");
        assert!(!context.advance());
    }
}
