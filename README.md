## regular_expression

This is a crate for parsing regular expressions and interacting with deterministic finite state automata (DFAs).

## Syntax

This library accepts regular expressions of the following form over a user-defined alphabet $\Sigma$.
The syntax is given in a language based on [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form) with the addition of set operations necessary to describe characters in the context of an arbitrary user-defined alphabet.

```text
/* Expression types ordered by precidence
--------------------------------------------------------------------- */

Expression          ::= [Choice]

Choice              ::= [Concatenation] {CHOICE Concatenation}

Concatenation       ::= Closure {Closure}

Closure             ::= Atomic [CLOSURE]

Atomic              ::= CHAR
                        | LEFT_PRECEDENCE Expression RIGHT_PRECEDENCE

/* Atomics 
--------------------------------------------------------------------- */

CHAR                ::= σ ∈ Σ \ {
                                    CHOICE,
                                    CLOSURE,
                                    LEFT_PRECEDENCE,
                                    RIGHT_PRECEDENCE,
                                }
CHOICE              ::= "|"
CLOSURE             ::= "*"
LEFT_PRECEDENCE     ::= "("
RIGHT_PRECEDENCE    ::= ")"
```

## Semantics

The semantics of the regular expressions admitted by this crate are represented below in denotational semantics, mapping regular expressions to the set of strings they accept.
A statement of the form $\left[\\!\left[ E \right]\\!\right] = V$ denotes a mapping between the expression $E$ and the mathematical object $V$.

Aside from the common set operations, let us define $A \times B = \left\\{ab\\ \vert a \in A \wedge b \in B\right\\}$ for sets of strings $A, B$ where juxtaposition in $ab$ represents string concatenation.

Let $\epsilon$ represent the empty string.

---

Identities

$$\left[\\!\left[ \epsilon \right]\\!\right] = \left\\{\epsilon\right\\}$$

$$\left[\\!\left[ \sigma \right]\\!\right], \sigma \in \Sigma = \left\\{\sigma\right\\}$$

Operations

$$\left[\\!\left[ AB \right]\\!\right] = \left[\\!\left[ A \right]\\!\right] \times \left[\\!\left[ B \right]\\!\right]$$

$$\left[\\!\left[ A|B \right]\\!\right] = \left[\\!\left[ A \right]\\!\right] \cup \left[\\!\left[ B \right]\\!\right]$$

$$\left[\\!\left[ A^* \right]\\!\right] = \bigcup_{n \in \mathbf{N}} \left[\\!\left[ A \right]\\!\right]^n \cup \left\\{\epsilon\right\\}$$
