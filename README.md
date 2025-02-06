## regular_expression

This is a crate for parsing regular expressions and interacting with deterministic finite state automata (DFAs).

## Syntax

This library accepts regular expressions of the following form over a user-defined alphabet $\Sigma$.
The syntax is given in a language based on [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form) with the addition of set operations necessary to describe characters in the context of an arbitrary user-defined alphabet.

```text
/* Expression types ordered by precedence
--------------------------------------------------------------------- */

Expression          ::= [Choice]

Choice              ::= [Concatenation] {CHOICE Concatenation}

Concatenation       ::= Closure {Closure}

Closure             ::= Atomic [CLOSURE_TYPE]

Atomic              ::= CHAR_TYPE
                        | LEFT_PRECEDENCE Expression RIGHT_PRECEDENCE

/* Union Types
--------------------------------------------------------------------- */

CLOSURE_TYPE        ::= CLOSURE
                        | AND_CLOSURE
                        | CHOICE_CLOSURE

CHAR_TYPE           ::= CHAR
                        | CHOICE_ALPHABET
                        | INVERSE_CHOICE_ALPHABET CHAR

/* Atomics 
--------------------------------------------------------------------- */

CHAR                ::= σ ∈ ALPHABET
CHOICE              ::= "|"
CLOSURE             ::= "*"
LEFT_PRECEDENCE     ::= "("
RIGHT_PRECEDENCE    ::= ")"

/* Sugar Atomics
--------------------------------------------------------------------- */

AND_CLOSURE             ::= "+"
CHOICE_CLOSURE          ::= "?"

CHOICE_ALPHABET         ::= "."
INVERSE_CHOICE_ALPHABET ::= "^"

/* Alphabet
--------------------------------------------------------------------- */

ALPHABET            ::= Σ \ {
                                CHOICE,
                                CLOSURE,
                                LEFT_PRECEDENCE,
                                RIGHT_PRECEDENCE,
                                AND_CLOSURE,
                                CHOICE_CLOSURE,
                                CHOICE_ALPHABET,
                                INVERSE_CHOICE_ALPHABET,
                            }

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

Core Operations

$$\left[\\!\left[ AB \right]\\!\right] = \left[\\!\left[ A \right]\\!\right] \times \left[\\!\left[ B \right]\\!\right]$$

$$\left[\\!\left[ A|B \right]\\!\right] = \left[\\!\left[ A \right]\\!\right] \cup \left[\\!\left[ B \right]\\!\right]$$

$$\left[\\!\left[ A^* \right]\\!\right] = \bigcup_{n \in \mathbf{N}} \left[\\!\left[ A \right]\\!\right]^n \cup \left\\{\epsilon\right\\}$$

Sugars

$$\left[\\!\left[ A+ \right]\\!\right] = \left[\\!\left[ AA^* \right]\\!\right]$$

$$\left[\\!\left[ A? \right]\\!\right] = \left[\\!\left[ \epsilon \right]\\!\right] \cup \left[\\!\left[ A \right]\\!\right]$$

$$\left[\\!\left[ . \right]\\!\right] = \bigcup_{\sigma \in \Sigma} \left[\\!\left[ \sigma \right]\\!\right]$$

$$\left[\\!\left[ \\;\hat{}\\,\sigma \right]\\!\right], \sigma \in \Sigma = \left[\\!\left[ . \right]\\!\right] \setminus \left[\\!\left[ \sigma \right]\\!\right]$$
