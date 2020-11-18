```
rustc +nightly -Zunpretty=mir main.rs >> main.mir
rustc +nightly -Zunpretty=hrir main.rs >> main.hir
```