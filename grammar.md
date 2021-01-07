# TIB Grammar (V2)
This document contains the parsing grammar for TIB.
Based off [this table](http://tibasicdev.wikidot.com/operators)

| Non Terminal  | -> | Rule          | Rule          | Rule          | Rule          |
|---------------|----|---------------|---------------|---------------|---------------|
| P[rogram]     | -> | S '\n' P      | 'EOF'
| S[tatement]   | -> | PL12          | Command       |
| Command       | -> | Disp PL11     | If PL11       | Then          | Else         
| PL12          | -> | # -> rvar     | PL11
| PL11          | -> | # >Frac       |
| PL10          | -> | # or $        | # xor $       | #
| PL9           | -> | # and $       | #
| PL8           | -> | # [=,!=] $    | # [>,>=] $    | # [<,<=] $    | #
| PL7           | -> | # + $         | # - $         | #
| PL6           | -> | # * $         | # $           | # / $         | #
| PL6           | -> | # * $         | # $           | # / $         | #
| PL5           | -> | # nPr $       | # nCr $       | #
| PL4.5         | -> | -#            | #
| PL4           | -> | #^$           | #xroot$       | #
| PL3           | -> | #!            | #
| PL2           | -> | func(#        | func(#)       | #
| PL1           | -> | (PL2)         | (PL2'EOL'     | {PL13_5'EOL'  | {PL13}
| PL0           | -> | Value

$ = recursion
\# = next priority level
\* = not implemented

## TI-84 Priority Levels (revised)

| Level | Operations
|-------|-------------------------------------------------------------
|   0   | Values and their equivalents (lists, strings)
|   1   | `()`, brackets `[ ]` and braces `{ }`
|   2   | Functions that precede their argument (`sqrt()`, `sin()`)
|   3   | Functions that follow their argument (such as `!`)
|   4   | `^` and `xroot`
|  4.5  | Negation
|   5   | `nPr` and `nCr`
|   6   | Multiplication, division, implied multiplication (`*`, `/`)
|   7   | Addition and subtraction (`+` and `-`)
|   8   | Relational operators (`=`, `!=`, `<`, `>`, `<=`, `>=`)
|   9   | Logical `and`
|   10  | Logical `or` and `xor`
|   11  | Conversions such as `>Frac`
|   12  | Storing Variables (`->`)