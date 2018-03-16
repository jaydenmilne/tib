#ifndef CONFIG_H_
#define CONFIG_H_

#include <string>

typedef enum returncode {
    Handled = 2,
    NotHandled = 1,
    OK = 0,
    NoInputGiven = -1,
    UnableToOpenFile = -2,
    DirtyWindowsUser = -3
} ReturnCode;

struct Config {
    bool debug = false;
    bool quiet = false;
    bool write = false;
    bool write_tokens = false;
    std::string input = "";
};

#endif