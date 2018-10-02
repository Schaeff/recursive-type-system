use std::fmt;
use types::Variant;
use FlatStatement;
use Flatten;
use LinComb;
use Variable;

#[derive(Debug)]
pub enum FieldElementVariant {
    Identifier(Variable),
    Value(usize),
    Add(Box<FieldElementVariant>, Box<FieldElementVariant>),
    FunctionCall(String, Vec<Box<Variant>>),
}

impl Variant for FieldElementVariant {
    fn get_primitive_count(&self) -> usize {
        1
    }
}

// implement flattening for each type
impl Flatten for FieldElementVariant {
    fn flatten(&self, flatten_statements: &mut Vec<FlatStatement>) -> Vec<LinComb> {
        match *self {
            FieldElementVariant::Identifier(ref v) => vec![LinComb(vec![(1, v.clone())])],
            FieldElementVariant::Value(ref v) => vec![LinComb(vec![(
                *v,
                Variable {
                    name: "~one".to_string(),
                },
            )])],
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for FieldElementVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldElementVariant::Identifier(ref id) => write!(f, "{:?}", id),
            FieldElementVariant::Value(ref v) => write!(f, "{}", v),
            FieldElementVariant::Add(ref f1, ref f2) => write!(f, "{} + {}", f1, f2),
            FieldElementVariant::FunctionCall(ref id, ref args) => write!(
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
