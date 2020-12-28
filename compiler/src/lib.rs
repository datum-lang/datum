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
    use crate::{codegen, process_string, CodegenResult};

    #[test]
    #[rustfmt::skip]
    fn should_run_print_number() {
        let mut ns = process_string("default$main() {println(8848);}", "hello.cj");
        let results = codegen(&mut ns, "jit");
        assert_eq!(1, results.len());

        if let CodegenResult::Jit { exit_code } = results[0] {
            return assert_eq!(0, exit_code);
        }

        panic!("run number failure");
    }

    #[test]
    #[rustfmt::skip]
    fn should_print_nums() {
        let mut ns = process_string("default$main() {println(5);}", "hello.cj");
        let results = codegen(&mut ns, "jit");
        assert_eq!(1, results.len());

        if let CodegenResult::Jit { exit_code } = results[0] {
            return assert_eq!(0, exit_code);
        }

        panic!("run hello, world failure");
    }

    #[test]
    #[rustfmt::skip]
    fn should_run_hello_world_utf8() {
        let mut ns = process_string("default$main() {println(\"你好，世界！\");}", "hello.cj");
        let results = codegen(&mut ns, "jit");

        if let CodegenResult::Jit { exit_code } = results[0] {
            return assert_eq!(0, exit_code);
        }

        panic!("run hello, world failure");
    }

    #[test]
    #[rustfmt::skip]
    fn should_down_wasm() {
        let mut ns = process_string("default$main() {println(\"hello,world\");}", "hello.cj");
        let results = codegen(&mut ns, "wasm");

        if let CodegenResult::Wasm { code } = &results[0] {
            assert!(code.len() > 0);
            return;
        }

        panic!("run hello, world failure");
    }

    #[test]
    #[rustfmt::skip]
    fn should_support_local_function_call() {
        let mut ns = process_string("
default$say_hello() {println(\"你好，世界！\");}
default$main() {say_hello();}
", "hello.cj");
        assert_eq!("say_hello", ns.cfgs[0].name);
        assert_eq!("main", ns.cfgs[1].name);
        let _results = codegen(&mut ns, "jit");
    }

    #[test]
    #[rustfmt::skip]
    fn should_support_local_function_call_utf8() {
        let mut ns = process_string("
default$你好() {println(\"你好，世界！\");}
default$main() {你好();}
", "hello.cj");
        assert_eq!("你好", ns.cfgs[0].name);
        assert_eq!("main", ns.cfgs[1].name);
        let _results = codegen(&mut ns, "jit");
    }

    #[test]
    #[rustfmt::skip]
    fn should_run_function_after_main() {
        
    }
}
