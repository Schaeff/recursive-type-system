use flat_ast::*;
use std::fmt;
use typed_ast::{Expression, Variable};

#[derive(Debug)]
pub enum Array<V: Expression> {
    Value(Vec<V>),
    Identifier(Variable),
    FunctionCall(String, Vec<Box<Expression>>),
}

impl<V: Expression> Array<V> {
    pub fn value(elements: Vec<V>) -> Array<V> {
        assert_eq!(2, elements.len());
        Array::Value(elements)
    }
}

impl<V: Expression> Expression for Array<V> {
    fn flatten(&self, flatten_statements: &mut Vec<FlatStatement>) -> Vec<LinComb> {
        match *self {
            Array::Identifier(ref v) => vec![0, 1]
                .iter()
                .map(|n| {
                    LinComb(vec![(
                        1,
                        FlatVariable::with_name(format!("{}_{}", v.name(), n)),
                    )])
                }).collect(),
            Array::Value(ref v) => v
                .iter()
                .map(|v| v.flatten(flatten_statements))
                .flat_map(|x| x)
                .collect(),
            _ => unimplemented!(),
        }
    }
}

impl<V: Expression> fmt::Display for Array<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Array::Identifier(ref id) => write!(f, "{:?}", id),
            Array::Value(ref values) => write!(
                f,
                "[{}]",
                values
                    .iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Array::FunctionCall(ref id, ref args) => write!(
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
