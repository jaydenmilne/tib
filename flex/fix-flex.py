#!/usr/bin/env python3
"""
Python script to make the needed modifications the the flex output for use in 
tib
"""

# Read in file
file = open("lex.yy.cc", 'r')

split_file = file.read().split("\n")

# Add pragma to surpress gcc warnings

split_file.insert(0, '#pragma GCC diagnostic ignored "-Wsign-compare"')

# Replace system header include with local include
if "#include <FlexLexer.h>" not in split_file:
    print("Error: could not find #include <FlexLexer.h> in the file???!!")
    exit()

split_file[split_file.index("#include <FlexLexer.h>")] = '#include "FlexLexer.h"'

file.close()

file = open("../FlexLexer.cpp", 'w')

file.write('\n'.join(split_file))
file.close()

print("done")