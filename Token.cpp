#include "Token.h"

unsigned const int NumTokens = 9;

char TokenNames[NumTokens][11] = {
    "NUM",
    "+",
    "-",
    "*",
    "/",
    "(EOL)",
    "(EOF)",
    "Undefined!"
};

std::string token_name(Tokens token) {
    return TokenNames[token];
}

std::set<std::string> get_token_set() {
    std::set<std::string> token_set;

    for (unsigned int i = 0; i < NumTokens; i++) {
        token_set.insert(TokenNames[i]);
    }

    return token_set;
}

const std::string Token::to_str() {
    std::stringstream ss;
    ss << '(' << token_name(this->type) << ",\"" << this->value << "\"," << this->line_number << ")";
    return ss.str();
}