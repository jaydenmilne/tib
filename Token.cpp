#include "Token.h"

unsigned const int NumTokens = 9;

char TokenNames[NumTokens][11] = {
    "NUM",
    "PLUS",
    "MINUS",
    "TIMES",
    "DIVIDE",
    "DOT",
    "EOL",
    "EOF",
    "UNDEFINED"
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
    std::string value = this->value;
    if (value == "\n")
        value = "\\n";

    ss << '(' << token_name(this->type) << ",\"" << value << "\"," << this->line_number << ")";
    return ss.str();
}