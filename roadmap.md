# Roadmap

##

 - [x] implementation basic compiler logic
     - [x] hir convert (neat)
     - [x] mir convert (medium)
     - [x] to LLVM     (codegen)
 - [ ] design HIR
 - [ ] design MIR
     - [ ] charj is a MIR for high level
 - [ ] target
     - [ ] WASM
     - [x] Desktop
 - [ ] build system
 - [ ] package manager
 - [ ] dependency manager

Others todo:

 - [ ] CI
    - [ ] replace Solang CI container.

## Make compiler works

 - [ ] lexer & parser
    - [ ] syntax design
       - [x] import
       - [x] package
       - [x] struct
       - [x] function
       - [ ] control flow
          - [x] if
          - [ ] loop
          - [ ] while
          - [ ] for
          - [ ] break
       - [ ] expression
          - [x] assignment
          - [x] method call
 - [x] compiler
    - [x] LLVM spike
    - [x] hello, world
