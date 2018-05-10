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

void TibParser::error(std::string text) {
    std::stringstream ss;
    ss << "Error: " << text << " at token " << this->token.to_str() << std::endl;
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

Value TibParser::pl_2(){
    Value v1 = this->pl_3();

    if (this->match_if_is(Tokens::OR)) {
        Value v2 = this->pl_2();
        return Value(static_cast<long>(v1 || v2));
    } else if (this->match_if_is(Tokens::XOR)) {
        Value v2 = this->pl_2();
        return Value(static_cast<long>(!v1 != !v2));
    } else {
        return v1;
    }
}

Value TibParser::pl_3(){
    Value v1 = this->pl_4();

    if (this->match_if_is(Tokens::AND)) {
        Value v2 = this->pl_3();
        return Value(static_cast<long>(v1 && v2));
    } else {
        return v1;
    }
}

Value TibParser::pl_4(){
    if (this->match_if_is(Tokens::NOT)) {
        Value v1 = this->pl_2();
        this->match_if_is(Tokens::R_PAREN);
        return Value(static_cast<long>(!v1));
    } else {
        return this->pl_5();
    }
}

Value TibParser::pl_5(){
    Value v1 = this->pl_6();

    if (this->match_if_is(Tokens::EQUAL)) {
        Value v2 = this->pl_5();
        return Value(static_cast<long>(v1 == v2));
    } else if (this->match_if_is(Tokens::N_EQUAL)) {
        Value v2 = this->pl_5();
        return Value(static_cast<long>(v1 != v2));
    } else if (this->match_if_is(Tokens::GREATER)) {
        Value v2 = this->pl_5();
        return Value(static_cast<long>(v1 > v2));
    } else if (this->match_if_is(Tokens::GREQ)) {
        Value v2 = this->pl_5();
        return Value(static_cast<long>(v1 >= v2));
    } else if (this->match_if_is(Tokens::LESS)) {
        Value v2 = this->pl_5();
        return Value(static_cast<long>(v1 < v2));
    } else if (this->match_if_is(Tokens::LESSEQ)) {
        Value v2 = this->pl_5();
        return Value(static_cast<long>(v1 <= v2));
    }

    return v1;
}

Value TibParser::pl_6() {
    Value v1 = this->pl_7();

    if (this->match_if_is(Tokens::PLUS)) {
        Value v2 = this->pl_6();
        return Value(v1 + v2);
    } else if (this->match_if_is(Tokens::MINUS)) {
        Value v2 = this->pl_6();
        return Value(v1 - v2);
    } else {
        return v1;
    }
}

Value TibParser::pl_7() {
    Value v1 = this->pl_9();

    if (this->match_if_is(Tokens::TIMES)) {
        Value v2 = this->pl_7();
        return Value(v1 * v2);
    } else if (this->match_if_is(Tokens::DIVIDE)) {
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
    if (this->match_if_is(Tokens::MINUS)) {
        Value val = this->pl_9();
        return -val;
    } else {
        return this->pl_10();
    }

}

Value TibParser::pl_10() {
    Value val1 = this->pl_13();
    if (this->match_if_is(Tokens::POW)) {
        Value val2 = this->pl_10();
        return Value(val1.exp(val2));
    } else {
        return val1;
    }
}

Value TibParser::pl_13() {
    if (this->match_if_is(Tokens::L_PAREN)) {
        Value val = this->pl_2();
        
        if (this->config.strict) {
            this->match(Tokens::R_PAREN);
        } else {
            this->match_if_is(Tokens::R_PAREN);
        }

        return val;
    } else if (this->match_if_is(Tokens::L_CURLY)) {
        Value val;
        val.type = ValueTypes::LIST;
        if (this->match_if_is(Tokens::R_CURLY)) {
            // Empty list
            return val;
        }
        val = this->pl_13_5();

        if (this->config.strict) {
            this->match(Tokens::R_CURLY);
        } else {
            this->match_if_is(Tokens::R_CURLY);
        }

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
    Value val, TEMP; // TEMP is for debugging only, it can be removed
    val.type = ValueTypes::LIST;
    // Empty lists are already handled in pl_13
    TEMP = this->pl_2();
    val.list.push_back(TEMP);

    if (this->config.emulate && val.list[0].type != ValueTypes::INT && val.list[0].type != ValueTypes::FLOAT) {
        // Tried to put a non-int value in a list, which the TI-84 does not allow.
        this->error("ERR:DATA TYPE, tried to put a non-number into a list and strict mode enabled");
    }
    while (this->match_if_is(Tokens::COMMA)) {
        // There is something to add
        TEMP = this->pl_2();
        val.list.push_back(TEMP);  

        if (this->config.emulate && val.list[0].type != ValueTypes::INT && val.list[0].type != ValueTypes::FLOAT) {
            // Tried to put a non-int value in a list, which the TI-84 does not allow.
            this->error("ERR:DATA TYPE, tried to put a non-number into a list and strict mode enabled");
    }

    }
    return val;
}

Value TibParser::pl_14() {

    if (this->token == Tokens::NUM) {
        if (this->token.value[0] == 'f') {
            // parse as float
            std::string copy = this->token.value;
            copy.erase(0,1);
            Value val(stod(copy));
            this->match(Tokens::NUM);
            return val;

        } else {
            // parse as int
            Value val(stol(this->token.value));
            this->match(Tokens::NUM);
            return val;
        }
    } else if (this->token == Tokens::STRING) {
        Value val(this->token.value);
        this->match(Tokens::STRING);
        return val;
    }
    else {
        this->error("Oops: This token is not implemented, try again later.");
        return Value(-1.0);
    }
    
}

void TibParser::statement() {
    // For now, just call result since it's the only option
    // Set to ans?
    if (this->token == Tokens::EOL)
        return;
    Value result = this->pl_2();
    std::cout << result.to_str();
    std::cout << std::endl;
}

void TibParser::tib_program() {
    if (this->token == Tokens::EOF_) {
        this->match(Tokens::EOF_);
    } else {
        this->statement();
        if (this->token == Tokens::EOF_)
            return;
        this->match(Tokens::EOL);
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