# Declaration of variables
CC = g++
CC_FLAGS = -Wall -Werror -std=c++14 -g3

# File names
EXEC = tib
SOURCES = $(wildcard *.cpp)
OBJECTS = $(SOURCES:.cpp=.o)
 
# Main target
$(EXEC): $(OBJECTS)
	$(CC) $(OBJECTS) -o $(EXEC)
 
# To obtain object files
%.o: %.cpp
	$(CC) -c $(CC_FLAGS) $< -o $@
 
# To remove generated files
clean:
	del -f $(EXEC) $(OBJECTS)