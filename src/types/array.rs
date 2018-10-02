use types::Type;
use types::Variant;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Array<T: Type>(pub Box<T>);

// Helper function to create an Array type
impl<T: Type> Array<T> {
    pub fn with(t: T) -> Self {
        Array(Box::new(t))
    }
}

impl<T: Type> Type for Array<T> {
    type Variant = ArrayVariant<T>;
    fn get_primitive_count(&self) -> usize {
        2 * self.0.get_primitive_count()
    }
}

#[derive(Debug, Clone)]
pub enum ArrayVariant<T: Type> {
    Value(Vec<T::Variant>),
    Identifier(String),
}

impl<T: Type> ArrayVariant<T> {
	pub fn value(elements: Vec<T::Variant>) -> ArrayVariant<T> {
		println!("SIZE {:?} ELEMENTS {:?}", 3, elements);
		assert_eq!(2, elements.len());
		ArrayVariant::Value(elements)
	}
}

impl<T: Type> Variant<Array<T>> for ArrayVariant<T> {}

impl<T: Type> fmt::Display for ArrayVariant<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ArrayVariant::Identifier(ref id) => write!(f, "{}", id),
            ArrayVariant::Value(ref values) => write!(
                f,
                "[{}]",
                values
                    .iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}