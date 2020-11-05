# Charj

![Charj Build](https://github.com/charj-lang/charj-poc/workflows/Charj%20Build/badge.svg)

> a common language as a temp language for transform other languages.

Jetbrains' IDEA support: `Charj` [![Version](https://img.shields.io/jetbrains/plugin/v/15119-charj.svg)](https://plugins.jetbrains.com/plugin/15119-charj)

## Roadmap

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
    - [ ] hello, world
 - [ ] build system
 - [ ] dependency manager

Others todo:

 - [ ] CI
    - [ ] replace solang CI container.

## Development

### Setup LLVM

```bash
cargo install llvmenv
```

usage

```
llvmenv init
llvmenv entries
llvmenv build-entry 10.0.0
```

or try download and build in local

```
llvmenv build-entry local-llvm
```

then export:

```
LLVM_SYS_100_PREFIX=$HOME/llvm/llvm-10.0.1.src/build cargo build
LLVM_SYS_100_PREFIX=$HOME/llvm/llvm-10.0.1.src/build cargo run
```

## Decisions

 - [ ] Thinking in remove complex assign such as `-=`, `*=` or `--`. It can make things simple.

## License

lexer based on & inspired by [solang](https://github.com/hyperledger-labs/solang) & [RustPython](https://github.com/RustPython/RustPython)

[![Phodal's Idea](http://brand.phodal.com/shields/idea-small.svg)](http://ideas.phodal.com/)

@ 2020 A [Phodal Huang](https://www.phodal.com)'s [Idea](http://github.com/phodal/ideas). This code is distributed under the Apache license. See `LICENSE` in this directory.
