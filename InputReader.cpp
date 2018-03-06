#include "InputReader.h"

ReturnCode InputReader::open() {  
    this->input.open(config.input);

    if (not this->input.is_open()) {
        std::cout << "Failed to open file " << config.input << ". Aborting!" << std::endl;
        return ReturnCode::UnableToOpenFile;
    }

    return ReturnCode::OK;
}

/*
    Gets the next character in the stream
    Will not return a newline character
*/
char InputReader::get_ch() {
    return this->get_ch(false);
};

/*
    Gets the next character in the stream
    Will return a newline character if return_endl is true
*/
char InputReader::get_ch(bool return_endl) {
    char ch = this->input.get();

    switch(ch) {
    case '\r':
        std::cout << "WARNING! \\r character detected in input, this parser is not build to handle that. Unexpected results may occur." << std::endl;
        break;
    case '\n':
        this->line_number++;

        if (!return_endl)
            ch = this->get_ch();
        
    default:
        return ch;
    }

    // Bow before the compiler gods
    return ch;
}

InputReader::~InputReader() {
    this->input.close();
}