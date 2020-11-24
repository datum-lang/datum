pub use expression::*;
pub use namespace::*;
pub use program::*;
pub use statements::*;
pub use struct_function::*;
pub use symbol_table::*;
pub use unit::*;

pub mod builtin;
pub mod expression;
pub mod namespace;
pub mod program;
pub mod statements;
pub mod struct_function;
pub mod symbol_table;
pub mod unit;

#[cfg(test)]
mod test {
    use crate::neat::program::program;

    #[test]
    fn init() {
        let _result = program("default$main() {println(\"hello,world\")}", "hello.cj");
    }
}
