#ifndef TIBSCANNER_H__
#define TIBSCANNER_H__

#include <string>
#include <sstream>
#include <iostream>
#include <vector>
#include <set>

#include "config.h"
#include "Token.h"
#include "InputReader.h"

class TibScanner {
    void add_token(Tokens type, TClass clss, std::string value, unsigned int line_number);
    void add_token(Tokens type, TClass clss, std::string value);
    bool next(char desired);
    void parse_number(char ch);
    void parse_char_operator(char ch);
    void parse_multi_char_operator(char ch);
    void parse_comment(char ch);
    void parse_string(char ch);

public:
    std::vector<Token> parsed_tokens;
    Config& config;
    InputReader in_reader;

    TibScanner(Config& _config) : config(_config), in_reader(_config) {};

    void output_tokens();

    ReturnCode parse();

};

#endif