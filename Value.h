#ifndef VALUE_H__
#define VALUE_H__

#include<string>

typedef enum v_types {
    INT,
    FLOAT,
    STRING
} ValueTypes;

// TODO: Implement some beautiful inheritance structure so that the same class
//       isn't used to hold integers, floats and strings (and potentially lists)
//       Downside would be having to use pointers

class Value {
public:
    // Deliberately uninitialized
    long int i_val;
    double f_val;
    std::string s_val;

    ValueTypes type = v_types::FLOAT;

    Value(long int v) : i_val(v), type(v_types::INT) {};
    Value(double v) : f_val(v), type(v_types::FLOAT) {};
    Value(std::string v) : s_val(v), type(v_types::STRING) {};
    Value() {};

    std::string to_str();

    Value operator-();
    Value operator-(const Value& rhs);
    Value operator+(const Value& rhs);
    Value operator*(const Value& rhs);
    Value operator/(const Value& rhs);

};

#endif