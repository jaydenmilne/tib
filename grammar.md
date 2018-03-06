# TIB Grammar
This document contains the parsing grammar for TIB. Non terminals are in quotes.
Based off [this table](http://tibasicdev.wikidot.com/68k:order-of-operations)

P[rogram]   -> S '\n' P | 'EOF'
S[tatement] -> S + S1 | S - S1 | S1
S1          -> S * S2 | SS2 | S / S2 | S3
S3          -> (S3) | S4
S4          -> Num | Rvar
Num         -> 'Digit'Num | -'Digit'Num | .'Digit'Num2 | ϵ
Num2        -> 'Digit'Num2 | ϵ
Rvar        -> 'A' | 'B' | ... | 'Z'