use std::fmt;
use types::Variant;
use FlatStatement;
use Flatten;
use LinComb;
use Variable;

#[derive(Debug)]
pub enum BooleanVariant {
    Identifier(Variable),
    Value(bool),
    And(Box<BooleanVariant>, Box<BooleanVariant>),
    FunctionCall(String, Vec<Box<Variant>>),
}

impl Variant for BooleanVariant {
    fn get_primitive_count(&self) -> usize {
        1
    }
}

impl Flatten for BooleanVariant {
    fn flatten(&self, flatten_statements: &mut Vec<FlatStatement>) -> Vec<LinComb> {
        match *self {
            BooleanVariant::Value(b) => {
                if { b } {
                    vec![LinComb(vec![(
                        1,
                        Variable {
                            name: String::from("~one"),
                        },
                    )])]
                } else {
                    vec![LinComb(vec![(
                        0,
                        Variable {
                            name: String::from("~one"),
                        },
                    )])]
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for BooleanVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BooleanVariant::Identifier(ref id) => write!(f, "{:?}", id),
            BooleanVariant::Value(ref v) => write!(f, "{}", v),
            BooleanVariant::And(ref b1, ref b2) => write!(f, "{} ^ {}", b1, b2),
            BooleanVariant::FunctionCall(ref id, ref args) => write!(
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

impl BooleanVariant {
    pub fn _true() -> Self {
        BooleanVariant::Value(true)
    }

    pub fn _false() -> Self {
        BooleanVariant::Value(false)
    }
}
