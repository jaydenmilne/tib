#include "VariableManager.h"

Value VariableManager::get(std::string var_name) {
    auto it = this->variables.find(var_name);
    if (it == this->variables.end()) {
        // Variable was not set. User deserves everything that is going to happen to them.
        // TODO: Detect variable type from name and return correct type, but with garbage data (tib #3)
        Value val;
        this->variables[var_name] = val;
        return val;
    } else {
        return this->variables[var_name];
    }
}

void VariableManager::set(std::string var_name, Value val) {
    // TODO: Detect type from name and when in emulation mode enforce types when appropriate (tib #2)
    this->variables[var_name] = val;
}
