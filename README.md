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

## Operational Semantics

The following big step operational semantics are written in the following format:

$$ \frac{P_1\\; P_2 \ldots\\; P_n}{Q}, $$
$$ P_i, Q\\; \text{of the form}\\; E \xrightarrow{s} V, s \in \Sigma^* \cup \\{\epsilon\\}. $$

Where $P_i$ is a predicate and $Q$ is an inference which holds if all $P_i,\\; i \in \\{1 \ldots n\\}$ hold, $E$ is a regular expression, $V \in \\{\texttt{True}, \texttt{False}\\}$ is a value, and $E \xrightarrow{s} V$ represents the evaluation of $E$ to $V$ on the string $s$, consuming it in its entirety.

---

EMPTY_STRING
$$\frac{}{
    \epsilon \xrightarrow{\epsilon} \texttt{True}
}$$

CHAR
$$\frac{}{
    \sigma \xrightarrow{\sigma} \texttt{True}
}$$

Concatenation
$$\frac{
    E \xrightarrow{s_1} \texttt{True}, F \xrightarrow{s_2} \texttt{True}
}{
    EF \xrightarrow{s_1s_2} \texttt{True}
}$$

Closure
$$\frac{}{
    E^* \xrightarrow{\epsilon} \texttt{True}
}$$
$$\frac{
    EE^* \xrightarrow{s} \texttt{True}
}{
    E^* \xrightarrow{s} \texttt{True}
}$$

Choice
$$\frac{
    E \xrightarrow{s} \texttt{True}
}{
    E|F \xrightarrow{s} \texttt{True}
}$$
$$\frac{
    F \xrightarrow{s} \texttt{True}
}{
    E|F \xrightarrow{s} \texttt{True}
}$$
