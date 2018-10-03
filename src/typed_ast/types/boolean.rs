use flat_ast::*;
use std::fmt;
use typed_ast::{Expression, Variable};

#[derive(Debug)]
pub enum Boolean {
    Identifier(Variable),
    Value(bool),
    And(Box<Boolean>, Box<Boolean>),
    FunctionCall(String, Vec<Box<Expression>>),
}

impl Expression for Boolean {
    fn flatten(&self, flatten_statements: &mut Vec<FlatStatement>) -> Vec<LinComb> {
        match *self {
            Boolean::Value(b) => {
                if { b } {
                    vec![LinComb(vec![(1, FlatVariable::one())])]
                } else {
                    vec![LinComb(vec![(0, FlatVariable::one())])]
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Boolean::Identifier(ref id) => write!(f, "{:?}", id),
            Boolean::Value(ref v) => write!(f, "{}", v),
            Boolean::And(ref b1, ref b2) => write!(f, "{} ^ {}", b1, b2),
            Boolean::FunctionCall(ref id, ref args) => write!(
                f,
                "{}({})",
                id,
                args.iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl Boolean {
    pub fn _true() -> Self {
        Boolean::Value(true)
    }

    pub fn _false() -> Self {
        Boolean::Value(false)
    }
}
