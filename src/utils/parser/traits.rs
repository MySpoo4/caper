use crate::utils::ParseQueue;

use super::error::PResult;

#[macro_export]
macro_rules! tup {
    // Base case: single element remains unchanged
    ($single:expr) => {
        (Some($single),)
    };
    // Recursive case: nest the first element with the rest
    ($first:expr, $($rest:tt)*) => {
        (Some($first), tup!($($rest)*))
    };
}

pub trait Parser {
    type Output;
    fn parse(&mut self, input: &mut ParseQueue) -> PResult<Self::Output>;
}

impl<F, O> Parser for F
where
    F: FnMut(&mut ParseQueue) -> PResult<O>,
{
    type Output = O;
    fn parse(&mut self, input: &mut ParseQueue) -> PResult<Self::Output> {
        input.save();
        let out = self(input);
        match out {
            Ok(_) => input.remove_save(),
            Err(_) => input.revert(),
        }
        out
    }
}

pub trait TupleParser {
    type Output;

    fn parse(&mut self, input: &mut ParseQueue) -> PResult<Self::Output>;
}

impl<F> TupleParser for (Option<F>,)
where
    F: Parser,
{
    type Output = F::Output;

    fn parse(&mut self, input: &mut ParseQueue) -> PResult<Self::Output> {
        (self.0.take().unwrap()).parse(input)
    }
}

impl<F, Tail> TupleParser for (Option<F>, Tail)
where
    F: Parser,
    Tail: TupleParser,
{
    type Output = (F::Output, Tail::Output);

    fn parse(&mut self, input: &mut ParseQueue) -> PResult<Self::Output> {
        let head_result = (self.0.take().unwrap()).parse(input)?;
        let tail_result = self.1.parse(input)?;
        Ok((head_result, tail_result))
    }
}

pub trait AltParser {
    type Output;
    fn parse(&mut self, input: &mut ParseQueue) -> PResult<Self::Output>;
}

impl<F> AltParser for (Option<F>,)
where
    F: Parser,
{
    type Output = F::Output;

    fn parse(&mut self, input: &mut ParseQueue) -> PResult<Self::Output> {
        (self.0.take().unwrap()).parse(input)
    }
}

impl<F, Tail> AltParser for (Option<F>, Tail)
where
    F: Parser,
    Tail: AltParser<Output = F::Output>,
{
    type Output = F::Output;

    fn parse(&mut self, input: &mut ParseQueue) -> PResult<Self::Output> {
        match (self.0.take().unwrap()).parse(input) {
            Ok(result) => Ok(result),
            Err(_) => self.1.parse(input),
        }
    }
}
