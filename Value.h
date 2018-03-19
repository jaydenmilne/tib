#ifndef VALUE_H__
#define VALUE_H__

#include <string>
#include <sstream>
#include <cmath>
#include <functional>
#include <cmath>

const static double F_PRECISION = 0.00000000001;

typedef enum class v_types {
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

    template<class OP> double generic_compare(const Value& rhs, OP action) {
        switch (this->type) {
            case ValueTypes::INT:
                switch(rhs.type) {
                    case ValueTypes::INT:
                        return action(this->i_val, rhs.i_val);
                    case ValueTypes::FLOAT:
                        return action(this->i_val, rhs.f_val);
                    default:
                        throw "ERR:DATA TYPE";
                };
            case ValueTypes::FLOAT:
                switch(rhs.type) {
                    case ValueTypes::INT:
                        return action(this->f_val, rhs.i_val);
                    case ValueTypes::FLOAT:
                        return action(this->f_val, rhs.f_val);
                    default:
                        throw "ERR: DATA TYPE";
                }
            default:
                throw "ERR: DATA TYPE";
            }
    }

    double get_float() const;

    Value operator-();
    Value operator-(const Value& rhs);
    Value operator+(const Value& rhs);
    Value operator*(const Value& rhs);
    Value operator/(const Value& rhs);
    Value operator^(const Value& param);

};

#endif