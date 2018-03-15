#include "TibScanner.h"
void TibScanner::add_token(Tokens type, std::string value, unsigned int line_number) {
    Token new_token(line_number, type, value);
    this->parsed_tokens.push_back(new_token);
}

void TibScanner::add_token(Tokens type, std::string value) {
    unsigned int ln = this->in_reader.line_number;
    this->add_token(type, value, ln);
}

void TibScanner::parse_char_operator(char ch) {
    std::string str(1, ch);

    switch(ch) {
        case '+':
            this->add_token(Tokens::PLUS, str);
            break;
        case '-':
            this->add_token(Tokens::MINUS, str);
            break;
        case '*':
            this->add_token(Tokens::TIMES, str);
            break;
        case '/':
            this->add_token(Tokens::DIVIDE, str);
            break;
        case '\n':
            this->add_token(Tokens::EOL, str, this->in_reader.line_number - 1);
            break;
        default:
            this->add_token(Tokens::UNDEFINED, str);
            std::cout << "Warning: Unrecognized token " << this->parsed_tokens.back().to_str() << std::endl;
    }
};

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
            this->add_token(Tokens::NUM, 'f' + ss.str());
        } else {
            this->add_token(Tokens::NUM, ss.str());
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

    this->add_token(Tokens::EOF_, "");

    return ReturnCode::OK;
}

std::string TibScanner::output_tokens() {
    std::stringstream ss;

    for (auto token : this->parsed_tokens) {
        ss << token.to_str() << std::endl; 
    }

    ss << "Total Tokens = " << this->parsed_tokens.size() << std::endl;

    
    if (!this->config.quiet) {
        std::cout << ss.str();
    }
    
    if (this->config.write) {
        std::cout << "Writing out\n";
        std::ofstream output;
        std::stringstream ss2;

        ss2 << this->config.input << "-out.txt";
        output.open(ss2.str());
        if (!output.is_open()) {
            std::cout << "Failed to open output file " << ss2.str() << "!" << std::endl;
        }
        output << ss.str();
        output.close();
    }

    return ss.str();
}