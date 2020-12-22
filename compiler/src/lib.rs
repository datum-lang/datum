pub use lowerify::*;
pub use meanify::*;
pub use neat::*;

pub mod lowerify;
pub mod meanify;
pub mod neat;

pub fn parse_and_resolve(input: &str, filename: &str) -> Namespace {
    let mut namespace = Namespace::new();
    namespace.files.push(filename.to_string());

    program(input, filename, &mut namespace);
    namespace
}

pub fn process_string(input: &str, filename: &str) -> Namespace {
    let mut namespace = parse_and_resolve(input, filename);
    meanify(&mut namespace);
    namespace
}

#[cfg(test)]
mod test {
    use cjc_hir::{Expression, Statement};

    use crate::{codegen, parse_and_resolve, process_string, CodegenResult};

    #[test]
    #[rustfmt::skip]
    fn should_build_print_builtin() {
        let ns = parse_and_resolve("default$main() {println(\"hello,world\");}", "hello.cj");
        assert_eq!(1, ns.functions.len());
        assert_eq!(1, ns.functions[0].body.len());

        let mut is_print_builtin = false;
        let statement = &ns.functions[0].body[0];
        if let Statement::Expression { location: _, expression } = statement {
            match expression {
                Expression::Builtin { location: _, types: _, builtin, args: _ } => {
                    if *builtin == cjc_hir::Builtin::Print {
                        is_print_builtin = true;
                    }
                }
                _ => {}
            }
        }

        assert_eq!(true, is_print_builtin);
    }

    #[test]
    #[rustfmt::skip]
    fn should_run_hello_world() {
        let mut ns = process_string("default$main() {println(\"hello,world\");}", "hello.cj");
        assert_eq!(1, ns.cfgs.len());
        let results = codegen(&mut ns, "jit");
        assert_eq!(1, results.len());
        match results[0] {
            CodegenResult::Jit { exit_code } => {
                assert_eq!(0, exit_code);
            }
            _ => {
                panic!("run hello, world failure");
            }
        }
    }

    #[test]
    #[rustfmt::skip]
    fn should_print_nums() {
        let mut ns = process_string("default$main() {println(5);}", "hello.cj");
        assert_eq!(1, ns.cfgs.len());
        let results = codegen(&mut ns, "jit");
        assert_eq!(1, results.len());
        match results[0] {
            CodegenResult::Jit { exit_code } => {
                assert_eq!(0, exit_code);
            }
            _ => {
                panic!("run hello, world failure");
            }
        }
    }

    #[test]
    #[rustfmt::skip]
    fn should_run_hello_world_utf8() {
        let mut ns = process_string("default$main() {println(\"你好，世界！\");}", "hello.cj");
        assert_eq!(1, ns.cfgs.len());
        let results = codegen(&mut ns, "jit");
        match results[0] {
            CodegenResult::Jit { exit_code } => {
                assert_eq!(0, exit_code);
            }
            _ => {
                panic!("run hello, world failure");
            }
        }
    }

    #[test]
    #[rustfmt::skip]
    fn should_down_wasm() {
        let mut ns = process_string("default$main() {println(\"hello,world\");}", "hello.cj");
        let results = codegen(&mut ns, "wasm");
        match &results[0] {
            CodegenResult::Wasm { code } => {
                assert!(code.len() > 0);
            }
            _ => {
                panic!("run hello, world failure");
            }
        }
    }

    #[test]
    #[rustfmt::skip]
    fn should_support_local_function_call() {
        let mut ns = process_string("default$main() {say_hello();}

default$say_hello() {println(\"你好，世界！\");}
", "hello.cj");
        assert_eq!(2, ns.cfgs.len());
        let _results = codegen(&mut ns, "jit");
        // todo: add linker
        // println!("{:?}", results);
    }
}
