#ifndef TIBSCANNER_H__
#define TIBSCANNER_H__

#include <string>
#include <sstream>
#include <iostream>
#include <fstream>
#include <vector>

#include "config.h"
#include "Token.h"
#include "FlexLexer.h"

class TibScanner {
    std::ifstream input;
    yyFlexLexer* lexer = nullptr;
    
public:
    Config& config;
    Token get_token();
    ReturnCode init();
    ReturnCode read();

    std::vector<Token> parsed_tokens;
    
    TibScanner(Config& _config) : config(_config) {};
    ~TibScanner();
};

#endif