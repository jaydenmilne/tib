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