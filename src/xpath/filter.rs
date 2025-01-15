use std::{iter::Rev, slice::Iter};

use crate::dom::{node::AttributeValue, DomNode};

use super::{
    parser::interface::{Axis, Condition, LogicalOperator, Predicate, SpType, XPathStep},
    XPath,
};

#[derive(Debug)]
pub enum AxisIterator<'a> {
    Child(ChildIterator<'a>),
    Descendant(DescendantIterator<'a>),
    RevChild(RevChildIterator<'a>),
    RevDescendant(RevDescendantIterator<'a>),
}

impl<'a> AxisIterator<'a> {
    pub fn new(axis: &Axis, start: &bool) -> Self {
        match start {
            true => match axis {
                Axis::Child => Self::Child(ChildIterator::new()),
                Axis::Descendant => Self::Descendant(DescendantIterator::new()),
            },
            false => match axis {
                Axis::Child => Self::RevChild(RevChildIterator::new()),
                Axis::Descendant => Self::RevDescendant(RevDescendantIterator::new()),
            },
        }
    }

    pub fn add(&mut self, node: &'a DomNode) {
        match self {
            Self::Child(child) => child.add(node),
            Self::Descendant(descendant) => descendant.add(node),
            Self::RevChild(child) => child.add(node),
            Self::RevDescendant(descendant) => descendant.add(node),
        }
    }
}

impl<'a> Iterator for AxisIterator<'a> {
    type Item = &'a DomNode;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            AxisIterator::Child(iter) => iter.next(),
            AxisIterator::Descendant(iter) => iter.next(),
            AxisIterator::RevChild(iter) => iter.next(),
            AxisIterator::RevDescendant(iter) => iter.next(),
        }
    }
}

#[derive(Debug)]
pub struct ChildIterator<'a> {
    children: std::slice::Iter<'a, DomNode>,
}

impl<'a> ChildIterator<'a> {
    pub fn new() -> Self {
        Self {
            children: std::slice::Iter::default(),
        }
    }

    pub fn add(&mut self, node: &'a DomNode) {
        self.children = node.children.iter();
    }
}

impl<'a> Iterator for ChildIterator<'a> {
    type Item = &'a DomNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.children.next()
    }
}

#[derive(Debug)]
pub struct DescendantIterator<'a> {
    stack: Vec<&'a DomNode>,
}

impl<'a> DescendantIterator<'a> {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn add(&mut self, node: &'a DomNode) {
        self.stack.push(node);
    }
}

impl<'a> Iterator for DescendantIterator<'a> {
    type Item = &'a DomNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            self.stack.extend(node.children.iter().rev());
            Some(node)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct RevChildIterator<'a> {
    children: Rev<Iter<'a, DomNode>>,
}

impl<'a> RevChildIterator<'a> {
    pub fn new() -> Self {
        Self {
            children: Rev::default(),
        }
    }

    pub fn add(&mut self, node: &'a DomNode) {
        self.children = node.children.iter().rev();
    }
}

impl<'a> Iterator for RevChildIterator<'a> {
    type Item = &'a DomNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.children.next()
    }
}

#[derive(Debug)]
pub struct RevDescendantIterator<'a> {
    stack: Vec<(&'a DomNode, bool)>,
}

impl<'a> RevDescendantIterator<'a> {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn add(&mut self, node: &'a DomNode) {
        self.stack.push((node, false));
    }
}

impl<'a> Iterator for RevDescendantIterator<'a> {
    type Item = &'a DomNode;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, processed)) = self.stack.pop() {
            if processed {
                return Some(node);
            } else {
                self.stack.push((node, true));

                for child in node.children.iter() {
                    self.stack.push((child, false));
                }
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct XPathFilter<'a> {
    chain: Vec<(AxisIterator<'a>, usize)>,
    steps: Vec<&'a XPathStep>,
}

impl<'a> XPathFilter<'a> {
    pub fn new_with_node(xpath: &'a XPath, node: &'a DomNode) -> Self {
        let mut filter = Self::new(xpath);
        filter.add_node(node);
        filter
    }

    pub fn new(xpath: &'a XPath) -> Self {
        let steps = xpath.steps.iter().collect::<Vec<_>>();
        let chain = steps
            .iter()
            .fold(Vec::with_capacity(steps.len()), |mut acc, step| {
                acc.push((
                    AxisIterator::new(&step.axis, &step.pos.as_ref().map_or(true, |p| p.start)),
                    0,
                ));
                acc
            });

        Self { chain, steps }
    }

    pub fn add_node(&mut self, node: &'a DomNode) {
        self.chain[0].0.add(node);
    }

    // get or resolves the iterator at pos
    pub fn get_resolve(&mut self, pos: usize) -> Option<&'a DomNode> {
        {
            let step = self.steps.get(pos).unwrap();
            let (iter, node_pos) = self.chain.get_mut(pos).unwrap();
            while let Some(node) = iter.next() {
                if node.tag.as_ref() == step.tag_name
                    && step.predicates.iter().all(|p| p.evaluate(node))
                {
                    *node_pos += 1;
                    if step.pos.as_ref().map_or(true, |pos| pos.pos == *node_pos) {
                        return Some(node);
                    }
                }
            }
        }

        if pos > 0 {
            if let Some(node) = self.get_resolve(pos - 1) {
                self.chain.get_mut(pos).unwrap().0.add(node);
                return self.get_resolve(pos);
            }
        }

        None
    }
}

impl<'a> Iterator for XPathFilter<'a> {
    type Item = &'a DomNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_resolve(self.chain.len() - 1)
    }
}

impl Predicate {
    pub fn evaluate(&self, node: &DomNode) -> bool {
        match self {
            Predicate::Expression(cond) => cond.evaluate(node),
            Predicate::Logical { op, left, right } => match op {
                LogicalOperator::And => left.evaluate(node) && right.evaluate(node),
                LogicalOperator::Or => left.evaluate(node) || right.evaluate(node),
            },
        }
    }
}

impl Condition {
    pub fn evaluate(&self, node: &DomNode) -> bool {
        match self {
            Condition::AttrExists(attr) => node.attributes.iter().any(|a| a.name.as_ref() == *attr),
            Condition::AttrCond { attr, sp_type, val } => node.attributes.iter().any(|a| {
                a.name.as_ref() == attr
                    && match a.value {
                        AttributeValue::Literal(ref v) => sp_equal(v, sp_type, val),
                        _ => false,
                    }
            }),
            Condition::TextCond { sp_type, val } => {
                sp_equal(&node.text_content.as_str(), sp_type, val)
            }
        }
    }
}

fn sp_equal(left: &str, sp: &SpType, right: &str) -> bool {
    match sp {
        SpType::Base => left == right,
        SpType::Contains => left.contains(&right),
        SpType::Starts => left.starts_with(&right),
        SpType::Ends => left.ends_with(&right),
    }
}
