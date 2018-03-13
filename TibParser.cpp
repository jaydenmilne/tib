#include "TibParser.h"

void TibParser::advance() {
    if (this->current_index == this->tokens.size() - 1)
        return;
    // TODO: if we get EOF or advance beyond the end of the vector, we need to exit somehow
    this->token = this->tokens[++this->current_index];
};

void TibParser::error() {
    // Current token is error
    std::stringstream ss;
    ss << "Failure!" << std::endl << "  " << this->token.to_str() << std::endl;
    throw ss.str();
}

void TibParser::match(Tokens t) {
    if (this->token.type == t) {
        this->advance();
    } else {
        this->error();
    }
}

void TibParser::write_out_string(std::string str) {

    if (this->config.write) {
        std::cout << "Writing out...\n";
        std::ofstream output;
        std::stringstream ss2;
        ss2 << this->config.input << "-out.txt";
        output.open(ss2.str());
        if (!output.is_open()) {
            std::cout << "Failed to open output file " << ss2.str() << "!" << std::endl;
        }
        output << str;
        output.close();
        return;
    }

    std::cout << str;
    return;

}

Value TibParser::pl_6() {
    Value v1 = this->pl_7();

    if (this->token == tokens::PLUS) {
        this->match(tokens::PLUS);
        Value v2 = this->pl_6();
        return Value(v1 + v2);
    } else if (this->token == tokens::MINUS) {
        this->match(tokens::MINUS);
        Value v2 = this->pl_6();
        return Value(v1 - v2);
    } else {
        return v1;
    }
}

Value TibParser::pl_7() {
    Value v1 = this->pl_9();

    if (this->token == tokens::TIMES) {
        this->match(tokens::TIMES);
        Value v2 = this->pl_7();
        return Value(v1 * v2);
    } else if (this->token == tokens::DIVIDE) {
        this->match(tokens::DIVIDE);
        Value v2 = this->pl_7();
        return Value(v1 / v2);
    } else {
        return v1;
    }
}

Value TibParser::pl_9() {
    if (this->token == tokens::MINUS) {
        this->match(tokens::MINUS);
        Value val = this->pl_9();
        return -val;
    } else {
        return this->pl_14();
    }

}

Value TibParser::pl_14() {
    Value val = this->pl_15();
    if (this->token == tokens::DOT) {
        std::string decimal_val;
        this->match(tokens::DOT);
        // Genius, I know.
        decimal_val += '.' + this->pl_15().to_str();
        Value dec_val(std::stod(decimal_val));
        return Value(val + dec_val);
    } else {
        return val;
    }
    
}

Value TibParser::pl_15() {
    std::string num = this->token.value;
    this->match(tokens::NUM);
    return Value(std::stol(num));
}
void TibParser::statement() {
    // For now, just call result since it's the only option
    // Set to ans?
    std::cout << this->pl_6().to_str();
    std::cout << std::endl;
}

void TibParser::tib_program() {
    if (this->token == tokens::EOL) {
        this->match(tokens::EOL);
    } else {
        this->statement();
        this->match(tokens::EOL);
        this->tib_program();
    }
}

bool TibParser::parse() {
    try {
        this->tib_program();
    } catch (char const* e) {
        this->write_out_string(e);
        return false;
    }
  
    return true;
}