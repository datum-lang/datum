## Platform Support

### Tier 1

| target | std | host | notes |
| --- | --- | --- | --- |
| `i686-pc-windows-gnu` | ✓ | ✓ | 32-bit MinGW (Windows 7+) |
| `i686-pc-windows-msvc` | ✓ | ✓ | 32-bit MSVC (Windows 7+) |
| `i686-unknown-linux-gnu` | ✓ | ✓ | 32-bit Linux (kernel 2.6.32+, glibc 2.11+) |
| `x86_64-apple-darwin` | ✓ | ✓ | 64-bit macOS (10.7+, Lion+) |
| `x86_64-pc-windows-gnu` | ✓ | ✓ | 64-bit MinGW (Windows 7+) |
| `x86_64-pc-windows-msvc` | ✓ | ✓ | 64-bit MSVC (Windows 7+) |
| `x86_64-unknown-linux-gnu` | ✓ | ✓ | 64-bit Linux (kernel 2.6.32+, glibc 2.11+) |
| `wasm32-unknown-unknown` | ✓ |   | WebAssembly |

### Tier 2

| target | std | host | notes |
| --- | --- | --- | --- |
| `wasm32-wasi` | ✓ |   | WebAssembly with WASI |
| `wasm32-unknown-emscripten` | ✓ |   | WebAssembly via Emscripten |
| `aarch64-unknown-linux-gnu` | ✓ | ✓ | ARM64 Linux (kernel 4.2, glibc 2.17+) |

## Usage

examples:

 - JavaScript/Java to WASM

TBD:

 - C++ to Charj. (partially support)

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
