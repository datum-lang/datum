output hir & mir

```
rustc +nightly -Zunpretty=mir main.rs >> main.mir
rustc +nightly -Zunpretty=hir main.rs >> main.hir
```

output HIR tree

```
rustc +nightly -Zunpretty=hir-tree main.rs >> main.hir-tree
```
