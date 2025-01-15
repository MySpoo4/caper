use crate::{
    dom::{
        node::{Attribute, AttributeValue},
        parser::interface::{Tag, TagKind, Token},
    },
    tup,
    utils::{
        parser::{
            alpha0, alpha1, alt, char, delimited, error::PError, many0, map, preceded, tag,
            tag_no_case, take_till, take_while1, traits::Parser, trimmed, tuple,
        },
        ParseQueue, SharedPool,
    },
};

pub fn parse_token() -> impl Parser<Output = Token> {
    alt(tup!(
        map(parse_tag(), |tag| Token::Tag(tag)),
        map(parse_doctype(), |doctype| Token::Doctype(doctype)),
        map(parse_comment(), |comment| Token::Comment(comment)),
        map(parse_text(), |text| Token::Text(text))
    ))
}

pub fn parse_special<'a>(special: &'a str) -> impl Parser<Output = Token> + 'a {
    alt(tup!(
        map(parse_special_end(special), |tag| Token::Tag(tag)),
        map(parse_text(), |text| Token::Text(text))
    ))
}

fn parse_tag() -> impl Parser<Output = Tag> {
    alt(tup!(parse_start_empty(), parse_end()))
}

pub fn parse_text() -> impl Parser<Output = String> {
    move |input: &mut ParseQueue| {
        let mut str = String::new();
        if let Ok(c) = char('<').parse(input) {
            str.push(c)
        }

        while let Some(c) = input.peek() {
            match c != '<' {
                true => str.push(input.dequeue().unwrap()),
                false => break,
            }
        }

        match !str.is_empty() {
            true => Ok(str),
            false => match input.peek() {
                Some(c) => Err(PError::InvalidChar(c)),
                None => Err(PError::EndOfInput),
            },
        }
    }
}

fn parse_doctype() -> impl Parser<Output = String> {
    delimited(tag_no_case("<!DOCTYPE"), trimmed(alpha1), char('>'))
}

fn parse_comment() -> impl Parser<Output = String> {
    delimited(tag("<!--"), trimmed(alpha0), tag("-->"))
}

// Parsed together due to similarity in early structure
pub fn parse_start_empty() -> impl Parser<Output = Tag> {
    map(
        preceded(
            char('<'),
            trimmed(tuple(tup!(
                map(alpha1, |name| { SharedPool::get_or_intern(name) }),
                parse_attrs(),
                alt(tup!(
                    map(char('>'), |_| TagKind::StartTag),
                    map(tag("/>"), |_| TagKind::EmptyTag)
                ))
            ))),
        ),
        |(name, (attrs, kind))| Tag { kind, name, attrs },
    )
}

fn parse_end() -> impl Parser<Output = Tag> {
    map(
        delimited(
            tag("</"),
            trimmed(map(alpha1, |name| SharedPool::get_or_intern(name))),
            char('>'),
        ),
        |name| Tag {
            kind: TagKind::EndTag,
            name,
            attrs: Vec::new(),
        },
    )
}

fn parse_special_end<'a>(special: &'a str) -> impl Parser<Output = Tag> + 'a {
    map(
        delimited(
            tag("</"),
            map(tag(special), |name| SharedPool::get_or_intern(name)),
            char('>'),
        ),
        |name| Tag {
            kind: TagKind::EndTag,
            name,
            attrs: Vec::new(),
        },
    )
}

fn parse_attrs() -> impl Parser<Output = Vec<Attribute>> {
    many0(trimmed(parse_attr()))
}

fn parse_attr() -> impl Parser<Output = Attribute> {
    |input: &mut ParseQueue| {
        let name = map(
            take_while1(|c: char| matches!(c, 'a'..='z' | 'A'..='Z' | '-' | ':' )),
            |name| SharedPool::get_or_intern(name),
        )
        .parse(input)?;
        let eq = trimmed(char('=')).parse(input);
        let value = match eq.is_ok() {
            true => AttributeValue::Literal(parse_str().parse(input)?),
            false => AttributeValue::Exists,
        };
        Ok(Attribute { name, value })
    }
}

fn parse_str() -> impl Parser<Output = String> {
    move |input: &mut ParseQueue| {
        alt(tup!(char('\''), char('"'))).parse(input).and_then(|q| {
            let content = take_till(|c| c == q).parse(input)?;
            char(q).parse(input)?;
            Ok(content)
        })
    }
}
