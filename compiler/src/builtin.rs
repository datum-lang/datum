use cjc_lexer::Location;
use cjc_parser::parse_tree::Type;

#[derive(PartialEq, Clone, Debug)]
pub enum Builtin {
    Assert,
    Print,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub builtin: Builtin,
    pub namespace: Option<&'static str>,
    pub name: &'static str,
    pub args: &'static [Type],
    pub ret: &'static [Type],
    pub doc: &'static str,
}

// A list of all Solidity builtins functions
static BUILTIN_FUNCTIONS: [Prototype; 3] = [
    Prototype {
        builtin: Builtin::Print,
        namespace: None,
        name: "print",
        args: &[Type::String],
        ret: &[Type::Void],
        doc: "log string without new line",
    },
    Prototype {
        builtin: Builtin::Print,
        namespace: None,
        name: "println",
        args: &[Type::String],
        ret: &[Type::Void],
        doc: "log string with line",
    },
    Prototype {
        builtin: Builtin::Assert,
        namespace: None,
        name: "assert",
        args: &[Type::Bool],
        ret: &[Type::Void],
        doc: "abort execution if argument evaluates to false",
    },
];

#[derive(Clone, PartialEq)]
pub enum Symbol {
    Function(Vec<Location>),
    Variable(Location, usize, usize),
    Struct(Location, usize),
    Import(Location, usize),
}

/// Does function call match builtin
pub fn is_builtin_call(namespace: Option<&str>, fname: &str) -> bool {
    BUILTIN_FUNCTIONS
        .iter()
        .any(|p| p.name == fname && p.namespace == namespace)
}

/// Resolve a builtin method call. The takes the unresolved arguments, since it has
/// to handle the special case "fmt.print("hello,world")" where the
/// second argument is a type list. The generic expression resolver cannot deal with
/// this. It is only used in for this specific call.
pub fn resolve_call(
    _name: &str,
    namespace: Option<&str>,
    id: &str,
    args: &Vec<cjc_parser::Argument>, // args: &[Expression],
) -> Result<GenExpression, ()> {
    let matches = BUILTIN_FUNCTIONS
        .iter()
        .filter(|p| p.name == id && p.namespace == namespace)
        .collect::<Vec<&Prototype>>();

    let mut resolved_args = Vec::new();
    for x in args {
        // todo: add parsed express
        resolved_args.push(x);
    }
    //
    // let mut cast_args = Vec::new();
    // for (i, arg) in resolved_args.iter().enumerate() {
    //     cast_args.push(arg);
    // }

    for func in &matches {
        if func.args.len() != args.len() {
            continue;
        }
        let matches = true;

        if matches {
            return Ok(GenExpression::Builtin {
                types: func.ret.to_vec(),
                builtin: func.builtin.clone(),
                // args: resolved_args,
            });
        }
        println!("{:?}", func);
    }

    Err(())
}

#[derive(PartialEq, Clone, Debug)]
pub enum GenExpression {
    Builtin {
        types: Vec<Type>,
        builtin: Builtin,
        // args: Vec<ExpressionType>,
    },
}

#[cfg(test)]
mod tests {
    use cjc_lexer::Location;
    use cjc_parser::parse_tree::{Expression, ExpressionType};

    use crate::builtin::is_builtin_call;

    #[test]
    fn should_identify_builtin_print() {
        let is_builtin = is_builtin_call(None, "print");
        assert_eq!(true, is_builtin);

        let no_builtin = is_builtin_call(None, "printf");
        assert_eq!(false, no_builtin);
    }

    #[test]
    fn should_resolved_call() {
        let expr = Expression {
            location: Location::new(0, 0),
            node: ExpressionType::String {
                value: "hello,world".to_string(),
            },
        };
        let mut exprs = vec![];
        exprs.push(expr);
        //
        // let result = resolve_call("demo", None, "print", &exprs);
        // assert_eq!(true, result.is_ok());
    }
}
