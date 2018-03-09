#ifndef TIBPARSER_H__
#define TIBPARSER_H__

#include <vector>
#include <string>
#include <sstream>
#include <iostream>
#include <fstream>

#include "Token.h"
#include "config.h"
#include "Value.h"

class TibParser {
    Config config;
    std::vector<Token> tokens;
    Token token;
    unsigned int current_index = 0;

    void advance();
    void match(Tokens t);
    void error();

    // Non Terminals
    void tib_program();
    void statement();
    Value result();
    Value result_2();
    Value result_3();

public:
    void write_out_string(std::string str);
    TibParser(std::vector<Token> tokens_, Config config_) : config(config_), tokens(tokens_), token(tokens[0]) {};

    bool parse();
    void output();
};

#endif
