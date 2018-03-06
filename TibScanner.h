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
public:
    std::vector<Token> parsed_tokens;
    Config config;
    InputReader in_reader;

    TibScanner(Config _config) : config(_config), in_reader(_config) {};

    std::string output_tokens();

    ReturnCode open();
    ReturnCode parse();

};

#endif