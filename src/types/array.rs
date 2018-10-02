use std::fmt;
use types::Variant;
use FlatStatement;
use Flatten;
use LinComb;
use Variable;

#[derive(Debug)]
pub enum ArrayVariant<V: Variant> {
    Value(Vec<V>),
    Identifier(Variable),
    FunctionCall(String, Vec<Box<Variant>>),
}

impl<V: Variant> ArrayVariant<V> {
    pub fn value(elements: Vec<V>) -> ArrayVariant<V> {
        assert_eq!(2, elements.len());
        ArrayVariant::Value(elements)
    }
}

impl<V: Variant> Flatten for ArrayVariant<V> {
    fn flatten(&self, flatten_statements: &mut Vec<FlatStatement>) -> Vec<LinComb> {
        match *self {
            ArrayVariant::Identifier(ref v) => vec![0, 1]
                .iter()
                .map(|n| {
                    LinComb(vec![(
                        1,
                        Variable {
                            name: format!("{}_{}", v.name, n),
                        },
                    )])
                }).collect(),
            ArrayVariant::Value(ref v) => v
                .iter()
                .map(|v| v.flatten(flatten_statements))
                .flat_map(|x| x)
                .collect(),
            _ => unimplemented!(),
        }
    }
}

impl<V: Variant> Variant for ArrayVariant<V> {
    fn get_primitive_count(&self) -> usize {
        2 * 42
    }
}

impl<V: Variant> fmt::Display for ArrayVariant<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ArrayVariant::Identifier(ref id) => write!(f, "{:?}", id),
            ArrayVariant::Value(ref values) => write!(
                f,
                "[{}]",
                values
                    .iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            ArrayVariant::FunctionCall(ref id, ref args) => write!(
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
