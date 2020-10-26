# 1. newline or semicolon

Date: 2020-10-26

## Status

2020-10-26 proposed

## Context

In order to save packages or file, we need to compatible different languages import styles. Such like go

```go
import (
	"fmt"
	"reflect"
	"strconv"

	"github.com/antlr/antlr4/runtime/Go/antlr"
)
```

or rust

```rust
use std::iter::Peekable;
use std::str::CharIndices;

use phf::phf_map;
use unicode_xid::UnicodeXID;

use crate::error::LexicalError;
use crate::token::Token;
```

or Java like

```kotlin
import java.nio.charset.StandardCharsets.UTF_8
import java.nio.file.Files
import java.nio.file.Path
import javax.annotation.processing.Filer
import javax.tools.JavaFileObject
import javax.tools.JavaFileObject.Kind
import javax.tools.SimpleJavaFileObject
import javax.tools.StandardLocation
import kotlin.reflect.KClass
```

## Decision

If it's possible, then we need to build a FileSystem to replace '/' to '.' .

## Consequences

Consequences here...
