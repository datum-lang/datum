## Development

 - cjc_lexer, aka lexer
 - cjc_parser (outputs: AST)
 - cjc_hir (define: CFG)
 - cjc_mir (define: MIR)
 - compiler (outputs: LLVM IR)
    - neat. AST -> HIR
    - medium. HIR -> MIR (TBD)
    - lowerify. MIR -> LLVM IR
 - cjc_codegen (process MIR -> LLVM IR)

## Process

-----> parser ----> AST ------------compiler------------------> LLVM IR
lexer -----> parser -----> hir -----> mir -----> codegen -----> LLVM IR
     lalrpop         AST

### Setup LLVM

we use [llvmenv](https://github.com/termoshtt/llvmenv) to manage multiple LLVM/Clang build in macOS.

#### install

```bash
cargo install llvmenv
```

usage

```
llvmenv init
llvmenv entries
llvmenv build-entry 10.0.0
```

#### offline build

if failure to download from GitHub, try download and build in local

```
llvmenv build-entry local-llvm
```

then export:

```
LLVM_SYS_100_PREFIX=$HOME/llvm/llvm-10.0.1.src/build cargo build
LLVM_SYS_100_PREFIX=$HOME/llvm/llvm-10.0.1.src/build cargo run
```

### Clion Debug Config

add: `LLVM_SYS_100_PREFIX=/Users/fdhuang/llvm/llvm-10.0.1.src/build` to **Run/Debug Configurations**.
