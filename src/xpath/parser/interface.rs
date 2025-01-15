#[derive(Debug)]
pub struct XPathStep {
    pub axis: Axis,
    pub tag_name: String,
    pub predicates: Vec<Predicate>,
    pub pos: Option<Position>,
}

#[derive(Debug)]
pub enum Axis {
    Child,
    Descendant,
}

#[derive(Debug)]
pub enum SpType {
    Base,
    Contains,
    Starts,
    Ends,
}

#[derive(Debug)]
pub enum Condition {
    AttrExists(String),
    AttrCond {
        attr: String,
        sp_type: SpType,
        val: String,
    },
    TextCond {
        sp_type: SpType,
        val: String,
    },
}

#[derive(Debug)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug)]
pub enum Predicate {
    Expression(Condition),
    Logical {
        op: LogicalOperator,
        left: Box<Predicate>,
        right: Box<Predicate>,
    },
}

#[derive(Debug)]
pub struct Position {
    pub start: bool,
    pub pos: usize,
}

#[derive(Debug)]
pub enum Token {
    XPathStep(XPathStep),

    // Errors
    InvalidChar(char),
    EndOfInput,
}

pub enum TokenSinkResult {
    Continue,
    Suspend,
}
