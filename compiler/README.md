# Charj compiler: Zao

## Workflow

 - Neat mod for transform AST -> HIR
 - Medium mod for transform HIR -> MIR (TBD)
 - Lowerify mod for transform MIR -> LLVM IR

## Compiler

in Rust design:

```rust
pub struct Compiler {
    pub(crate) input: Input,
    pub(crate) input_path: Option<PathBuf>,
    pub(crate) output_dir: Option<PathBuf>,
    pub(crate) output_file: Option<PathBuf>,
    pub(crate) crate_name: Option<String>,
}
```

We should name our compiler in Chinese way, so use some like `Zao` will be more easily to think.

## Hello, World

C version:

```c
#include <stdio.h>

int main() {
  printf("Hello World!\n");
}
```

LLVM version:


```ll
; Copied directly from the documentation
; Declare the string constant as a global constant.
@.str = private unnamed_addr constant [13 x i8] c"hello world\0A\00"

; External declaration of the puts function
declare i32 @puts(i8* nocapture) nounwind

; Definition of main function
define i32 @main() { ; i32()*
    ; Convert [13 x i8]* to i8  *...
    %cast210 = getelementptr [13 x i8],[13 x i8]* @.str, i64 0, i64 0

    ; Call puts function to write out the string to stdout.
    call i32 @puts(i8* %cast210)
    ret i32 0
}

; Named metadata
!0 = !{i32 42, null, !"string"}
!foo = !{!0}
```

