# 18. use semicolon for split context

Date: 2020-12-22

## Status

2020-12-22 proposed

## Context

In current design with LALRPOP, a expression like `!function()` with be cause such issues.

```bash
 "!" PrimaryExpr
At that point, if the next token is a `"("`, then the parser can proceed in two different ways.

First, the parser could execute the production at /Users/fdhuang/charj/charj/dc_parser/src/charj.lalrpop:334:4: 334:14, which would consume the top 1 token(s) from the stack and produce a `NotExpression`. This might then yield a parse tree like
  PrimaryExpr            ╷ Statement
  ├─NotExpression────────┤         │
  ├─Term─────────────────┤         │
  ├─ArithmeticExpression─┤         │
  ├─ShiftExpression──────┤         │
  ├─CompareExpression────┤         │
  ├─AndExpression────────┤         │
  ├─OrExpression─────────┤         │
  ├─RangeExpression──────┤         │
  ├─UnaryExpr────────────┤         │
  ├─Expression───────────┤         │
  ├─ExpressionStatement──┤         │
  ├─CompoundStatement────┤         │
  ├─Statement────────────┤         │
  ├─Statement+───────────┘         │
  └─Statement+─────────────────────┘

Alternatively, the parser could shift the `"("` token and later use it to construct a `FunctionCall`. This might then yield a parse tree like
  "!" PrimaryExpr "(" Comma<Argument> ")"
  │   ├─FunctionCall────────────────────┤
  │   ├─PrimaryExpr─────────────────────┤
  │   └─NotExpression───────────────────┤
  └─NotExpression───────────────────────┘
```

## Decision

use semicolon for split keywords before we write by ourself.

## Consequences

Consequences here...
