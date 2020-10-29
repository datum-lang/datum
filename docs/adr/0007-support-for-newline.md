# 7. support for newline

Date: 2020-10-29

## Status

2020-10-29 proposed

## Context

[lalrpop](https://github.com/lalrpop/lalrpop) don't have a correct position for code.7

In different language have different implementation

1. RustPython use `'\n'` for new line
2. solang
3. gluon use [codespan](https://github.com/brendanzab/codespan)

## Decision

use lexer with `\n`

## Consequences

Consequences here...
