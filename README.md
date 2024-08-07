# rsLox

![workflow](https://github.com/LionsWrath/rsLox/actions/workflows/rust.yml/badge.svg)

Interpreter for the lox language written in Rust.

## Grammar:

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
error and needs to be correctly dealt by the parser. This is basically a calculator.

### Statements Grammar

```
program   → statement* EOF;
statement → exprStmt | printStmt;
exprStmt  → expression ";";
printStmt → "print" expression ";";

```

Additional grammar for statements. The `program` will now be te new beginning of the AST;

### Variable Declaration Grammar (Global Variable)

```
program   → declaration* EOF;
declaration → varDecl | statement;
varDecl  → "var" identifier ("=" expression)? ";";
statement → exprStmt | printStmt;
exprStmt  → expression ";";
printStmt → "print" expression ";";

```

Incremental changes for variable declaration;

## Variable Assignment Grammar

```
expression → assignment;
assignment → IDENTIFIER "=" assignment | equality
```

Beginning of variable assignment. Declaration is different for mutation of the variable.
It is possible to not allow mutability in variable, like a SSA mode.

## Grammar (24/07/2024)

```
program   → declaration* EOF;
declaration → varDecl | statement;
varDecl  → "var" identifier ("=" expression)? ";";
statement → exprStmt | printStmt;
exprStmt  → expression ";";
printStmt → "print" expression ";";

expression → assignment;
assignment → IDENTIFIER "=" assignment | ternary
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

## Scope (Lexical Scope)

Support for shadowing (like rust) and hierarchical environments.

### Block Syntax and semantics

```
statement → exprStmt | printStmt | block;
block     → "{" declaration* "}";
```

## Tests

TODO (will add tests for each part later)
