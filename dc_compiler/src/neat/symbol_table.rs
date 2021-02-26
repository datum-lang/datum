use core::fmt;

use indexmap::map::IndexMap;

#[derive(Clone, Copy, PartialEq)]
pub enum SymbolTableType {
    Module,
    // same to class symbol
    Struct,
    Function,
    Variable,
    BuiltinType,
}

impl fmt::Display for SymbolTableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SymbolTableType::Module => write!(f, "module"),
            SymbolTableType::Struct => write!(f, "struct"),
            SymbolTableType::Function => write!(f, "function"),
            SymbolTableType::Variable => write!(f, "variable"),
            SymbolTableType::BuiltinType => write!(f, "builtintype"),
        }
    }
}

#[derive(Clone)]
pub struct SymbolTable {
    /// The name of this symbol table. Often the name of the class or function.
    pub name: String,

    /// The type of symbol table
    pub typ: SymbolTableType,

    /// The line number in the sourcecode where this symboltable begins.
    pub line_number: usize,

    // Return True if the block is a nested class or function
    pub is_nested: bool,

    /// A set of symbols present on this scope level.
    pub symbols: IndexMap<String, Symbol>,

    /// A list of subscopes in the order as found in the
    /// AST nodes.
    pub sub_tables: Vec<SymbolTable>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            name: "".to_string(),
            typ: SymbolTableType::Module,
            line_number: 0,
            is_nested: false,
            symbols: Default::default(),
            sub_tables: vec![],
        }
    }
}

/// Indicator for a single symbol what the scope of this symbol is.
/// The scope can be unknown, which is unfortunate, but not impossible.
#[derive(Debug, Clone)]
pub enum SymbolScope {
    Global,
    Nonlocal,
    Local,
    Unknown,
}

/// A single symbol in a table. Has various properties such as the scope
/// of the symbol, and also the various uses of the symbol.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    // pub table: SymbolTableRef,
    pub scope: SymbolScope,
}
