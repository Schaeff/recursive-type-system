// FooType implements Type
// FooVariant implements Variant<FooType>
// FooExpression carries FooType and FooVariant

#![feature(box_patterns, box_syntax)]

mod types;

use std::fmt;
use types::*;

///////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone)]
pub struct LinComb(Vec<(usize, Variable)>);

#[derive(Debug, PartialEq)]
pub enum FlatStatement {
    Directive(Vec<Variable>),
    Assertion(LinComb, LinComb, LinComb),
    Return(Vec<Variable>),
}

#[derive(Debug, PartialEq)]
pub struct FlatFunction {
    statements: Vec<FlatStatement>,
}

pub trait Flatten {
    fn flatten(&self, flattened_statements: &mut Vec<FlatStatement>) -> Vec<LinComb>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    name: String,
}

#[derive(Debug)]
enum Statement {
    Definition(Variable, Box<Variant>),
    Return(Vec<Box<Variant>>),
}

impl Statement {
    fn flatten(self, flattened_statements: &mut Vec<FlatStatement>) {
        match self {
            Statement::Definition(v, e) => {
                let e = e.flatten(flattened_statements);

                flattened_statements.push(FlatStatement::Directive(
                    e.iter()
                        .enumerate()
                        .map(|(index, _)| Variable {
                            name: format!("{}_{}", v.name, index),
                        }).collect(),
                ));
                for (index, e) in e.iter().enumerate() {
                    flattened_statements.push(FlatStatement::Assertion(
                        LinComb(vec![(
                            1,
                            Variable {
                                name: format!("{}_{}", v.name, index),
                            },
                        )]),
                        LinComb(vec![(
                            1,
                            Variable {
                                name: String::from("~one"),
                            },
                        )]),
                        e.clone(),
                    ));
                }
            }
            Statement::Return(e) => flattened_statements.push(FlatStatement::Return(vec![])),
        }
    }
}

#[derive(Debug)]
struct Function {
    statements: Vec<Statement>,
}

impl Function {
    fn flatten(self) -> FlatFunction {
        let mut flattened_statements = vec![];
        for s in self.statements {
            s.flatten(&mut flattened_statements)
        }

        FlatFunction {
            statements: flattened_statements,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn array_expression() {
        let v0 = ArrayVariant::value(vec![
            FieldElementVariant::Value(0),
            FieldElementVariant::Value(1),
        ]);
        println!("{}", v0);
    }

    #[test]
    fn nested_array_expression() {
        let v1 = ArrayVariant::value(vec![
            ArrayVariant::value(vec![
                FieldElementVariant::Value(0),
                FieldElementVariant::Value(1),
            ]),
            ArrayVariant::value(vec![
                FieldElementVariant::Value(0),
                FieldElementVariant::Value(1),
            ]),
        ]);
        println!("{}", v1);
    }

    #[test]
    fn definition() {
        let _s = Statement::Definition(
            Variable {
                name: String::from("a"),
            },
            box BooleanVariant::_true(),
        );
    }

    #[test]
    fn return_statement() {
        let _r: Statement = Statement::Return(vec![
            box BooleanVariant::Value(true),
            box FieldElementVariant::Value(1),
        ]);
    }

    #[test]
    #[should_panic]
    fn wrong_count_in_def() {
        let _e2 = ArrayVariant::value(vec![
            ArrayVariant::value(vec![
                FieldElementVariant::Value(0), // /!\ here 1 element /!\
            ]),
            ArrayVariant::value(vec![
                FieldElementVariant::Value(0), // here 3 elements
                FieldElementVariant::Value(1),
            ]),
        ]); // should panic
    }

    #[test]
    fn flatten_function() {
        let f = Function {
            statements: vec![
                Statement::Definition(
                    Variable {
                        name: String::from("a"),
                    },
                    box FieldElementVariant::Value(42),
                ),
                Statement::Definition(
                    Variable {
                        name: String::from("b"),
                    },
                    box FieldElementVariant::Add(
                        box FieldElementVariant::Identifier(Variable {
                            name: String::from("a"),
                        }),
                        box FieldElementVariant::Value(3),
                    ),
                ),
                Statement::Return(vec![box FieldElementVariant::Add(
                    box FieldElementVariant::Identifier(Variable {
                        name: String::from("b"),
                    }),
                    box FieldElementVariant::Value(1),
                )]),
            ],
        };

        println!("{:#?}", f);

        let flattened = f.flatten();

        println!("{:#?}", flattened);

        assert_eq!(flattened, FlatFunction { statements: vec![] });
    }

    #[test]
    fn flatten_definition() {
        let f2 = Function {
            statements: vec![Statement::Definition(
                Variable {
                    name: String::from("a"),
                },
                box ArrayVariant::Value(vec![
                    FieldElementVariant::Value(42),
                    FieldElementVariant::Value(55),
                ]),
            )],
        };

        let flattened = f2.flatten();

        println!("{:#?}", flattened);

        assert_eq!(
            flattened,
            FlatFunction {
                statements: vec![
                    FlatStatement::Directive(vec![
                        Variable {
                            name: String::from("a_0")
                        },
                        Variable {
                            name: String::from("a_1")
                        }
                    ]),
                    FlatStatement::Assertion(
                        LinComb(vec![(
                            1,
                            Variable {
                                name: String::from("a_0")
                            }
                        )]),
                        LinComb(vec![(
                            1,
                            Variable {
                                name: String::from("~one")
                            }
                        )]),
                        LinComb(vec![(
                            42,
                            Variable {
                                name: String::from("~one")
                            }
                        )]),
                    ),
                    FlatStatement::Assertion(
                        LinComb(vec![(
                            1,
                            Variable {
                                name: String::from("a_1")
                            }
                        )]),
                        LinComb(vec![(
                            1,
                            Variable {
                                name: String::from("~one")
                            }
                        )]),
                        LinComb(vec![(
                            55,
                            Variable {
                                name: String::from("~one")
                            }
                        )]),
                    )
                ]
            }
        );
    }

    // #[test]
    // fn flatten_primitive_types() {
    //     // we can flatten expressions
    //     let _42_plus_42 = FieldElementVariant::Add(
    //         box FieldElementVariant::Value(42),
    //         box FieldElementVariant::Value(42),
    //     );
    //     assert_eq!(_42_plus_42.flatten(), vec![LinComb(vec![])]);
    //     let _true = BooleanVariant::_true();
    //     assert_eq!(_true.flatten(), vec![LinComb(vec![])]);
    //     let _false = BooleanVariant::_false();
    //     assert_eq!(_false.flatten(), vec![LinComb(vec![])]);
    // }
}

pub fn main() {}
