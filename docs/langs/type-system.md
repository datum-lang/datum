Origins: [Comparison of programming languages by type system](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system)

Brief definitions

*   A [nominal type system](https://en.wikipedia.org/wiki/Nominal_type_system "Nominal type system") means that the language decides whether types are compatible and/or equivalent based on explicit declarations and names.
*   A [structural type system](https://en.wikipedia.org/wiki/Structural_type_system "Structural type system") means that the language decides whether types are compatible and/or equivalent based on the definition and characteristics of the types.
*   _Type checking_ determines whether and when types are verified. Static checking means that type errors are reported based on a program's text (source code). Dynamic checking means that type errors are reported based on a program's dynamic (run-time) behavior.

| Language | [Type safety](https://en.wikipedia.org/wiki/Type_safety "Type safety") | [Type expression](https://en.wikipedia.org/wiki/Type_system#Explicit_or_implicit_declaration_and_inference "Type system") | [Type compatibility and equivalence](https://en.wikipedia.org/wiki/Type_equivalence "Type equivalence") | [Type checking](https://en.wikipedia.org/wiki/Type_checking "Type checking") |
| --- | --- | --- | --- | --- |
| [ActionScript](https://en.wikipedia.org/wiki/ActionScript "ActionScript") 3.0 | strong | implicit with optional explicit typing |   | static |
| [Ada](https://en.wikipedia.org/wiki/Ada_(programming_language) "Ada (programming language)") | strong<sup id="cite_ref-1" class="reference">[[TS 1]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-1)</sup> | explicit | nominal | static |
| [Aldor](https://en.wikipedia.org/wiki/Aldor "Aldor") | weak | implicit |   | static |
| [ALGOL 58](https://en.wikipedia.org/wiki/ALGOL_58 "ALGOL 58") | strong | explicit |   | static |
| [ALGOL 60](https://en.wikipedia.org/wiki/ALGOL_60 "ALGOL 60") | strong | explicit |   | static |
| [ALGOL 68](https://en.wikipedia.org/wiki/ALGOL_68 "ALGOL 68") | strong | explicit | structural | static & [tagged unions](https://en.wikipedia.org/wiki/Tagged_union "Tagged union") |
| [APL](https://en.wikipedia.org/wiki/APL_(programming_language) "APL (programming language)") | strong |   |   | dynamic |
| [AutoHotkey](https://en.wikipedia.org/wiki/AutoHotkey "AutoHotkey") | typeless | n/a | n/a | n/a |
| [Ateji PX](https://en.wikipedia.org/wiki/Ateji_PX "Ateji PX") | strong | explicit | nominal | static |
| [Bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell) "Bash (Unix shell)") | _**?**_ | _**?**_ | _**?**_ | _**?**_ |
| [BASIC](https://en.wikipedia.org/wiki/BASIC "BASIC") | strong | explicit | nominal | static |
| [BLISS](https://en.wikipedia.org/wiki/BLISS "BLISS") | typeless | n/a | n/a | n/a |
| [BeanShell](https://en.wikipedia.org/wiki/BeanShell "BeanShell") | strong |   | nominal | dynamic |
| [Boo](https://en.wikipedia.org/wiki/Boo_(programming_language) "Boo (programming language)") | strong | implicit with optional explicit typing |   | static with optional dynamic typing |
| [Bro](https://en.wikipedia.org/wiki/Bro_(software) "Bro (software)") | strong | implicit with optional explicit typing | nominal | static |
| [C](https://en.wikipedia.org/wiki/C_(programming_language) "C (programming language)") | weak | explicit | nominal | static |
| [C++](https://en.wikipedia.org/wiki/C%2B%2B "C++") ([ISO/IEC 14882](https://en.wikipedia.org/wiki/ISO/IEC_14882 "ISO/IEC 14882")) | weak | explicit with optional implicit typing (by using auto in C++11) | nominal | static<sup id="cite_ref-2" class="reference">[[TS 2]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-2)</sup> |
| [C#](https://en.wikipedia.org/wiki/C_Sharp_(programming_language) "C Sharp (programming language)") | weak<sup id="cite_ref-r2_3-0" class="reference">[[TS 3]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-r2-3)</sup> | implicit with optional explicit typing | nominal | static<sup id="cite_ref-4" class="reference">[[TS 4]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-4)</sup> |
| [Clean](https://en.wikipedia.org/wiki/Clean_(programming_language) "Clean (programming language)") | strong | implicit |   | static |
| [Clojure](https://en.wikipedia.org/wiki/Clojure "Clojure") | strong | implicit with optional explicit typing |   | dynamic |
| [COBOL](https://en.wikipedia.org/wiki/COBOL "COBOL") | strong | explicit | nominal | static |
| [ColdFusion](https://en.wikipedia.org/wiki/ColdFusion_Markup_Language "ColdFusion Markup Language") (CFML) | strong | implicit |   | dynamic |
| [Common Lisp](https://en.wikipedia.org/wiki/Common_Lisp "Common Lisp") | strong | implicit with optional explicit typing | structural for implicit typing, nominal for explicit typing | dynamic, some static checking(depending on implementation) |
| [Curl](https://en.wikipedia.org/wiki/Curl_(programming_language) "Curl (programming language)") | strong |   | nominal |   |
| [Cython](https://en.wikipedia.org/wiki/Cython "Cython") | strong | implicit with optional explicit typing | nominal (extension types) and structural (Python) | dynamic with optional static typing |
| [D](https://en.wikipedia.org/wiki/D_(programming_language) "D (programming language)") | weak<sup id="cite_ref-r2_3-1" class="reference">[[TS 3]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-r2-3)</sup> | explicit | nominal | static |
| [Dylan](https://en.wikipedia.org/wiki/Dylan_(programming_language) "Dylan (programming language)") | strong |   |   | dynamic |
| [Eiffel](https://en.wikipedia.org/wiki/Eiffel_(programming_language) "Eiffel (programming language)") | strong |   | nominal | static |
| [Elixir](https://en.wikipedia.org/wiki/Elixir_(programming_language) "Elixir (programming language)") | strong | implicit |   | dynamic |
| [Erlang](https://en.wikipedia.org/wiki/Erlang_(programming_language) "Erlang (programming language)") | strong | implicit |   | dynamic |
| [Euphoria](https://en.wikipedia.org/wiki/Euphoria_(programming_language) "Euphoria (programming language)") | strong | explicit, implicit with objects | nominal | static, dynamic with objects |
| [F#](https://en.wikipedia.org/wiki/F_Sharp_(programming_language) "F Sharp (programming language)") | strong | implicit | nominal | static |
| [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language) "Forth (programming language)") | typeless | n/a | n/a | n/a |
| [Fortran](https://en.wikipedia.org/wiki/Fortran "Fortran") | strong | explicit<sup id="cite_ref-5" class="reference">[[TS 5]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-5)</sup> | nominal | static |
| [Gambas](https://en.wikipedia.org/wiki/Gambas "Gambas") | strong | explicit | nominal |   |
| [GLBasic](https://en.wikipedia.org/wiki/GLBasic "GLBasic") | strong | explicit. Non-explicit declarations available through project options | nominal | static |
| [Go](https://en.wikipedia.org/wiki/Go_(programming_language) "Go (programming language)")<sup id="cite_ref-6" class="reference">[[1]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-6)</sup> | strong | implicit with optional explicit typing | structural | static |
| [Gosu](https://en.wikipedia.org/wiki/Gosu_(programming_language) "Gosu (programming language)") | strong | partially implicit (local type inference) | nominal (subclassing) and structural | static |
| [Groovy](https://en.wikipedia.org/wiki/Groovy_(programming_language) "Groovy (programming language)") | strong | implicit with optional explicit typing |   | dynamic with optional static typing |
| [Harbour](https://en.wikipedia.org/wiki/Harbour_(programming_language) "Harbour (programming language)") | strong | implicit with optional explicit typing |   | dynamic |
| [Haskell](https://en.wikipedia.org/wiki/Haskell_(programming_language) "Haskell (programming language)") | strong | implicit with optional explicit typing | nominal<sup id="cite_ref-7" class="reference">[[2]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-7)</sup><sup id="cite_ref-:0_8-0" class="reference">[[3]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-:0-8)</sup> | static |
| [Haxe](https://en.wikipedia.org/wiki/Haxe "Haxe") | strong | implicit with optional explicit typing | nominal (subclassing) and structural | static with optional dynamic typing |
| [Io](https://en.wikipedia.org/wiki/Io_(programming_language) "Io (programming language)") | strong | implicit |   | dynamic |
| [ISLISP](https://en.wikipedia.org/wiki/ISLISP "ISLISP") | strong |   |   | dynamic |
| [J](https://en.wikipedia.org/wiki/J_(programming_language) "J (programming language)") | strong |   |   | dynamic |
| [Java](https://en.wikipedia.org/wiki/Java_(programming_language) "Java (programming language)") | strong<sup id="cite_ref-9" class="reference">[[4]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-9)</sup> | explicit | nominal | static |
| [JavaScript](https://en.wikipedia.org/wiki/JavaScript "JavaScript") | weak | implicit | n/a | dynamic |
| [Julia](https://en.wikipedia.org/wiki/Julia_(programming_language) "Julia (programming language)") | strong | implicit with optional explicit typing<sup id="cite_ref-10" class="reference">[[5]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-10)</sup> | structural for implicit typing, nominal for explicit typing | dynamic |
| [Joy](https://en.wikipedia.org/wiki/Joy_(programming_language) "Joy (programming language)") | strong |   |   | dynamic |
| [Kotlin](https://en.wikipedia.org/wiki/Kotlin_(programming_language) "Kotlin (programming language)") | strong | partially implicit (local type inference) | nominal | static |
| [LabVIEW](https://en.wikipedia.org/wiki/LabVIEW "LabVIEW") | strong |   |   |   |
| [Lua](https://en.wikipedia.org/wiki/Lua_(programming_language) "Lua (programming language)") | strong | implicit |   | dynamic |
| [Maple](https://en.wikipedia.org/wiki/Maple_(programming_language) "Maple (programming language)") | strong |   |   | dynamic |
| [Mathematica](https://en.wikipedia.org/wiki/Mathematica "Mathematica") | strong |   |   | dynamic |
| [MATLAB](https://en.wikipedia.org/wiki/MATLAB "MATLAB") M-code | strong |   |   | dynamic |
| [Modula-2](https://en.wikipedia.org/wiki/Modula-2 "Modula-2") | weak<sup id="cite_ref-r2_3-2" class="reference">[[TS 3]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-r2-3)</sup> | explicit | nominal | static |
| [Modula-3](https://en.wikipedia.org/wiki/Modula-3 "Modula-3") | weak<sup id="cite_ref-r2_3-3" class="reference">[[TS 3]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-r2-3)</sup> | explicit | structural | static |
| [MUMPS](https://en.wikipedia.org/wiki/MUMPS "MUMPS") (M) | typeless | n/a | n/a | n/a |
| [Oberon](https://en.wikipedia.org/wiki/Oberon_(programming_language) "Oberon (programming language)") | strong | explicit | nominal | static and partially dynamic<sup id="cite_ref-11" class="reference">[[TS 6]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-11)</sup> |
| [Objective-C](https://en.wikipedia.org/wiki/Objective-C "Objective-C") | strong | explicit | nominal | dynamic with optional static typing<sup id="cite_ref-12" class="reference">[[6]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-12)</sup> |
| [OCaml](https://en.wikipedia.org/wiki/OCaml "OCaml") | strong | implicit with optional explicit typing | nominal for records,<sup id="cite_ref-13" class="reference">[[7]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-13)</sup> structural for objects<sup id="cite_ref-:0_8-1" class="reference">[[3]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-:0-8)</sup><sup id="cite_ref-14" class="reference">[[8]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-14)</sup> | static |
| [Object Pascal](https://en.wikipedia.org/wiki/Object_Pascal "Object Pascal") | strong | explicit | nominal | static |
| [Opa](https://en.wikipedia.org/wiki/Opa_(programming_language) "Opa (programming language)") | strong | implicit with optional explicit typing | structural | static |
| [Oxygene](https://en.wikipedia.org/wiki/Oxygene_(programming_language) "Oxygene (programming language)") | weak | implicit |   | static |
| [Oz-Mozart](https://en.wikipedia.org/wiki/Oz_(programming_language) "Oz (programming language)") | strong | implicit | structural | dynamic |
| [Pascal](https://en.wikipedia.org/wiki/Pascal_(programming_language) "Pascal (programming language)") | weak<sup id="cite_ref-r2_3-4" class="reference">[[TS 3]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-r2-3)</sup> | explicit | nominal | static |
| [Perl](https://en.wikipedia.org/wiki/Perl "Perl") 5 |   | implicit |   | dynamic |
| [PHP](https://en.wikipedia.org/wiki/PHP "PHP") |   | implicit with optional explicit typing | nominal | dynamic |
| [Plus](https://en.wikipedia.org/wiki/Plus_(programming_language) "Plus (programming language)") | strong | explicit | structural | static, dynamic (optional) |
| [Prolog](https://en.wikipedia.org/wiki/Prolog "Prolog") |   |   |   | dynamic |
| [Pure](https://en.wikipedia.org/wiki/Pure_(programming_language) "Pure (programming language)") |   |   |   | dynamic |
| [Python](https://en.wikipedia.org/wiki/Python_(programming_language) "Python (programming language)") | strong | implicit (with optional explicit typing as of 3.5) | n/a | dynamic |
| [Raku](https://en.wikipedia.org/wiki/Raku_(programming_language) "Raku (programming language)") |   | partially implicit<sup id="cite_ref-15" class="reference">[[TS 7]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-15)</sup> |   | dynamic with optional static typing |
| [REBOL](https://en.wikipedia.org/wiki/REBOL "REBOL") | strong | implicit |   | dynamic |
| [Rexx](https://en.wikipedia.org/wiki/Rexx "Rexx") | typeless | n/a, implicit wrt numbers | n/a | static+dynamic wrt numbers |
| [RPG](https://en.wikipedia.org/wiki/IBM_RPG "IBM RPG") | weak |   |   | static |
| [Ruby](https://en.wikipedia.org/wiki/Ruby_(programming_language) "Ruby (programming language)") | strong | implicit | n/a | dynamic |
| [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language) "Rust (programming language)") | strong | explicit with optional implicit typing<sup id="cite_ref-16" class="reference">[[9]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-16)</sup> | mostly nominal | static |
| [S](https://en.wikipedia.org/wiki/S_(programming_language) "S (programming language)") |   |   |   | dynamic |
| [S-Lang](https://en.wikipedia.org/wiki/S-Lang_(programming_library) "S-Lang (programming library)") | strong | implicit |   | dynamic |
| [Scala](https://en.wikipedia.org/wiki/Scala_(programming_language) "Scala (programming language)") | strong | partially implicit (local type inference) | nominal (subclassing) and structural | static |
| [Scheme](https://en.wikipedia.org/wiki/Scheme_(programming_language) "Scheme (programming language)") | strong | implicit |   | dynamic ([latent](https://en.wikipedia.org/wiki/Latent_typing "Latent typing")) |
| [Seed7](https://en.wikipedia.org/wiki/Seed7 "Seed7") | strong | explicit | nominal | static |
| [Simula](https://en.wikipedia.org/wiki/Simula "Simula") | strong |   |   | static<sup id="cite_ref-17" class="reference">[[TS 8]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-17)</sup> |
| [Smalltalk](https://en.wikipedia.org/wiki/Smalltalk "Smalltalk") | strong | implicit |   | dynamic |
| [Swift](https://en.wikipedia.org/wiki/Swift_(programming_language) "Swift (programming language)") | strong | partially implicit (local type inference) | nominal (subclassing) and structural | static |
| [Standard ML](https://en.wikipedia.org/wiki/Standard_ML "Standard ML") | strong | implicit with optional explicit typing | structural | static |
| [Tcl](https://en.wikipedia.org/wiki/Tcl "Tcl") |   |   |   | dynamic |
| [TypeScript](https://en.wikipedia.org/wiki/TypeScript "TypeScript") | ? | optional | structural | static |
| [Visual Basic](https://en.wikipedia.org/wiki/Visual_Basic "Visual Basic") | strong | implicit with optional explicit typing | nominal | static |
| [Visual Basic .NET](https://en.wikipedia.org/wiki/Visual_Basic_.NET "Visual Basic .NET") | weak<sup id="cite_ref-r2_3-5" class="reference">[[TS 3]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-r2-3)</sup> | explicit |   | static |
| [Visual Prolog](https://en.wikipedia.org/wiki/Visual_Prolog "Visual Prolog") | strong | partially implicit | nominal | static |
| [Wolfram Language](https://en.wikipedia.org/wiki/Wolfram_Language "Wolfram Language") | strong |   |   | dynamic |
| [Windows PowerShell](https://en.wikipedia.org/wiki/Windows_PowerShell "Windows PowerShell") | strong | implicit |   | dynamic |
| [XL](https://en.wikipedia.org/wiki/XL_(programming_language) "XL (programming language)") | strong |   | nominal | static |
| [Xojo](https://en.wikipedia.org/wiki/Xojo "Xojo") | strong | explicit | nominal | static |
| [XPath](https://en.wikipedia.org/wiki/XPath "XPath")/[XQuery](https://en.wikipedia.org/wiki/XQuery "XQuery") | strong | partially implicit | nominal | dynamic with optional static typing |
| [Dart](https://en.wikipedia.org/wiki/Dart_(programming_language) "Dart (programming language)") | strong<sup id="cite_ref-18" class="reference">[[10]](https://en.wikipedia.org/wiki/Comparison_of_programming_languages_by_type_system#cite_note-18)</sup> | gradual typing | nominal | static with optional dynamic typing |

