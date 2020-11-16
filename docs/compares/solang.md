Solang does not have complete language support yet. The language features which are supported are clearly documented. See: [https://solang.readthedocs.io/](https://solang.readthedocs.io/)

Solang tries to be a traditional compiler

1.  lexer
2.  parser (outputs: AST)
3.  resolver: (outputs: CFG)
4.  code emitter (output: LLVM IR)
5.  llvm wasm codegen
6.  linker

The layout of the source code is as follows:

### src/parse/*

lexer and LALRPOP Solidity grammer

output: Abstract Syntax Tree

### src/resolve/*

Resolves types, variables, functions etc

Generates control flow graph

### src/emit/*

Converts Control Flow graph to LLVM IR

Has to do some tricks to generate PHI nodes

ABI encoder/decoder (eth/scale)

### src/link.rs

Converts wasm object file to final wasm file

### src/abi/*

Generates and reads ABIs
