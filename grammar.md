# TIB Grammar
This document contains the parsing grammar for TIB. Non terminals are in quotes.
Based off [this table](http://tibasicdev.wikidot.com/68k:order-of-operations)

|Non Terminal   | -> | Rule          | Rule          | Rule          | Rule          |
|---------------|----|---------------|---------------|---------------|---------------|
| P[rogram]     | -> | S '\n' P      | 'EOF'
| S[tatement]   | -> | R             | (Disp, etc)*
| R[esult]      | -> | R2 + R        | R2 - R        | R2
| R2            | -> | R2 * R3       | R2R3*         | R2 / R3       | R3
| R3            | -> | Num           | Rvar
| Num           | -> | 'Digit'Num    | -'Digit'Num   | .'Digit'Num2  | ϵ
| Num2          | -> | 'Digit'Num2   | ϵ
| Rvar*         | -> | 'A'           | 'B'           | ...           | 'Z'


\* = not implemented

## Priority Levels
(Highest number = highest priority)
| Level | Operations                                                                |
|-------|---------------------------------------------------------------------------|
|   0   | Values and their equivalents (variables and constants)                    |
|   1 	| ( ), brackets [ ], and braces { }                                         |
|   3 	| Function calls                                                            |
|   4 	| Operators that go after their operand, eg {1,2}(1)                        |
|   5 	| Exponentiation (^, .^)2                                                   |
|   6 	| Negation (-)                                                              | 
|   7   | String concatenation (+)                                                  |  
8 	Multiplication and division (*, /, .*, ./)
9 	Addition and Subtraction (+, -, .+, .-)
10 	Equality relations: (=, ≠, >, ≥, <, ≤)
11 	Logical and arithmetic not3
12 	Logical and arithmetic and
13 	Logical and arithmetic or, xor
14 	Constraint "with" operator (|)
15 	Store (→)