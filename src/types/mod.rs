// Define a trait to describe variants of a given type
use std::fmt::{Debug, Display};
use Flatten;

mod array;
mod boolean;
mod field_element;

pub use self::array::*;
pub use self::boolean::*;
pub use self::field_element::*;

// Define a trait to describe a type, embedding the variants as an associated type
pub trait Variant: Debug + Display + Flatten {
    fn get_primitive_count(&self) -> usize;
}
