# TIB Grammar
This document contains the parsing grammar for TIB. Non terminals are in quotes.
Based off [this table](http://tibasicdev.wikidot.com/68k:order-of-operations)

|Non Terminal   | -> | Rule          | Rule          | Rule          | Rule          |
|---------------|----|---------------|---------------|---------------|---------------|
| P[rogram]     | -> | S '\n' P      | 'EOF'
| S[tatement]   | -> | PL6           | (Disp, etc)*
| PL6           | -> | # + $         | # - $         | #
| PL7           | -> | # * $         | # $           | # / $         | #
| PL9           | -> | -PL14         | #
| PL14          | -> | [num]$        | .[num]        | rvar

$ = recursion
\# = next priority level
\* = not implemented

## TI-84 Priority Levels
(Highest number = highest priority)
| Level | Operations
|-------|-----------
|  14   | Values and their equivalents (variables and constants)
|  13 	| `()`, brackets `[ ]`, and braces `{ }`
|  12 	| Functions (`sin()`, `dim()`)
|  11 	| Operators that go after their operand, eg `{1,2}(1)`
|  10 	| Exponentiation (`^`)
|   9 	| Negation (`-`)
|   8   | String concatenation (`+`)
|   7 	| Multiplication and division (`*`, `/`)
|   6 	| Addition and Subtraction (`+`, `-`)
|   5 	| Equality relations: (`=`, `≠`, `>`, `≥`, `<`, `≤`)
|   4 	| Logical and arithmetic `not`
|   3 	| Logical and arithmetic `and`
|   2 	| Logical and arithmetic `or`, `xor`
|   1 	| Store (`→`)