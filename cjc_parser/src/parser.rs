use crate::charj;
use crate::error::Diagnostic;
use crate::lexer;
use crate::parse_tree::SourceUnit;

macro_rules! do_lalr_parsing {
    ($input: expr) => {{
        let lex = lexer::Lexer::new($input);
        match charj::CharjParser::new().parse($input, lex) {
            Err(err) => Err(Diagnostic::handle_error(err)),
            Ok(s) => Ok(s),
        }
    }};
}

pub fn parse_program(source: &str) -> Result<SourceUnit, Diagnostic> {
    do_lalr_parsing!(source)
}

#[cfg(test)]
mod test {
    use crate::location::Loc;
    use crate::parse_tree::{Identifier, Package, SourceUnit, SourceUnitPart};
    use crate::parser::parse_program;

    #[test]
    #[rustfmt::skip]
    fn test_parse_empty() {
        let parse_ast = parse_program("");
        assert!(parse_ast.is_err());
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_package() {
        let package = parse_program("package charj");
        assert_eq!(package.unwrap(), SourceUnit {
            0: vec![SourceUnitPart::PackageDirective(Package::Plain(
                Identifier {
                    loc: Loc(8, 13),
                    name: "charj".to_string(),
                }
            ))]
        });
        let pkg_alias = parse_program("pkg charj");
        assert!(pkg_alias.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_struct() {
        let package = parse_program("struct IO {}");
        assert!(package.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_basic_location() {
        let code = parse_program("pkg charj
struct IO {}");
        assert!(code.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_normal_struct_function() {
        let normal_struct_fun = parse_program("default$main() {}");
        assert!(normal_struct_fun.is_ok());
        let with_empty_struct_fun = parse_program("default $ main () {}");
        assert!(with_empty_struct_fun.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_function_parameters() {
        let params = parse_program("default$main(string name) {}");
        assert!(params.is_ok());

        let multi_params = parse_program("default$main(string name, string first, int id) {}");
        assert!(multi_params.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_comment() {
        let comments = parse_program("// this is a comment
pkg comment
");
        assert!(comments.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_if_statement() {
        let empty_if = parse_program("default$main(string name) {
    if(string == \"name\") {
        return;
    }
}");
        assert!(empty_if.is_ok());

        let if_with_expr = parse_program("default$main(string name) {
    if( a == true) {}
}");
        assert!(if_with_expr.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_while() {
        let empty_if = parse_program("default$main(string name) {
    while(string == \"name\") {
        return;
    }
}");
        assert!(empty_if.is_ok());

        let if_with_expr = parse_program("default$main(string name) {
    while( a == true) {}
}");
        assert!(if_with_expr.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_return() {
        let if_return = parse_program("default$main(string name) {
    if(a == true) {
        return a;
    }
}");
        assert!(if_return.is_ok());

        let if_greater = parse_program("default$main(int a, int b) {
    if(a > b) {
        return a;
    }
}");
        assert!(if_greater.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_if_else() {
        let if_else = parse_program("default$compare(int a, int b) {
    if(a > b) {
        return a;
    } else {
        return b;
    }
}");
        assert!(if_else.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_function_return() {
        let function_return = parse_program("default$compare(int a, int b) -> int {
    if(a > b) {
        return a;
    } else {
        return b;
    }
}");

        match function_return.unwrap().0.get(0).unwrap() {
            SourceUnitPart::StructFuncDef(def) => {
                let string = format!("{:?}", def.returns.as_ref().unwrap().node);
                assert_eq!("Type { ty: Int(256) }", string);
            }
            _ => {
                panic!("expected get StructFuncDef")
            }
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_function_string_return() {
        let function_return = parse_program("default$compare(int a, int b) -> string {
}");

        match function_return.unwrap().0.get(0).unwrap() {
            SourceUnitPart::StructFuncDef(def) => {
                let string = format!("{:?}", def.returns.as_ref().unwrap().node);
                assert_eq!("Type { ty: String }", string);
            }
            _ => {
                panic!("expected get StructFuncDef")
            }
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_import() {
        let parse_ast = parse_program("import io");
        assert!(parse_ast.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_function_call() {
        let basic_function_call = parse_program("default$main(string name) {
    println(\"hello,world\")
}");
        assert!(basic_function_call.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_struct_vars() {
        let code = parse_program("pkg charj
struct Summary {
  	Name   : string
	FanIn  : int
	FanOut : int
}");

        match code.unwrap().0.get(1).unwrap() {
            SourceUnitPart::StructDef(def) => {
                assert_eq!("Summary", def.name.name);
            }
            _ => {
                panic!("expected get StructDef")
            }
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_struct_array_vars() {
        let code = parse_program("pkg charj
struct Summary {
  	Name   : []string
}");
        assert!(code.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_struct_with_method_define() {
        let code = parse_program("pkg charj
struct Summary {
  	Name   : string
}

Summary$constructor(string name) {
}
");

        match code.unwrap().0.get(2).unwrap() {
            SourceUnitPart::StructFuncDef(def) => {
                assert_eq!("Summary", def.struct_name.name);
            }
            _ => {
                panic!("expected get StructDef")
            }
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_struct_in_struct() {
        let code = parse_program("pkg charj
struct Summary {
  	Name   : string
	FanIn  : int
	FanOut : int
}

struct Hello {
    summary : Summary
}
");
        assert!(code.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_assign() {
        let str_assign = parse_program("default$main() {
    let words: string  = \"hello,world\"
    println(words)
    let b: int = 2333
    println(b)
}");
        assert!(str_assign.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_assign_with_sum() {
        let str_assign = parse_program("default$main() {
    let b: int = 2333 + 5
    let c: int = b - 10
    println(b)
}");
        assert!(str_assign.is_ok());

        let multiple_expr = parse_program("default$main() {
    let b: int = 2333 + 5 - 10 -10 + 5 + 100
}");
        assert!(multiple_expr.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_mul() {
        let mul = parse_program("default$main() {
    let b: int = 2333 * 5 - 10 + 100
    println(b)
}");
        assert!(mul.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_basic_div() {
        let mul = parse_program("default$main() {
    let b: int = 2333 * 5 - 10 + 100 / 5
    println(b)
}");
        assert!(mul.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_basic_mode() {
        let mod_code = parse_program("default$main() {
    let b: int = 100 % 5
    println(b)
}");
        assert!(mod_code.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_for_and_or_symbol() {
        let and_symbol = parse_program("default$main() {
    let b: bool = a && b
}");
        assert!(and_symbol.is_ok());

        let or_symbol = parse_program("default$main() {
    let b: bool = a || b
}");
        assert!(or_symbol.is_ok());

        let complex = parse_program("default$main() {
    let b: bool = a || b && c || d && e || f
}");
        assert!(complex.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_for_loop() {
        let for_loop = parse_program("default$main(string name) {
    for(x in 1..10) {
        println(x)
    }
}");
        assert!(for_loop.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_for_not() {
        let not_cond = parse_program("default$main(string name) {
    if (!true){}
}");
        assert!(not_cond.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_for_shift() {
        let shift = parse_program("default$main(string name) {
    let a: int = 1000 << 0
    let b: int = 1000 >> 1
}");
        assert!(shift.is_ok());
    }

    #[test]
    #[ignore]
    #[rustfmt::skip]
    fn test_for_complex_if() {
        let complex_not_cond = parse_program("default$main(string name) {
    if ((i % 3) == 0) {}
}");
        println!("{:?}", complex_not_cond);
        assert!(complex_not_cond.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn test_for_array() {
        let array = parse_program("default$main(string name) {
    let i: []int = [1, 2, 3]
    let j: string = [1, 2, 3]
}");
        assert!(array.is_ok());
    }
}
