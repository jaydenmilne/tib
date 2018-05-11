#include "TibScanner.h"
void TibScanner::add_token(Tokens type, TClass clss, std::string value, unsigned int line_number) {
    Token new_token(line_number, type, clss, value);
    this->parsed_tokens.push_back(new_token);
}

void TibScanner::add_token(Tokens type, TClass clss, std::string value) {
    unsigned int ln = this->in_reader.line_number;
    this->add_token(type, clss, value, ln);
}

bool TibScanner::next(char desired) {
    if (this->in_reader.input.peek() == desired) {
        this->in_reader.get_ch();
        return true;
    } else {
        return false;
    }
}

bool is_uppercase_letter(char ch) {
    return ch >= 65 && ch <= 90;
}

void TibScanner::parse_comment(char ch) {
    // Comments are ignored till the end of the line
        char n_char = 0;
        while (
            ((n_char = this->in_reader.input.peek()) != '\n' )  &&
            (n_char != EOF) &&
            (ch = this->in_reader.get_ch())
        ) {};
}

void TibScanner::parse_string(char ch) {
    // We don't care about the " character
    std::stringstream ss;
    char n_char;
    do {
        ss << ch;
    } while (
        ((n_char = this->in_reader.input.peek()) != '\n') &&
        (n_char != EOF) && 
        (n_char != '"') &&
        (ch = this->in_reader.get_ch())
    );

    // The first char is a ", we don't want it
    if (n_char == '"')
        this->in_reader.get_ch();
        
    this->add_token(Tokens::STRING, TClass::VALUE, ss.str().erase(0, 1));
    return;
}

void TibScanner::parse_char_operator(char ch) {
    std::string str(1, ch);

    switch(ch) {
        case '+':
            this->add_token(Tokens::PLUS, TClass::OPERATOR, str);
            break;
        case '-':
            if (!this->next('-'))
                this->add_token(Tokens::MINUS, TClass::OPERATOR, str);
            break;
        case '*':
            this->add_token(Tokens::TIMES, TClass::OPERATOR, str);
            break;
        case '/':
            this->add_token(Tokens::DIVIDE, TClass::OPERATOR, str);
            break;
        case '\n':
            this->add_token(Tokens::EOL, TClass::KEYWORD, str, this->in_reader.line_number - 1);
            break;
        case '#':
            this->parse_comment(ch);
            break;
        case '(':
            this->add_token(Tokens::L_PAREN, TClass::VALUE, str);
            break;
        case ')':
            this->add_token(Tokens::R_PAREN, TClass::OPERATOR, str);
            break;
        case '{':
            this->add_token(Tokens::L_CURLY, TClass::VALUE, str);
            break;
        case '}':
            this->add_token(Tokens::R_CURLY, TClass::OPERATOR, str);
            break;
        case ',':
            this->add_token(Tokens::COMMA, TClass::OPERATOR, str);
            break;
        case '"':
            this->parse_string(ch);
            break;
        case '^':
            this->add_token(Tokens::POW, TClass::OPERATOR, str);
            break;
        case '=':
            this->add_token(tokens::EQUAL, TClass::OPERATOR, str);
            break;
        default:
            if (is_uppercase_letter(ch)) {
                this->add_token(Tokens::VAR, TClass::VALUE, str);
                break;
            }
            this->parse_multi_char_operator(ch);
    }
};

void TibScanner::parse_multi_char_operator(char ch) {
    std::string str(1, ch);

    switch(ch) {
        case '-':
            if (this->next('>')) {
                this->add_token(tokens::STO, TClass::OPERATOR, "->");
            } else {
                this->add_token(tokens::UNDEFINED, TClass::VALUE, "-"); // should be impossible
            }
        case '>':
            if (this->next('=')) {   
                this->add_token(tokens::GREQ, TClass::OPERATOR, ">=");
            } else {
                this->add_token(tokens::GREATER, TClass::OPERATOR, str);
            }
            break;
        case '<':
            if (this->next('=')) {
                this->add_token(tokens::LESSEQ, TClass::OPERATOR, "<=");
            } else {
                this->add_token(tokens::LESS, TClass::OPERATOR, str);
            }
            break;
        case 'o':
            if (this->next('r')) {
                this->add_token(tokens::OR, TClass::OPERATOR, "or");
            }
            else {
                this->add_token(Tokens::UNDEFINED, TClass::VALUE, str);
            }
            break;
        case 'a':
            if (this->next('n') && this->next('d')) {
                this->add_token(tokens::AND, TClass::OPERATOR, "and");
            } else { // TODO: this won't have the correct token in str if n matches and d doesn't
                this->add_token(Tokens::UNDEFINED, TClass::VALUE, str);
            }
            break;
        case 'n':
            if (this->next('o') && this->next('t') && this->next('(')) {
                this->add_token(tokens::NOT, TClass::OPERATOR, "not(");
            } else {
                this->add_token(tokens::UNDEFINED, TClass::VALUE, str);
            }
            break;
        case 'x':
            if (this->next('o') && this->next('r')) {
                this->add_token(tokens::XOR, TClass::OPERATOR, "xor");
            } else {
                this->add_token(tokens::UNDEFINED, TClass::VALUE, str);
            }
            break;
        case '!':
            if (this->next('=')) {
                this->add_token(tokens::N_EQUAL, TClass::OPERATOR, "!=");
            }
            break;
        default:
            this->add_token(Tokens::UNDEFINED, TClass::VALUE, str);
            std::cout << "Warning: Unrecognized token " << str << std::endl;
        }
}
void TibScanner::parse_number(char ch) {
    if (isdigit(ch) || ch == '.') {
        bool is_float = false;
        std::stringstream ss;

        char n_char = 0;
        do {
            if (ch == '.')
                is_float = true;
            ss << ch;
        } while (
            ((n_char = this->in_reader.input.peek()) != '\n' )  &&
            (n_char != EOF) &&
            // As long as the next character is a digit, we'll keep consuming it as part of this one
            (isdigit(n_char) || n_char == '.') &&
            (ch = this->in_reader.get_ch())
        );

        if (is_float) {
            this->add_token(Tokens::NUM, TClass::VALUE, 'f' + ss.str());
        } else {
            this->add_token(Tokens::NUM, TClass::VALUE, ss.str());
        }

    } else {
        this->parse_char_operator(ch);
    }
}

ReturnCode TibScanner::parse() {
    ReturnCode code = this->in_reader.open();
    if (code)
        return code;
    
    char ch = this->in_reader.get_ch();

    if (ch != EOF) {
        do {
            if (isspace(ch) && ch != '\n')
                continue;
            this->parse_number(ch);
        } while ((ch = this->in_reader.get_ch()) != EOF);
    }

    this->add_token(Tokens::EOF_, TClass::KEYWORD, "");

    if (this->config.write_tokens)
        this->output_tokens();

    return ReturnCode::OK;
}

void TibScanner::output_tokens() {
    for (auto token : this->parsed_tokens) {
        std::cout << token.to_str() << std::endl; 
    }
    std::cout << "Total Tokens = " << this->parsed_tokens.size() << std::endl;
    
    return;
}