#include "Token.h"

char TokenNames[Tokens::__LAST][25] = {
    "__CATEGORY_VALUES",
    "NUM",
    "STRING",
    "VAR",
    "L_PAREN",
    "L_CURLY",

    "__CATEGORY_OPERATORS",
    "PLUS",
    "MINUS",
    "TIMES",
    "DIVIDE",
    "OR",
    "AND",
    "XOR",
    "NOT",
    "EQUAL",
    "N_EQUAL",
    "GREATER",
    "GREQ",
    "LESS",
    "LESSEQ",
    "R_PAREN",
    "R_CURLY",
    "COMMA",
    "POW",

    "__CATEGORY_KEYWORDS",
    "DISP",
    "LBL",
    "GOTO",
    "IF",
    "STO",
    "COLON",
    "EOL",
    "EOF",
    "UNDEFINED"
};

char TClassNames[sizeof(TClass)][9] = {
    "VALUE",
    "KEYWORD",
    "OPERATOR"
};

std::string token_name(Tokens token) {
    return TokenNames[token];
}

const std::string Token::to_str() {
    std::stringstream ss;
    std::string value = this->value;
    if (value == "\n") {
        value = "/n"; // use backwards slash so that the testrunner doesn't turn it into an actual carriage return
    }

    ss << TokenNames[this->type] << "/" << TClassNames[this->clss] << " with value \"" << value << "\" on line " << this->line_number;
    return ss.str();
}

TClass Token::get_class() const {
    if (this->type < Tokens::__CATEGORY_OPERATORS) {
        return TClass::VALUE;
    } else if (this->type > Tokens::__CATEGORY_OPERATORS && this->type < Tokens::__CATEGORY_KEYWORDS) {
        return TClass::OPERATOR;
    } else {
        return TClass::KEYWORD;
    }
}