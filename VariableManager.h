#ifndef VARIABLES_H__
#define VARIABLES_H__

#include <string>
#include <map>

#include "Value.h"
#include "config.h"

class VariableManager {
    Config& config;
    std::map<std::string, Value> variables;

public:
    VariableManager(Config& config_) : config(config_) {};
    Value get(std::string var_name);
    void set(std::string var_name, Value val);
    Value ans;
};

#endif