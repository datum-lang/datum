# Charj

![Charj Build](https://github.com/charj-lang/charj-poc/workflows/Charj%20Build/badge.svg)

> A easy maintain(read/write) language for transform **from**/**to** other languages.

Design for:

 - legacy system migration.
 - multiple-targets compiled languages. (by LLVM)
 - quick pseudocode.
 - simple DSL design. (TBD)
     - domain model design for languages.
     - visualize architecture.

examples:

 - JavaScript/Java to WASM

TBD:

 - C++ to Charj. (partially support)

## Development

see in [DEVELOPMENT.md](DEVELOPMENT.md)

Roadmap：

 - [ ] implementation basic compiler logic
     - [x] hir convert (neat)
        - [ ] optimized
            - [ ] parameter value to vars
     - [ ] mir convert (medium)
     - [ ] to LLVM     (codegen)
 - [ ] implementation grammars

## Roadmap

see in [ROADMAP.md](ROADMAP.md)

## License

lexer based on & inspired by [solang](https://github.com/hyperledger-labs/solang) & [RustPython](https://github.com/RustPython/RustPython)

This code is distributed under the MIT license. See `LICENSE` in this directory.
