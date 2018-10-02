// Define a trait to describe variants of a given type
use std::fmt::{Debug, Display};

mod field_element;
mod boolean;
mod array;

pub use self::field_element::*;
pub use self::boolean::*;
pub use self::array::*;

pub trait Variant<T: Type>: Debug + Display {}

// Define a trait to describe a type, embedding the variants as an associated type
pub trait Type: Sized + Debug {
    type Variant: Variant<Self>;
    fn get_primitive_count(&self) -> usize;
}