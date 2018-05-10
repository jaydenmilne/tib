#ifndef VALUE_H__
#define VALUE_H__

#include <string>
#include <sstream>
#include <cmath>
#include <functional>
#include <cmath>
#include <vector>
#include <iostream>

const static double F_PRECISION = 0.00000000001;

typedef enum class v_types {
    INT,
    FLOAT,
    STRING,
    LIST,
    MATRIX      // Not implemented for now
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
    std::vector<Value> list;

    ValueTypes type = v_types::FLOAT;

    Value(long int v) : i_val(v), type(v_types::INT) {};
    Value(double v) : f_val(v), type(v_types::FLOAT) {};
    Value(std::string v) : s_val(v), type(v_types::STRING) {};
    Value(std::vector<Value> v_) : list(v_), type(v_types::LIST) {};
    Value() {};

    std::string to_str();

    Value detect_type(Value val) const;
    Value detect_type(double val) const;

    template<class OP> Value generic_compare(const Value& rhs, OP action) {
        return this->generic_compare(*this, rhs, action);
    }

    template<class OP> Value generic_compare(const Value& lhs, const Value& rhs, OP action) {
        // TODO: Replace pyramid/switch case of doom with something better
        switch (lhs.type) {
            case ValueTypes::INT:
                switch(rhs.type) {
                    case ValueTypes::INT:
                        return this->detect_type(action(lhs.i_val, rhs.i_val));
                    case ValueTypes::FLOAT:
                        return this->detect_type(action(lhs.i_val, rhs.f_val));
                    case ValueTypes::LIST: {
                        // LHS is not list, RHS is
                        Value val;
                        val.type = ValueTypes::LIST;
                        for (auto& item : rhs.list) {
                            val.list.push_back(this->detect_type(this->generic_compare(lhs, item, action)));
                        }
                        return val;
                    }
                    default:
                        throw "ERR:DATA TYPE";
                };
            case ValueTypes::FLOAT:
                switch(rhs.type) {
                    case ValueTypes::INT:
                        return this->detect_type(action(lhs.f_val, rhs.i_val));
                    case ValueTypes::FLOAT:
                        return this->detect_type(action(lhs.f_val, rhs.f_val));
                    case ValueTypes::LIST: {
                        // LHS is not list, RHS is
                        Value val;
                        val.type = ValueTypes::LIST;
                        for (auto& item : rhs.list) {
                            val.list.push_back(this->detect_type(this->generic_compare(lhs, item, action)));
                        }
                        return val;
                    }
                    default:
                        throw "ERR: DATA TYPE"; 
                }
            case ValueTypes::LIST: {
                Value val;
                val.type = ValueTypes::LIST;
                switch(rhs.type) {
                    case ValueTypes::LIST:
                        // Both sides are lists
                        if (lhs.list.size() != rhs.list.size()) {
                            throw "ERR: DIM MISMATCH";
                        }

                        for(std::vector<Value>::size_type i = 0; i != lhs.list.size(); i++) {
                            // Allows for recursion and nested lists
                            val.list.push_back(this->detect_type(this->generic_compare(lhs.list[i], rhs.list[i], action)));
                        }
                        return val;
                    default:
                        // LHS is list, RHS isn't, so iterate over LHS and apply RHS to each
                        for (auto& item : lhs.list) {
                            val.list.push_back(this->detect_type(this->generic_compare(item, rhs, action)));
                        }
                        return val;
                }
            }
            default:
                throw "ERR: DATA TYPE";
            }
    }

    double get_float() const;

    explicit operator bool() const {
        switch(type) {
            case ValueTypes::STRING:
                return this->s_val.length();
            case ValueTypes::INT:
                return this->i_val;
            case ValueTypes::FLOAT:
                return this->f_val;
            case ValueTypes::LIST:
                return this->list.size();
            default:
                return false;
        }
    };
    Value operator-();
    Value operator-(const Value& rhs);
    Value operator+(const Value& rhs);
    Value operator*(const Value& rhs);
    Value operator/(const Value& rhs);

    Value operator==(const Value& rhs);
    Value operator!=(const Value& rhs);
    Value operator<(const Value& rhs);
    Value operator>(const Value& rhs);
    Value operator<=(const Value& rhs);
    Value operator>=(const Value& rhs);
    Value exp(const Value& param);

};

#endif