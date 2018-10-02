use types::Type;
use types::Variant;
use std::fmt;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FieldElement();

impl Type for FieldElement {
    type Variant = FieldElementVariant;
    fn get_primitive_count(&self) -> usize {
        1
    }
}

#[derive(Clone, Debug)]
pub enum FieldElementVariant {
    Identifier(String),
    Value(usize),
    Add(Box<FieldElementVariant>, Box<FieldElementVariant>),
}

impl Variant<FieldElement> for FieldElementVariant {}

impl fmt::Display for FieldElementVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldElementVariant::Identifier(ref id) => write!(f, "{}", id),
            FieldElementVariant::Value(ref v) => write!(f, "{}", v),
            FieldElementVariant::Add(ref f1, ref f2) => write!(f, "{} + {}", f1, f2),
        }
    }
}
