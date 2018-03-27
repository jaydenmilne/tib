#include <iostream>
#include <string>

#include "config.h"
#include "TibScanner.h"
#include "TibParser.h"

using std::cout;
using std::endl;

void print_help() {
    cout << "Usage: tib [optional arguments] INPUT_FILE" << endl << 
                        "\t--about or -a:\t\t about tib" << endl <<
                        "\t--debug or -d:\t\t enable debug outputs" << endl <<
                        "\t--emulate or -e:\t disable tib extensions (act more like calculator)" << endl <<
                        "\t--quiet or -q:\t\t don't output to standard output" << endl <<
                        "\t--strict or -s:\t\t strict parsing mode (no leaving off parentheses" << endl <<
                        "\t--write or -w:\t\t save output to file" << endl <<
                        "\t--write-tokens or -wt:\t print tokenizer output" << endl;
    return;
}

void print_about() {
    cout << "tib ([t]ib [i]s not TI-[B]asic) v0.0.0.0 \"Super Beta Version\"" << endl <<
            "(c) 2018 Jayden Milne, All Rights Reserved" << endl <<
            "This program is distributed as free software" << endl <<
            "Remember kids - drink lots of milk!" << endl;
}

ReturnCode parse_options(int argc, char* argv[], Config& config) {
    bool messed_up = false;

    // Start at two to ignore command and the first paramater which must be the input file
    for (int i = 1; i < argc; i++) {
        std::string arg = argv[i];
        if (arg == "--debug" || arg == "-d")
            config.debug = true;
        else if (arg == "--about" || arg == "-a") {
            print_about();
            return ReturnCode::Handled;
        }
        else if (arg == "--write" || arg == "-w")
            config.write = true;
        else if (arg == "--quiet" || arg == "-q")
            config.quiet = true;
        else if (arg == "--write-tokens" || arg == "-wt")
            config.write_tokens = true;
        else if (arg == "--emulate" || arg == "-e")
            config.emulate = true;
        else if (arg == "--strict" || arg == "-s")
            config.strict = true;
        else if (arg[0] != '-' && arg[0] != ' ') {
            config.input = arg;
        }
        else {
            cout << "Invalid argument specified: \"" << arg << "\"" << endl; 
            messed_up = true;
        }
    }

    if (messed_up) {
        print_help();
        return ReturnCode::InvalidArgument;
    }
    return ReturnCode::OK;
}

int main(int argc, char*argv[]) {
    if (argc == 1) {
        print_help();
        return ReturnCode::NoInputGiven;
    }
    Config config;
    switch (parse_options(argc, argv, config)) {
        case ReturnCode::InvalidArgument:
            return ReturnCode::InvalidArgument;
        case ReturnCode::Handled:
            return 0;
        default:
            ;
    };

    TibScanner scanner(config);

    ReturnCode code = scanner.parse();
    if (code)
        return code;

    TibParser parser(scanner.parsed_tokens, config);
    parser.parse();

    return code;
}