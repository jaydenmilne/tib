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
#include "VariableManager.h"

class TibParser {
    Config& config;
    std::vector<Token> tokens;
    Token token;
    VariableManager vars;
    unsigned int current_index = 0;

    void advance();
    void match(Tokens t);
    bool match_if_is(Tokens type);
    void error();
    void error(std::string err);

    // Non Terminals
    void tib_program();
    void statement();
    Value pl_2();
    Value pl_3();
    Value pl_4();
    Value pl_5();
    Value pl_6();
    Value pl_7();
    Value pl_9();
    Value pl_10();
    Value pl_13();
    Value pl_13_5();
    Value pl_14();
    Value pl_15();

public:
    void write_out_string(std::string str);
    TibParser(std::vector<Token> tokens_, Config& config_) : config(config_), tokens(tokens_), token(tokens[0]), vars(config_) {};

    bool parse();
    void output();
};

#endif
