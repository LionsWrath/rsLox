# rsLox

![workflow](https://github.com/LionsWrath/rsLox/actions/workflows/rust.yml/badge.svg)

JVM Interpreter for the lox language written in Rust.

## Current Grammar:

```
expression → ternary
ternary    → comma ( "?" expression ":" ternary )?;
comma      → equality ( "," equality )*;
equality   → comparison ( ( "!=" | "==" ) comparison )* ;
comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term       → factor ( ( "-" | "+" ) factor )* ;
factor     → unary ( ( "/" | "*" ) unary )* ;
unary      → ( "!" | "-" ) unary
           | primary ;
primary    → NUMBER | STRING | "true" | "false" | "nil"
           | "(" expression ")"
           // Error productions - No Left-Operand on operation
           | ( "!=" | "==" ) equality
           | ( ">" | ">=" | "<" | "<=" ) comparison
           | ( "+" ) term
           | ( "/" | "*" ) factor ;
```

This grammar has additional rules for syncronizing specific types of errors like
missing left-operands on operations. The grammar itself is not aware that it is an
error and needs to be correctly dealt by the parser.
