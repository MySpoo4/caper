use crate::utils::ParseQueue;

use super::{
    error::PError,
    traits::{AltParser, Parser, TupleParser},
    whitespace0,
};

pub fn opt<F, O>(mut parser: F) -> impl Parser<Output = Option<O>>
where
    F: Parser<Output = O>,
{
    move |input: &mut ParseQueue| Ok(parser.parse(input).ok())
}

pub fn many1<F, O>(parser: F) -> impl Parser<Output = Vec<O>>
where
    F: Parser<Output = O>,
{
    let mut many = many0(parser);
    move |input: &mut ParseQueue| {
        let out = many.parse(input)?;
        match out.len() {
            0 => match input.peek() {
                Some(c) => Err(PError::InvalidChar(c)),
                None => Err(PError::EndOfInput),
            },
            _ => Ok(out),
        }
    }
}

pub fn many0<F, O>(mut parser: F) -> impl Parser<Output = Vec<O>>
where
    F: Parser<Output = O>,
{
    move |input: &mut ParseQueue| {
        let mut vec = Vec::new();
        while let Ok(out) = parser.parse(input) {
            vec.push(out);
        }
        Ok(vec)
    }
}

pub fn tuple<P>(mut parsers: P) -> impl Parser<Output = P::Output>
where
    P: TupleParser,
{
    move |input: &mut ParseQueue| parsers.parse(input)
}

pub fn alt<A, Out>(mut parsers: A) -> impl Parser<Output = Out>
where
    A: AltParser<Output = Out>,
{
    move |input: &mut ParseQueue| parsers.parse(input)
}

pub fn take_while<F>(cond: F) -> impl Parser<Output = String>
where
    F: Fn(char) -> bool,
{
    move |input: &mut ParseQueue| Ok(input.consume_while(|c| cond(c)))
}

pub fn take_while1<F>(cond: F) -> impl Parser<Output = String>
where
    F: Fn(char) -> bool,
{
    move |input: &mut ParseQueue| {
        let out = input.consume_while(|c| cond(c));
        match out.len() {
            0 => match input.peek() {
                Some(c) => Err(PError::InvalidChar(c)),
                None => Err(PError::EndOfInput),
            },
            _ => Ok(out),
        }
    }
}

pub fn take_till<F>(cond: F) -> impl Parser<Output = String>
where
    F: Fn(char) -> bool,
{
    move |input: &mut ParseQueue| Ok(input.consume_till(|c| cond(c)))
}

pub fn take_till1<F>(cond: F) -> impl Parser<Output = String>
where
    F: Fn(char) -> bool,
{
    move |input: &mut ParseQueue| {
        let out = input.consume_till(|c| cond(c));
        match out.len() {
            0 => match input.peek() {
                Some(c) => Err(PError::InvalidChar(c)),
                None => Err(PError::EndOfInput),
            },
            _ => Ok(out),
        }
    }
}

pub fn trimmed<F, O>(mut parser: F) -> impl Parser<Output = O>
where
    F: Parser<Output = O>,
{
    move |input: &mut ParseQueue| {
        delimited(
            whitespace0,
            |input: &mut ParseQueue| parser.parse(input),
            whitespace0,
        )
        .parse(input)
    }
}

pub fn preceded<F1, F2, O>(mut p1: F1, mut p2: F2) -> impl Parser<Output = O>
where
    F1: Parser,
    F2: Parser<Output = O>,
{
    move |input: &mut ParseQueue| {
        p1.parse(input)?;
        p2.parse(input)
    }
}

pub fn terminated<F1, O, F2>(mut p1: F1, mut p2: F2) -> impl Parser<Output = O>
where
    F1: Parser<Output = O>,
    F2: Parser,
{
    move |input: &mut ParseQueue| {
        let out = p1.parse(input)?;
        p2.parse(input)?;
        Ok(out)
    }
}

pub fn delimited<F1, F2, O, F3>(mut p1: F1, mut p2: F2, mut p3: F3) -> impl Parser<Output = O>
where
    F1: Parser,
    F2: Parser<Output = O>,
    F3: Parser,
{
    move |input: &mut ParseQueue| {
        p1.parse(input)?; // Call `parse` method for p1
        let o = p2.parse(input)?; // Call `parse` method for p2
        p3.parse(input).map(|_| o) // Call `parse` method for p3
    }
}

pub fn map<F, O1, M, O2>(mut parser: F, mapper: M) -> impl Parser<Output = O2>
where
    F: Parser<Output = O1>,
    M: Fn(F::Output) -> O2,
{
    move |input: &mut ParseQueue| Ok(mapper(parser.parse(input)?))
}

pub fn tag<'a>(str: &'a str) -> impl Parser<Output = &'a str> {
    move |input: &mut ParseQueue| {
        str.chars()
            .into_iter()
            .try_for_each(|c| char(c).parse(input).map(|_| ()))?;
        Ok(str)
    }
}

pub fn tag_no_case<'a>(str: &'a str) -> impl Parser<Output = &'a str> {
    move |input: &mut ParseQueue| {
        str.chars()
            .into_iter()
            .try_for_each(|c| char_no_case(c).parse(input).map(|_| ()))?;
        Ok(str)
    }
}

pub fn char(c: char) -> impl Parser<Output = char> {
    move |input: &mut ParseQueue| {
        let out = input.peek();
        match out == Some(c) {
            true => Ok(input.dequeue().unwrap()),
            false => match out {
                Some(c) => Err(PError::InvalidChar(c)),
                None => Err(PError::EndOfInput),
            },
        }
    }
}

pub fn char_no_case(c: char) -> impl Parser<Output = char> {
    move |input: &mut ParseQueue| {
        let c = c.to_lowercase().next().unwrap();
        let out = input.peek().map(|c| c.to_lowercase().next().unwrap());
        match out == Some(c) {
            true => Ok(input.dequeue().unwrap()),
            false => match out {
                Some(c) => Err(PError::InvalidChar(c)),
                None => Err(PError::EndOfInput),
            },
        }
    }
}
