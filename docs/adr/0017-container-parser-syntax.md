# 17. Container Parser Syntax

Date: 2020-12-16

## Status

2020-12-16 proposed

## Context

In current design, we need to:

1. builtin DSL support for Charj.
2. implementation a Typography syntax with lalrpop.

It means we can find some way to combine their' syntax. For examples,

```rust
ast {
  node parameters {
    parameters parameter*;
  }
}
```

can like things in Rust

```rust
struct AST {
  nodes: Vec<node>
}

enum node {
  parameters(Vec<parameter>)
}

struct parameter {

}
```

in top can be like `struct` or `class`, and some method in class. `ast{}` will be equal `struct ast{}`

In such things, we can call it `container`.a container can be `struct` or `enum` in.

## Decision

Decision here...

## Consequences

Consequences here...
