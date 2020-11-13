
Origin: [Comparison of programming languages](https://en.wikipedia.org/wiki/Assignment_(computer_science))

 - Augmented assignment: `a = 2*a`, or `a *= 2`
 - Chained assignment: `a = b = c = d = f = 0 `
 - Parallel assignment: `a, b := 0, 1`


Parallel Assignment in function :

```python
def f():
    return 1, 2
a, b = f()
```

| format | Language |
|-|-|
| variable = expression | Fortran, PL/I, C (and descendants such as C++, Java, etc.), Bourne shell, Python, Go (assignment to pre-declared variables), R, PowerShell, etc. |
| variable := expression  |  ALGOL (and derivatives), Simula, CPL, BCPL, Pascal[23] (and descendants such as Modula), Mary, PL/M, Ada, Smalltalk, Eiffel,[24][25] Oberon, Dylan,[26] Seed7, Python (an assignment expression),[27] Go (shorthand for declaring and defining a variable),[28] Io, AMPL, ML,[29] AutoHotkey etc. |

Other possibilities include a left arrow or a keyword, though there are other, rarer, variants:

| `_variable_ << _expression_` | [Magik](https://en.wikipedia.org/wiki/Magik_(programming_language) "Magik (programming language)") |
| `_variable_ <- _expression_` | [F#](https://en.wikipedia.org/wiki/F_Sharp_(programming_language) "F Sharp (programming language)"), [OCaml](https://en.wikipedia.org/wiki/OCaml "OCaml"), [R](https://en.wikipedia.org/wiki/R_(programming_language) "R (programming language)"), [S](https://en.wikipedia.org/wiki/S_(programming_language) "S (programming language)") |
| `_variable_ <<- _expression_` | [R](https://en.wikipedia.org/wiki/R_(programming_language) "R (programming language)") |
| `assign("_variable_", _expression_)` | [R](https://en.wikipedia.org/wiki/R_(programming_language) "R (programming language)") |
| `_variable_ ← _expression_` | [APL](https://en.wikipedia.org/wiki/APL_(programming_language) "APL (programming language)"),<sup id="cite_ref-aplbook_31-0" class="reference">[[30]](https://en.wikipedia.org/wiki/Assignment_(computer_science)#cite_note-aplbook-31)</sup> [Smalltalk](https://en.wikipedia.org/wiki/Smalltalk "Smalltalk"), [BASIC Programming](https://en.wikipedia.org/wiki/BASIC_Programming "BASIC Programming") |
| `_variable_ =: _expression_` | [J](https://en.wikipedia.org/wiki/J_(programming_language) "J (programming language)") |
| `LET _variable_ = _expression_` | [BASIC](https://en.wikipedia.org/wiki/BASIC "BASIC") |
| `let _variable_ := _expression_` | [XQuery](https://en.wikipedia.org/wiki/XQuery "XQuery") |
| `set _variable_ to _expression_` | [AppleScript](https://en.wikipedia.org/wiki/AppleScript "AppleScript") |
| `set _variable_ = _expression_` | [C shell](https://en.wikipedia.org/wiki/C_shell "C shell") |
| `Set-Variable _variable_ _(expression)_` | [PowerShell](https://en.wikipedia.org/wiki/PowerShell "PowerShell") |
| `_variable_ : _expression_` | [Macsyma, Maxima](https://en.wikipedia.org/wiki/Macsyma "Macsyma"), [Rebol](https://en.wikipedia.org/wiki/Rebol "Rebol"), [K](https://en.wikipedia.org/wiki/K_(programming_language) "K (programming language)") |
| `var _variable_ _expression_` | [mIRC scripting language](https://en.wikipedia.org/wiki/MIRC_scripting_language "MIRC scripting language") |
| `_reference-variable_ :- _reference-expression_` | [Simula](https://en.wikipedia.org/wiki/Simula "Simula") |

Mathematical [pseudo code](https://en.wikipedia.org/wiki/Pseudocode#Common_mathematical_symbols "Pseudocode") assignments are generally depicted with a left-arrow.

Some platforms put the expression on the left and the variable on the right:

| `MOVE _expression_ TO _variable_` | [COBOL](https://en.wikipedia.org/wiki/COBOL "COBOL") |
| `_expression_ → _variable_` | [TI-BASIC](https://en.wikipedia.org/wiki/TI-BASIC "TI-BASIC"), [Casio](https://en.wikipedia.org/wiki/Casio_graphic_calculators "Casio graphic calculators") BASIC |
| `_expression_ -> _variable_` | [POP-2](https://en.wikipedia.org/wiki/POP-2 "POP-2"), [BETA](https://en.wikipedia.org/wiki/BETA_(programming_language) "BETA (programming language)"), [R](https://en.wikipedia.org/wiki/R_(programming_language) "R (programming language)") |
| `put _expression_ into _variable_` | [LiveCode](https://en.wikipedia.org/wiki/LiveCode "LiveCode") |

Some expression-oriented languages, such as [Lisp](https://en.wikipedia.org/wiki/Lisp_(programming_language) "Lisp (programming language)")<sup id="cite_ref-clisp_32-0" class="reference">[[31]](https://en.wikipedia.org/wiki/Assignment_(computer_science)#cite_note-clisp-32)</sup><sup id="cite_ref-cmlisp_33-0" class="reference">[[32]](https://en.wikipedia.org/wiki/Assignment_(computer_science)#cite_note-cmlisp-33)</sup> and Tcl, uniformly use prefix (or postfix) syntax for all statements, including assignment.

| `(setf _variable_ _expression_)` | [Common Lisp](https://en.wikipedia.org/wiki/Common_Lisp "Common Lisp") |
| `(set! _variable_ _expression_)` | [Scheme](https://en.wikipedia.org/wiki/Scheme_(programming_language) "Scheme (programming language)")<sup id="cite_ref-scheme_34-0" class="reference">[[33]](https://en.wikipedia.org/wiki/Assignment_(computer_science)#cite_note-scheme-34)</sup><sup id="cite_ref-schemeint_35-0" class="reference">[[34]](https://en.wikipedia.org/wiki/Assignment_(computer_science)#cite_note-schemeint-35)</sup><sup id="cite_ref-sussman_36-0" class="reference">[[35]](https://en.wikipedia.org/wiki/Assignment_(computer_science)#cite_note-sussman-36)</sup> |
| `set _variable_ _expression_` | [Tcl](https://en.wikipedia.org/wiki/Tcl "Tcl") |
| `_expression_ _variable_ !` | [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language) "Forth (programming language)") |