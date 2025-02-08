```
program        → declaration* EOF ;

declaration    → varDecl
               | statement ;

statement      → exprStmt
               | ifStmt
               | printStmt
               | whileStmt
               | loopStmt
               | block ;

whileStmt      → "while" "(" expression ")" statement ;
               ( "else" statement )? ;

ifStmt         → "if" "(" expression ")" statement
               ( "else" statement )? ;

loopStmt       → "loop" statement

block          → "{" declaration* "}" ;

exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;

varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;

expression     → assignment ;
assignment     → IDENTIFIER "=" assignment
               | ternary ;
               | logic_or ;

logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;

ternary        → comma ("?" expression ":" ternary)?;
comma          → equality ( "," equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;

primary        → "true" | "false" | "nil"
               | NUMBER | STRING
               | "(" expression ")"
               | IDENTIFIER ;

```

| Grammar notation | Code representation               |
| ---------------- | --------------------------------- |
| Terminal         | Code to match and consume a token |
| Nonterminal      | Call to that rule’s function     |
| `\|`            | `if` or `switch` statement    |
| `*` or `+`   | `while` or `for` loop         |
| `?`            | `if` statement                  |
