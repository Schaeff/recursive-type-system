use types::Type;
use types::Variant;
use std::fmt;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Boolean();

impl Type for Boolean {
    type Variant = BooleanVariant;
    fn get_primitive_count(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone)]
pub enum BooleanVariant {
    Identifier(String),
    Value(bool),
    And(Box<BooleanVariant>, Box<BooleanVariant>),
}

impl Variant<Boolean> for BooleanVariant {}

impl fmt::Display for BooleanVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BooleanVariant::Identifier(ref id) => write!(f, "{}", id),
            BooleanVariant::Value(ref v) => write!(f, "{}", v),
            BooleanVariant::And(ref b1, ref b2) => write!(f, "{} ^ {}", b1, b2),
        }
    }
}