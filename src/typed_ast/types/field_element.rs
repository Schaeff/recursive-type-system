use flat_ast::*;
use std::fmt;
use typed_ast::{Expression, Variable};

#[derive(Debug)]
pub enum FieldElement {
    Identifier(Variable),
    Value(usize),
    Add(Box<FieldElement>, Box<FieldElement>),
    FunctionCall(String, Vec<Box<Expression>>),
}

// implement flattening for each type
impl Expression for FieldElement {
    fn flatten(&self, flatten_statements: &mut Vec<FlatStatement>) -> Vec<LinComb> {
        match *self {
            FieldElement::Identifier(ref v) => vec![LinComb(vec![(
                1,
                FlatVariable::with_name(format!("{}_0", v.name())),
            )])],
            FieldElement::Value(ref v) => vec![LinComb(vec![(*v, FlatVariable::one())])],
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldElement::Identifier(ref id) => write!(f, "{:?}", id),
            FieldElement::Value(ref v) => write!(f, "{}", v),
            FieldElement::Add(ref f1, ref f2) => write!(f, "{} + {}", f1, f2),
            FieldElement::FunctionCall(ref id, ref args) => write!(
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
