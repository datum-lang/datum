Some of the key characteristics of MIR are:

- It is based on a control-flow graph.
- It does not have nested expressions.
- All types in MIR are fully explicit.

Key MIR vocabulary

 - Basic blocks: units of the control-flow graph, consisting of:
    - statements: actions with one successor
    - terminators: actions with potentially multiple successors; always at the end of a block

## Python Samples

```rust
LoadName { name: "print", scope: Free },
LoadConst { value: String { value: "hello,world" } },
CallFunction { typ: Positional(1) },

Pop,
LoadConst { value: None },
ReturnValue
```


## Android Smali Samples

[Smali ZH](https://ctf-wiki.github.io/ctf-wiki/android/basic_operating_mechanism/java_layer/smali/smali-zh/)

### Fields

```smali
#instance fields
.field <访问权限修饰符> [非权限修饰符] <字段名>:<字段类型>
```

### Method

```smali
# 描述方法类型
.method <访问权限修饰符> [修饰符] <方法原型>
      <.locals>
      [.parameter]
      [.prologue]
      [.line]
      <代码逻辑>
      [.line]
      <代码逻辑>
.end
```

### Class

```smali
.class <访问权限修饰符> [非权限修饰符] <类名>
.super <父类名>
.source <源文件名称>
```

### Annotations

```smali
#annotations
.annotation [注解的属性] <注解范围>
    [注解字段=值]
    ...
.end
```
