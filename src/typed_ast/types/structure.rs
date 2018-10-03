use flat_ast::*;
use std::fmt;
use typed_ast::{Expression, Variable};

#[derive(Debug)]
pub enum Structure {
    Value(Vec<Box<Expression>>),
    Identifier(Variable),
    FunctionCall(String, Vec<Box<Expression>>),
}

impl Structure {
    pub fn value(elements: Vec<Box<Expression>>) -> Structure {
        assert_eq!(2, elements.len());
        Structure::Value(elements)
    }
}

impl Expression for Structure {
    fn flatten(&self, flatten_statements: &mut Vec<FlatStatement>) -> Vec<LinComb> {
        match *self {
            Structure::Identifier(ref v) => vec![0, 1]
                .iter()
                .map(|n| {
                    LinComb(vec![(
                        1,
                        FlatVariable::with_name(format!("{}_{}", v.name(), n)),
                    )])
                }).collect(),
            Structure::Value(ref v) => v
                .iter()
                .map(|v| v.flatten(flatten_statements))
                .flat_map(|x| x)
                .collect(),
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Structure::Identifier(ref id) => write!(f, "{:?}", id),
            Structure::Value(ref values) => write!(
                f,
                "[{}]",
                values
                    .iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Structure::FunctionCall(ref id, ref args) => write!(
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
