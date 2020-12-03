# Roadmap

## Todo

### Main

High priority:

 - [x] implementation basic compiler logic
     - [x] hir convert (neat)
     - [x] mir convert (medium)
     - [x] to LLVM     (codegen)
 - [ ] improve Charj syntax
 - [ ] design HIR
 - [ ] design MIR
     - [ ] charj is a MIR for high level

Medium priority:

 - [ ] multiple-target
     - [ ] WASM
     - [x] Desktop
 - [ ] build system
 - [ ] package manager
     - [ ] use Maven ?
 - [ ] dependency manager
 - [ ] document system
      - [ ] document system

### Syntax Design

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


### Workflow [low priority]

 - [ ] CI
    - [ ] replace Solang CI container.

### Document

