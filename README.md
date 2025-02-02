## regular_expression

This is a crate for parsing regular expressions and interacting with deterministic finite state automata (DFAs).

## Syntax

This library accepts regular expressions of the following form over a user-defined alphabet Σ.
The syntax is given in a language based on [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form) with the addition of set operations necessary to describe characters in the context of an arbitrary user-defined alphabet.

```text
Expression          ::= EMPTY_STRING
                        | String
                        | LEFT_PRECEDENCE Expression RIGHT_PRECEDENCE
                        | Expression CLOSURE
                        | Expression CHOICE Expression

String              ::= CHAR
                        | CHAR String

CHAR                ::= σ ∈ Σ \ {
                                    CHOICE,
                                    CLOSURE,
                                    LEFT_PRECEDENCE,
                                    RIGHT_PRECEDENCE,
                                    EMPTY_STRING.
                                }
CHOICE              ::= "|"
CLOSURE             ::= "*"
LEFT_PRECEDENCE     ::= "("
RIGHT_PRECEDENCE    ::= ")"
EMPTY_STRING        ::=  "\e"
```
