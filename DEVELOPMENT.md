
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

### Clion Config

add: `LLVM_SYS_100_PREFIX=/Users/fdhuang/llvm/llvm-10.0.1.src/build` to **Run/Debug Configurations**.
