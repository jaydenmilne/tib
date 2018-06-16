# Procedure for re-generating parser

1. ```flex flex-input```
2. Change the line
```
#include <FlexLexer.h>
```
to
```
#include "FlexLexer.h"
```
3. Add
```
#pragma GCC diagnostic ignored "-Wsign-compare"
```
to the beginning of the .cc file to surpress error and allow the build to succeed (we compile with -Wall and -Werror).

4. Rename to FlexLexer.cpp and overwrite old lexer