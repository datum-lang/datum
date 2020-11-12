# Roadmap

 - [ ] make compiler works
 - [ ] design MIR
     - [ ] charj is a MIR for high level
 - [ ] target
     - [ ] WASM
     - [ ] Desktop
 - [ ] convert
     - [ ] compile from java/javascipt
     - [ ] compile to java/javascript

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
    - [ ] poet design
 - [x] compiler
    - [x] LLVM spike
    - [x] hello, world
 - [ ] build system
 - [ ] dependency manager

Others todo:

 - [ ] CI
    - [ ] replace Solang CI container.
