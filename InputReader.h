#ifndef INPUT_READER__
#define INPUT_READER__

#include <iostream>
#include <string>
#include <fstream>

#include "config.h"

class InputReader {
public: // Fight me

    InputReader(Config _config) : config(_config) {};
    ~InputReader();

    Config config;
    std::ifstream input;

    unsigned int line_number = 1;
    ReturnCode open();
    char get_ch();
    char get_ch(bool return_endl);
};

#endif