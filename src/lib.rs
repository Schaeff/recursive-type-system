// FooType implements Type
// FooVariant implements Variant<FooType>
// FooExpression carries FooType and FooVariant

#![feature(box_patterns, box_syntax)]

mod types;

use types::*;

use std::default::Default;
use std::fmt;

///////////////////////////////////////////////////////////////////

// we assume that a FlatExpression is a number for now, which works for constants
#[derive(PartialEq, Debug)]
pub struct FlatExpression(usize);

pub trait Flatten: Sized {
    fn flatten(self) -> Vec<FlatExpression>;
}

#[derive(Debug, Clone)]
struct Expression<T: Type> {
    _type: T,
    variant: T::Variant,
}

impl<T: Type + Default> Expression<T> {
    // used for primitive types for which T::new() is not ambiguous
    fn new(v: T::Variant) -> Self {
        Expression::with(T::default(), v)
    }
}

impl<T: Type> Expression<T> {
    // used for complex types for which we need to specify more information
    fn with(t: T, v: T::Variant) -> Self {
        Expression {
            _type: t,
            variant: v,
        }
    }
}

impl Expression<FieldElement> {

    fn field_element(v: FieldElementVariant) -> Self {
        Self::new(v)
    }

    fn add(e1: FieldElementVariant, e2: FieldElementVariant) -> Self {
        Self::field_element(FieldElementVariant::Add(box e1, box e2))
    }
}

impl Expression<Boolean> {
    fn _true() -> Self {
        Expression::new(BooleanVariant::Value(true))
    }

    fn _false() -> Self {
        Expression::new(BooleanVariant::Value(false))
    }
}

// t: the type stored in the array
// v: the elements stored in the array
impl<T: Type> Expression<Array<T>> {
    fn array_with_type(t: T, v: Vec<T::Variant>) -> Self {
        Expression::with(Array::with(t), ArrayVariant::value(v))
    }
}

impl<T: Type + Default> Expression<Array<T>> {
    fn array(v: Vec<T::Variant>) -> Self {
        Self::array_with_type(T::default(), v)
    }
}

// implement flattening for each type
impl Flatten for Expression<FieldElement> {
    fn flatten(self) -> Vec<FlatExpression> {
        vec![FlatExpression(1)]
    }
}

impl Flatten for Expression<Boolean> {
    fn flatten(self) -> Vec<FlatExpression> {
        match self.variant {
            BooleanVariant::Value(b) => {
                if { b } {
                    vec![FlatExpression(1)]
                } else {
                    vec![FlatExpression(0)]
                }
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Variable<T: Type> {
    _type: T,
    name: String,
}

#[derive(Debug)]
enum Statement<T: Type> {
    Definition(Variable<T>, Expression<T>),
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn create_nested_array_type() {
	    let arr_of_arr = Array(Box::new(Array(Box::new(FieldElement()))));
	    assert_eq!(arr_of_arr.get_primitive_count(), 2 * 2);
	}

	#[test]
	fn array_expression() {
		let e0: Expression<Array<FieldElement>> = Expression::array(vec![
	        FieldElementVariant::Value(0),
	        FieldElementVariant::Value(1),
	    ]);
	    assert_eq!(e0._type, Array(box FieldElement()));
	}

	#[test]
	fn nested_array_expression() {
		let e1 = Expression::array_with_type(
	        Array(Box::new(FieldElement())),
	        vec![
	            ArrayVariant::value(vec![
	                FieldElementVariant::Value(0),
	                FieldElementVariant::Value(1),
	            ]),
	            ArrayVariant::value(vec![
	                FieldElementVariant::Value(0),
	                FieldElementVariant::Value(1),
	            ]),
	        ],
	    );
	    println!("{}", e1);
	    assert_eq!(e1._type, Array(box Array(box FieldElement())));
	}

	#[test]
	fn definition() {
	    let _s = Statement::Definition(
	        Variable {
	            _type: Boolean(),
	            name: String::from("a"),
	        },
	        Expression::_true(),
	    );
	}

	#[test]
	#[should_panic]
	fn wrong_count_in_def() {
	    let _e2 = Expression::array_with_type(
	        Array(Box::new(FieldElement())), // here 3 elements
	        vec![
	            ArrayVariant::value(vec![
	                FieldElementVariant::Value(0), // /!\ here 1 element /!\
	            ]),
	            ArrayVariant::value(vec![
	                FieldElementVariant::Value(0), // here 3 elements
	                FieldElementVariant::Value(1),
	            ]),
	        ],
	    ); // should panic
	}

	#[test]
	fn flatten_primitive_types() {
		// we can flatten expressions
	    let _42_plus_42 = Expression::add(FieldElementVariant::Value(42), FieldElementVariant::Value(42));
	    assert_eq!(_42_plus_42.flatten(), vec![FlatExpression(1)]);
	    let _true = Expression::_true();
	    assert_eq!(_true.flatten(), vec![FlatExpression(1)]);
	    let _false = Expression::_false();
	    assert_eq!(_false.flatten(), vec![FlatExpression(0)]);
	}
}

pub fn main() {

}

impl<T: Type> fmt::Display for Expression<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variant)
    }
}


