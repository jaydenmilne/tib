#include <iostream>
#include <string>

#include "config.h"
#include "TibScanner.h"
#include "TibParser.h"

using std::cout;
using std::endl;

void print_help() {
    cout << "Usage: tib INPUT_FILE [optional arguments]" << endl << 
                        "\t--write : save output to file" << endl <<
                        "\t--quiet : don't output to standard output" << endl <<
                        "\t--debug : enable debug outputs" << endl;
    return;
}

void parse_options(int argc, char* argv[], Config& config) {
    config.input = argv[1];

    bool messed_up = false;

    // Start at two to ignore command and the first paramater which must be the input file
    for (int i = 2; i < argc; i++) {

        std::string arg = argv[i];
        if (arg == "--debug")
            config.debug = true;
        else if (arg == "--write")
            config.write = true;
        else if (arg == "--quiet")
            config.quiet = true;
        else {
            cout << "Invalid argument specified: \"" << arg << "\", ignoring." << endl; 
            messed_up = true;
        }
    }

    if (messed_up) 
        print_help();
    return;
}

int main(int argc, char*argv[]) {
    if (argc == 1) {
        cout << "You must specify an input file." << endl;
        print_help();
        return ReturnCode::NoInputGiven;
    }
    Config config;

    parse_options(argc, argv, config);

    TibScanner scanner(config);

    ReturnCode code = scanner.parse();

    TibParser parser(scanner.parsed_tokens, config);
    parser.parse();

    return code;
}