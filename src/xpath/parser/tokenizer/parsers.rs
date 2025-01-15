use crate::{
    tup,
    utils::{
        parser::{
            alpha1, alt, char, delimited, digit, error::PError, many0, map, opt, preceded, tag,
            take_till, traits::Parser, trimmed, tuple,
        },
        ParseQueue,
    },
    xpath::parser::interface::{
        Axis, Condition, LogicalOperator, Position, Predicate, SpType, XPathStep,
    },
};

pub fn parse_xpath_step() -> impl Parser<Output = XPathStep> {
    map(
        tuple(tup!(
            parse_axis(),
            parse_tag_name(),
            parse_predicates(),
            opt(parse_position())
        )),
        |(axis, (tag_name, (predicates, pos)))| XPathStep {
            axis,
            tag_name,
            predicates,
            pos,
        },
    )
}

fn parse_axis() -> impl Parser<Output = Axis> {
    |input: &mut ParseQueue| {
        char('/').parse(input)?;
        match char('/').parse(input) {
            Ok(_) => Ok(Axis::Descendant),
            Err(_) => Ok(Axis::Child),
        }
    }
}

fn parse_tag_name() -> impl Parser<Output = String> {
    alpha1
}

fn parse_predicates() -> impl Parser<Output = Vec<Predicate>> {
    many0(delimited(char('['), trimmed(parse_logical()), char(']')))
}

fn parse_position() -> impl Parser<Output = Position> {
    preceded(
        preceded(tag(":nth"), trimmed(char('='))),
        map(
            tuple(tup!(opt(char('-')), digit)),
            |(sign, num): (Option<char>, usize)| {
                println!("{:#?}", sign);
                match sign.is_some() {
                    true => Position {
                        start: false,
                        pos: num,
                    },
                    false => Position {
                        start: true,
                        pos: num,
                    },
                }
            },
        ),
    )
}

fn parse_logical() -> impl Parser<Output = Predicate> {
    |input: &mut ParseQueue| {
        let mut left = alt(tup!(
            parse_condition(),
            delimited(char('('), parse_logical(), char(')'))
        ))
        .parse(input)?;

        while let Ok(op) = trimmed(parse_logical_op()).parse(input) {
            let right = alt(tup!(
                parse_condition(),
                delimited(char('('), parse_logical(), char(')'))
            ))
            .parse(input)?;

            left = Predicate::Logical {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }
}

fn parse_logical_op() -> impl Parser<Output = LogicalOperator> {
    alt(tup!(
        map(char('&'), |_| LogicalOperator::And),
        map(char('|'), |_| LogicalOperator::Or)
    ))
}

fn parse_condition() -> impl Parser<Output = Predicate> {
    map(alt(tup!(parse_attr(), parse_text())), |cond| {
        Predicate::Expression(cond)
    })
}

fn parse_attr() -> impl Parser<Output = Condition> {
    |input: &mut ParseQueue| {
        let attr = preceded(char('@'), alpha1).parse(input)?;
        let sp_type = parse_sp().parse(input)?;
        let eq = trimmed(char('=')).parse(input);
        match (&sp_type, &eq) {
            (SpType::Base, Err(_)) => Ok(Condition::AttrExists(attr)),
            (_, Ok(_)) => Ok(Condition::AttrCond {
                attr,
                sp_type,
                val: parse_str().parse(input)?,
            }),
            _ => match input.peek() {
                Some(c) => Err(PError::InvalidChar(c)),
                _ => Err(PError::EndOfInput),
            },
        }
    }
}

fn parse_text() -> impl Parser<Output = Condition> {
    |input: &mut ParseQueue| {
        tag("text").parse(input)?;
        let sp_type = parse_sp().parse(input)?;
        let val = preceded(trimmed(char('=')), parse_str()).parse(input)?;
        Ok(Condition::TextCond { sp_type, val })
    }
}

fn parse_sp() -> impl Parser<Output = SpType> {
    |input: &mut ParseQueue| {
        Ok(alt(tup!(
            map(char('*'), |_| SpType::Contains),
            map(char('^'), |_| SpType::Starts),
            map(char('$'), |_| SpType::Ends)
        ))
        .parse(input)
        .unwrap_or(SpType::Base))
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
