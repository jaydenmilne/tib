#include "TibScanner.h"

ReturnCode TibScanner::init() {
    this->input.open(this->config.input);
    if (not this->input.is_open()) {
        std::cout << "Failed to open file " << this->config.input << ". Aborting!" << std::endl;
        return ReturnCode::UnableToOpenFile;
    }
    this->lexer = new yyFlexLexer(this->input, std::cout);

    return ReturnCode::OK;
}

ReturnCode TibScanner::read() {
    // Read in all of the tokens to the vector
    do {
        this->parsed_tokens.push_back(this->get_token());
        Token tk = this->parsed_tokens.back();
        if (tk.line_number == 1) {
            tk.line_number = 2;
        }
    } while (this->parsed_tokens.back().type != EOF_);

    if (config.write_tokens) {
        for (auto tk : this->parsed_tokens) {
            std::cout << tk.to_str() << std::endl;
        }
        std::cout << "Total Tokens = " << this->parsed_tokens.size() << std::endl;
    }

    return ReturnCode::OK;
}

Token TibScanner::get_token() {
    Tokens type = static_cast<Tokens>(this->lexer->yylex());

    // yylex returns 0 for EOF, convert it into our version
    if (type == 0)
        type = Tokens::EOF_;

    std::string val = this->lexer->YYText();
    unsigned line_num = this->lexer->lineno();

    if (type == Tokens::EOL)
        --line_num;

    return Token(line_num, type, val);
}

TibScanner::~TibScanner() {
    delete this->lexer;
}