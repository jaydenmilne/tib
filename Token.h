#pragma once
#ifndef TOKEN_H_
#define TOKEN_H_

#include <map>
#include <string>
#include <sstream>
#include <set>

typedef enum tokens {
    NUM,
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    EOL,
    EOF_,
    UNDEFINED
} Tokens;

std::string token_name(Tokens token);

std::set<std::string> get_token_set();

class Token {
public:
    Token(unsigned int _ln, Tokens _tp, std::string _val) : line_number(_ln), type(_tp), value(_val) {};
    unsigned int line_number = 0;
    Tokens type = Tokens::UNDEFINED;
    std::string value = "";
    const std::string to_str();

    /*
    std::ostream& operator<< (std::ostream& stream, Token& obj) {
        return stream << obj.toString();
    }
    */

};

#endif
