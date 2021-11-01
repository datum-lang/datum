## Development

 - dc_lexer, aka lexer
 - dc_parser (outputs: AST)
 - dc_hir (define: CFG)
 - dc_mir (define: MIR)
 - compiler (outputs: LLVM IR)
    - neat. AST -> HIR
    - medium. HIR -> MIR (TBD)
    - lowerify. MIR -> LLVM IR
 - dc_codegen (process MIR -> LLVM IR)

## Process

-----> parser ----> AST ------------compiler------------------> LLVM IR
lexer -----> parser -----> hir -----> mir -----> codegen -----> LLVM IR
     lalrpop         AST


## Setup LLVM

```
brew install llvm
```

## Target Platform Support (Plan)

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
