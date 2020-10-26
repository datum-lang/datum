# 3. standard library package format

Date: 2020-10-27

## Status

2020-10-27 proposed

## Context

Java use directory as package, it will cause fat package.

JavaScript don't have a useful package structure.

Rust use `mod.rs` or `lib.rs`, but it's hard to direct goto package

Golang almost use `bytes.go` for `bytes` package

So, we can try in a design

```
 - io
    - io.cj
    - *.cj
```

## Decision

Decision here...

## Consequences

Consequences here...
