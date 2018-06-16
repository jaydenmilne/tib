#pragma once
#ifndef TOKEN_H_
#define TOKEN_H_

#include <string>
#include <sstream>

// Note: The following enum is sorted into the tree token_classes categories
//       be sure to add new tokens to the correct category
typedef enum tokens {
    __CATEGORY_VALUES,
    NUM,
    STRING,
    VAR,
    L_PAREN,
    L_CURLY,

    __CATEGORY_OPERATORS,
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    OR,
    AND,
    XOR,
    NOT,
    EQUAL,
    N_EQUAL,
    GREATER,
    GREQ,
    LESS,
    LESSEQ,
    R_PAREN,
    R_CURLY,
    COMMA,
    POW,

    __CATEGORY_KEYWORDS,
    DISP,
    LBL,
    GOTO,
    IF,
    STO,
    COLON,
    EOL,
    EOF_,
    UNDEFINED,
    __LAST // Last token, for size measurement purposes
} Tokens;

typedef enum token_classes {
    VALUE,       // Anything that has to do with getting a like constants / operators
    KEYWORD,     // Lbl Goto Disp etc (must go at beginning of line)
    OPERATOR,
} TClass;

std::string token_name(Tokens token);

TClass get_class(Tokens tok);

class Token {
    TClass get_class() const;
public:
    Token(unsigned int _ln, Tokens _tp, std::string _val) : line_number(_ln), type(_tp), clss(get_class()),  value(_val) {};
    unsigned int line_number = 0;
    Tokens type = Tokens::UNDEFINED;   
    TClass clss = TClass::VALUE;
    std::string value = "";
    const std::string to_str();

    bool operator==(const Token &other) const {
        return this->type == other.type;
    };

    bool operator==(const Tokens &other) const {
        return this->type == other;
    }

};

#endif
