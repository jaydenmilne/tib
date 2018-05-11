#pragma once
#ifndef TOKEN_H_
#define TOKEN_H_

#include <string>
#include <sstream>
#include <set>

typedef enum tokens {
    NUM,
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
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    COMMA,
    STRING,
    POW,
    VAR,
    STO,
    EOL,
    EOF_,
    UNDEFINED
} Tokens;

typedef enum token_classes {
    VALUE,       // Anything that has to do with getting a like constants / operators
    UNSUSED,
    KEYWORD,     // Lbl Goto Disp etc (must go at beginning of line)
    FUNCTION,    // Anything that takes paramaters & has parentheses sin() output()
    OPERATOR,
} TClass;

std::string token_name(Tokens token);

std::set<std::string> get_token_set();

class Token {
public:
    Token(unsigned int _ln, Tokens _tp, TClass clss_, std::string _val) : line_number(_ln), clss(clss_), type(_tp), value(_val) {};
    unsigned int line_number = 0;
    TClass clss = TClass::VALUE;
    Tokens type = Tokens::UNDEFINED;
    std::string value = "";
    const std::string to_str();

    bool operator==(const Token &other) const {
        return this->type == other.type;
    };

    bool operator==(const Tokens &other) const {
        return this->type == other;
    }

    /*
    std::ostream& operator<< (std::ostream& stream, Token& obj) {
        return stream << obj.toString();
    }
    */

};

#endif
