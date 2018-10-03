// FooType implements Type
// FooExpression implements Expression<FooType>
// FooExpression carries FooType and FooExpression

mod types;
use self::types::*;
use flat_ast::*;
use std::fmt::{Debug, Display};

///////////////////////////////////////////////////////////////////

pub trait Expression: Debug + Display {
    fn flatten(&self, flattened_statements: &mut Vec<FlatStatement>) -> Vec<LinComb>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable(String);

impl Variable {
    pub fn with_name<S: Into<String>>(name: S) -> Self {
        Variable(name.into())
    }

    pub fn name(&self) -> &String {
        &self.0
    }
}

#[derive(Debug)]
enum Statement {
    Definition(Variable, Box<Expression>),
    Return(Vec<Box<Expression>>),
}

impl Statement {
    fn flatten(self, flattened_statements: &mut Vec<FlatStatement>) {
        match self {
            Statement::Definition(v, e) => {
                let e = e.flatten(flattened_statements);

                flattened_statements.push(FlatStatement::Directive(
                    e.iter()
                        .enumerate()
                        .map(|(index, _)| {
                            FlatVariable::with_name(format!("{}_{}", v.name(), index))
                        }).collect(),
                ));
                for (index, e) in e.iter().enumerate() {
                    flattened_statements.push(FlatStatement::Assertion(
                        LinComb(vec![(
                            1,
                            FlatVariable::with_name(format!("{}_{}", v.name(), index)),
                        )]),
                        LinComb(vec![(1, FlatVariable::one())]),
                        e.clone(),
                    ));
                }
            }
            Statement::Return(e) => {
                flattened_statements.push(FlatStatement::Return(vec![]));
            }
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

        FlatFunction::with_statements(flattened_statements)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn array_expression() {
        let v0 = Array::value(vec![FieldElement::Value(0), FieldElement::Value(1)]);
        println!("{}", v0);
    }

    #[test]
    fn nested_array_expression() {
        let v1 = Array::value(vec![
            Array::value(vec![FieldElement::Value(0), FieldElement::Value(1)]),
            Array::value(vec![FieldElement::Value(0), FieldElement::Value(1)]),
        ]);
        println!("{}", v1);
    }

    #[test]
    fn definition() {
        let _s = Statement::Definition(Variable::with_name("a"), box Boolean::_true());
    }

    #[test]
    fn return_statement() {
        let _r: Statement =
            Statement::Return(vec![box Boolean::Value(true), box FieldElement::Value(1)]);
    }

    #[test]
    #[should_panic]
    fn wrong_count_in_def() {
        let _e2 = Array::value(vec![
            Array::value(vec![
                FieldElement::Value(0), // /!\ here 1 element /!\
            ]),
            Array::value(vec![
                FieldElement::Value(0), // here 3 elements
                FieldElement::Value(1),
            ]),
        ]); // should panic
    }

    #[test]
    fn flatten_function() {
        let f = Function {
            statements: vec![
                Statement::Definition(Variable::with_name("a"), box FieldElement::Value(42)),
                Statement::Definition(
                    Variable::with_name("b"),
                    box FieldElement::Add(
                        box FieldElement::Identifier(Variable::with_name("a")),
                        box FieldElement::Value(3),
                    ),
                ),
                Statement::Return(vec![box FieldElement::Add(
                    box FieldElement::Identifier(Variable::with_name("b")),
                    box FieldElement::Value(1),
                )]),
            ],
        };

        println!("{:#?}", f);

        let flattened = f.flatten();

        println!("{}", flattened);

        assert_eq!(flattened, FlatFunction::with_statements(vec![]));
    }

    #[test]
    fn flatten_definition() {
        let f2 = Function {
            statements: vec![Statement::Definition(
                Variable::with_name("a"),
                box Array::Value(vec![FieldElement::Value(42), FieldElement::Value(55)]),
            )],
        };

        let flattened = f2.flatten();

        println!("{}", flattened);

        assert_eq!(
            flattened,
            FlatFunction::with_statements(vec![
                FlatStatement::Directive(vec![
                    FlatVariable::with_name("a_0"),
                    FlatVariable::with_name("a_1")
                ]),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_0"))]),
                    LinComb(vec![(1, FlatVariable::one())]),
                    LinComb(vec![(42, FlatVariable::one())]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_1"))]),
                    LinComb(vec![(1, FlatVariable::one())]),
                    LinComb(vec![(55, FlatVariable::one())]),
                )
            ])
        );
    }

    #[test]
    fn flatten_nested_definition() {
        // field[2][2] a = [[42, 55], [42, 55]]
        // ->
        // # a_0, a_1, a_2, a_3 := ... // this will be a directive, could also be multiple..
        // (1 * a_0) * (1 * one) == (42 * one)
        // (1 * a_1) * (1 * one) == (55 * one)
        // (1 * a_2) * (1 * one) == (42 * one)
        // (1 * a_3) * (1 * one) == (55 * one)

        let f2 = Function {
            statements: vec![Statement::Definition(
                Variable::with_name("a"),
                box Array::Value(vec![
                    Array::Value(vec![FieldElement::Value(42), FieldElement::Value(55)]),
                    Array::Value(vec![FieldElement::Value(42), FieldElement::Value(55)]),
                ]),
            )],
        };

        let flattened = f2.flatten();

        println!("{}", flattened);

        assert_eq!(
            flattened,
            FlatFunction::with_statements(vec![
                FlatStatement::Directive(vec![
                    FlatVariable::with_name("a_0"),
                    FlatVariable::with_name("a_1"),
                    FlatVariable::with_name("a_2"),
                    FlatVariable::with_name("a_3"),
                ]),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_0"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(42, FlatVariable::one(),)]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_1"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(55, FlatVariable::one(),)]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_2"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(42, FlatVariable::one(),)]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_3"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(55, FlatVariable::one(),)]),
                ),
            ])
        );
    }

    #[test]
    fn flatten_double_nested_definition() {
        // field[2][2][2] a = [[[42, 55], [42, 55]],[[42, 55], [42, 55]]]
        // ->
        // # a_0, a_1, a_2, a_3 := ... // this will be a directive, could also be multiple..
        // (1 * a_0) * (1 * one) == (42 * one)
        // (1 * a_1) * (1 * one) == (55 * one)
        // (1 * a_2) * (1 * one) == (42 * one)
        // (1 * a_3) * (1 * one) == (55 * one)

        let f2 = Function {
            statements: vec![Statement::Definition(
                Variable::with_name("a"),
                box Array::Value(vec![
                    Array::Value(vec![
                        Array::Value(vec![FieldElement::Value(42), FieldElement::Value(55)]),
                        Array::Value(vec![FieldElement::Value(42), FieldElement::Value(55)]),
                    ]),
                    Array::Value(vec![
                        Array::Value(vec![FieldElement::Value(42), FieldElement::Value(55)]),
                        Array::Value(vec![FieldElement::Value(42), FieldElement::Value(55)]),
                    ]),
                ]),
            )],
        };

        let flattened = f2.flatten();

        println!("{}", flattened);

        assert_eq!(
            flattened,
            FlatFunction::with_statements(vec![
                FlatStatement::Directive(vec![
                    FlatVariable::with_name("a_0"),
                    FlatVariable::with_name("a_1"),
                    FlatVariable::with_name("a_2"),
                    FlatVariable::with_name("a_3"),
                    FlatVariable::with_name("a_4"),
                    FlatVariable::with_name("a_5"),
                    FlatVariable::with_name("a_6"),
                    FlatVariable::with_name("a_7"),
                ]),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_0"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(42, FlatVariable::one(),)]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_1"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(55, FlatVariable::one(),)]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_2"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(42, FlatVariable::one(),)]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_3"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(55, FlatVariable::one(),)]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_4"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(42, FlatVariable::one(),)]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_5"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(55, FlatVariable::one(),)]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_6"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(42, FlatVariable::one(),)]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_7"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(55, FlatVariable::one(),)]),
                ),
            ])
        );
    }

    #[test]
    fn flatten_structure_definition() {
        // 
        // {foo: field, bar: bool} a = {foo: 42, bar: true}
        // ->
        // # a_0, a_1 := ... // this will be a directive, could also be multiple..
        // (1 * a_0) * (1 * one) == (42 * one)
        // (1 * a_1) * (1 * one) == (1 * one)

        let f2 = Function {
            statements: vec![Statement::Definition(
                Variable::with_name("a"),
                box Structure::Value(vec![
                    box FieldElement::Value(42), 
                    box Boolean::Value(true),
                ])
            )],
        };

        let flattened = f2.flatten();

        println!("{}", flattened);

        assert_eq!(
            flattened,
            FlatFunction::with_statements(vec![
                FlatStatement::Directive(vec![
                    FlatVariable::with_name("a_0"),
                    FlatVariable::with_name("a_1"),
                ]),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_0"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(42, FlatVariable::one(),)]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_1"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                ),
            ])
        );
    }

    #[test]
    fn flatten_recursive_structure_definition() {
        // 
        // {foo: { baz: field, qux: field[2] }, bar: bool} a = {foo: { baz: 42, qux: [21, 21] }, bar: true}
        // ->
        // # a_0, a_1, a_3, a_4 := ... // this will be a directive, could also be multiple..
        // (1 * a_0) * (1 * one) == (42 * one)
        // (1 * a_1) * (1 * one) == (21 * one)
        // (1 * a_0) * (1 * one) == (21 * one)
        // (1 * a_1) * (1 * one) == (1 * one)

        let f2 = Function {
            statements: vec![Statement::Definition(
                Variable::with_name("a"),
                box Structure::Value(vec![
                    box Structure::Value(vec![
                        box FieldElement::Value(42),
                        box Array::Value(vec![FieldElement::Value(21), FieldElement::Value(21)])]),
                    box Boolean::Value(true),
                ])
            )],
        };

        let flattened = f2.flatten();

        println!("{}", flattened);

        assert_eq!(
            flattened,
            FlatFunction::with_statements(vec![
                FlatStatement::Directive(vec![
                    FlatVariable::with_name("a_0"),
                    FlatVariable::with_name("a_1"),
                    FlatVariable::with_name("a_2"),
                    FlatVariable::with_name("a_3"),
                ]),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_0"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(42, FlatVariable::one(),)]),
                ),
                FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_1"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(21, FlatVariable::one(),)]),
                ),
                    FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_2"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(21, FlatVariable::one(),)]),
                ),
                    FlatStatement::Assertion(
                    LinComb(vec![(1, FlatVariable::with_name("a_3"),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                    LinComb(vec![(1, FlatVariable::one(),)]),
                ),
            ])
        );
    }

    // #[test]
    // fn flatten_primitive_types() {
    //     // we can flatten expressions
    //     let _42_plus_42 = FieldElement::Add(
    //         box FieldElement::Value(42),
    //         box FieldElement::Value(42),
    //     );
    //     assert_eq!(_42_plus_42.flatten(), vec![LinComb(vec![])]);
    //     let _true = Boolean::_true();
    //     assert_eq!(_true.flatten(), vec![LinComb(vec![])]);
    //     let _false = Boolean::_false();
    //     assert_eq!(_false.flatten(), vec![LinComb(vec![])]);
    // }
}

pub fn main() {}
