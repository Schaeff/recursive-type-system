use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct LinComb(pub Vec<(usize, FlatVariable)>);

#[derive(Debug, PartialEq)]
pub enum FlatStatement {
    Directive(Vec<FlatVariable>),
    Assertion(LinComb, LinComb, LinComb),
    Return(Vec<FlatVariable>),
}

#[derive(Debug, PartialEq)]
pub struct FlatFunction {
    statements: Vec<FlatStatement>,
}

impl FlatFunction {
    pub fn with_statements(statements: Vec<FlatStatement>) -> Self {
        FlatFunction { statements }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FlatVariable(String);

impl FlatVariable {
    pub fn with_name<S: Into<String>>(name: S) -> Self {
        FlatVariable(name.into())
    }

    pub fn one() -> Self {
        Self::with_name("~one")
    }
}

impl fmt::Display for FlatFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.statements
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl fmt::Display for FlatVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for LinComb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|(mult, var)| format!("{} * {}", mult, var))
                .collect::<Vec<_>>()
                .join(" + ")
        )
    }
}

impl fmt::Display for FlatStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FlatStatement::Directive(ref vars) => write!(
                f,
                "# {} := TODO_DIRECTIVE(TODO_INPUTS)",
                vars.iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            FlatStatement::Assertion(ref a, ref b, ref c) => {
                write!(f, "({}) * ({}) == ({})", a, b, c)
            }
            FlatStatement::Return(ref vars) => write!(
                f,
                "return {}",
                vars.iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
