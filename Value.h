#ifndef VALUE_H__
#define VALUE_H__

class Value {
    // In the future this will hold floats, lists, etc but for now just an int
public:
    long int value = 0;
    Value(int v) : value(v) {};
};

#endif