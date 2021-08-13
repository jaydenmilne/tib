// Based off of https://github.com/tree-sitter/tree-sitter-go/blob/master/grammar.js

const 
    PRECEDENCE = {
        values: 14,
        grouping: 13,
        functions: 12,
        post_functions: 11,
        exponentiation: 10,
        negation: 9,
        stats_thing: 8,
        multiplicative: 7,
        additive: 6,
        comparative: 5,
        and: 4,
        or: 3,
        conversions: 2,
        store: 1
    },

    multiplicative_operators = ['*', '/'],
    additive_operators = ['+', '-'],
    comparative_operators = ['=', '!=', '<', '>', '<=', '>='],
    
    newline = '\n',
    terminator = choice(newline, ':'),
    
    decimalDigit = /[0-9]/,
    decimalDigits = repeat1(decimalDigit),
    decimalFloat = choice(
        // 3. and 3.00
        seq(decimalDigits, '.', optional(decimalDigits)),
        seq('.', decimalDigits)
    );

module.exports = grammar({
    name: 'tib',
    rules: {
        program: $ => repeat(seq($._statement, terminator)),

        _statement: $ => choice(
            $._command,
            $._expression
        ),

        _command: $ => 'Disp',

        _expression: $ => choice(
            $.int_literal,
            $.float_literal
        ),

        int_literal: $ => token(decimalDigits),

        float_literal: $ => token(decimalFloat)
    }
});