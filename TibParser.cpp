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
    ss << "Unexpected token " << this->token.to_str() << std::endl;
    throw ss.str();
}

void TibParser::match(Tokens t) {
    if (this->token.type == t) {
        this->advance();
    } else {
        this->error();
    }
}

bool TibParser::match_if_is(Tokens type) {
    if (this->token.type == type) {
        this->advance();
        return true;
    } else {
        return false;
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

    if (this->match_if_is(tokens::PLUS)) {
        Value v2 = this->pl_6();
        return Value(v1 + v2);
    } else if (this->match_if_is(tokens::MINUS)) {
        Value v2 = this->pl_6();
        return Value(v1 - v2);
    } else {
        return v1;
    }
}

Value TibParser::pl_7() {
    Value v1 = this->pl_9();

    if (this->match_if_is(tokens::TIMES)) {
        Value v2 = this->pl_7();
        return Value(v1 * v2);
    } else if (this->match_if_is(tokens::DIVIDE)) {
        Value v2 = this->pl_7();
        return Value(v1 / v2);
    } else if (this->token.clss == TClass::VALUE) {
        // Adjacent multiplication
        Value v2 = this->pl_7();
        return Value(v1 * v2);
    } else {
        return v1;
    }
}

Value TibParser::pl_9() {
    if (this->match_if_is(tokens::MINUS)) {
        Value val = this->pl_9();
        return -val;
    } else {
        return this->pl_10();
    }

}

Value TibParser::pl_10() {
    Value val1 = this->pl_13();
    if (this->match_if_is(tokens::POW)) {
        Value val2 = this->pl_10();
        return Value(val1 ^ val2);
    } else {
        return val1;
    }
}

Value TibParser::pl_13() {
    if (this->match_if_is(tokens::L_PAREN)) {
        Value val = this->pl_6();
        // The TI-84 is very "flexible" with closing parentheses, so match it if we can
        if (this->token == tokens::R_PAREN)
            this->match(tokens::R_PAREN);

        return val;
    } else if (this->match_if_is(tokens::L_CURLY)) {
        Value val;
        val.type = ValueTypes::LIST;
        if (this->match_if_is(tokens::R_CURLY)) {
            // Empty list
            return val;
        }
        val = this->pl_13_5();
        this->match_if_is(tokens::R_CURLY);
        return val;
    } else {
        return this->pl_14();
    };
}

// This function is for parsing lists. It will always return a type list.
// eg 1,2,3
// {1}
//    {2}
//       {3}
//    {2,3}
// {1,2,3}
Value TibParser::pl_13_5() {
    // The first value we want to add as-is
    Value val1;
    // Empty lists are already handled in pl_13
    val1 = this->pl_6();
    Value val2;
    if (this->match_if_is(tokens::COMMA)) {
        // There is something else
        val2 = this->pl_13_5();
    }
    Value val_list;
    val_list.type = ValueTypes::LIST;
    val_list.list.push_back(val1);
    // Insert the elements ov val2 instead of pushing them back, this allows chaining.
    val_list.list.insert(val_list.list.end(), val2.list.begin(), val2.list.end());
    return val_list;
}

Value TibParser::pl_14() {

    if (this->token == tokens::NUM) {
        if (this->token.value[0] == 'f') {
            // parse as float
            std::string copy = this->token.value;
            copy.erase(0,1);
            Value val(stod(copy));
            this->match(tokens::NUM);
            return val;

        } else {
            // parse as int
            Value val(stol(this->token.value));
            this->match(tokens::NUM);
            return val;
        }
    } else if (this->match_if_is(tokens::STRING)) {
        Value val(this->token.value);
        this->match(tokens::STRING);
        return val;
    }
    else {
        throw "Not implemented!";
    }
    
}

void TibParser::statement() {
    // For now, just call result since it's the only option
    // Set to ans?
    if (this->token == tokens::EOL)
        return;
    std::cout << this->pl_6().to_str();
    std::cout << std::endl;
}

void TibParser::tib_program() {
    if (this->token == tokens::EOF_) {
        this->match(tokens::EOF_);
    } else {
        this->statement();
        if (this->token == tokens::EOF_)
            return;
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
    } catch (std::string str) {
        this->write_out_string(str);
        return false;
    }
  
    return true;
}