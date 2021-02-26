# Charj compiler: Zao

## Workflow

 - Neat mod for transform AST -> HIR
 - Medium mod for transform HIR -> MIR (TBD)
 - Lowerify mod for transform MIR -> LLVM IR

## Target

### X86-X64

### WASM

### Bitcode

Run bitcode

```rust
lli main.cjc
```

