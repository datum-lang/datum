## More is More

> Less syntax sugar will make languages easy to understand and convert.

Charj's IR was defined close to common IR, so we need to define almost all details.

a basic `hello, world`.

```charj
pkg examples

default$main() {
    println("hello,world")
}
```

if we change to this will be better:

```
pkg examples

import fmt;

default$main() {
    fmt.println("hello,world")
}
```

HIR will be

```charj
pkg examples

import fmt;

default$main() {
  let text: string = "hello,world";
  fmt.println(text);
}
```