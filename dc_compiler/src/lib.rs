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
    use crate::{codegen, process_string};

    #[test]
    #[rustfmt::skip]
    fn should_support_local_function_call() {
        let mut ns = process_string("
default$say_hello() {println(\"hello, world\");println(5);}
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
