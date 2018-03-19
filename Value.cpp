#include "Value.h"

bool is_int(double d) {
    return (fabs(roundf(d) - d) <= F_PRECISION);
}

double Value::get_float() const {
    switch(this->type) {
        case ValueTypes::FLOAT:
            return this->f_val;
        case ValueTypes::INT:
            return this->i_val;
        default:
            throw "ERR:DATA TYPE";
    }
    return -1;
}

Value Value::operator-() {
    switch(this->type){
        case ValueTypes::INT:
            return Value(-this->i_val);
        case ValueTypes::FLOAT:
            return Value(-this->f_val);
        case ValueTypes::STRING:
            throw "ERR:DATA TYPE\nAttempted to negate string.";
        default:
            throw "ERR:DATA TYPE";
    }
}

Value Value::operator-(const Value& rhs) {
    double result = this->generic_compare(rhs, std::minus<double>());
    if (is_int(result)) {
        return Value(std::lround(result));
    } else {
        return Value(result);
    }
}


Value Value::operator+(const Value& rhs) {
    Value val;

    if (this->type == ValueTypes::STRING && rhs.type == ValueTypes::STRING) {
        val.type = ValueTypes::STRING;
        val.s_val = this->s_val + rhs.s_val;
        return val;
    }

    double result = this->generic_compare(rhs, std::plus<double>());
    if (is_int(result)) {
        return Value(std::lround(result));
    } else {
        return Value(result);
    }
}

Value Value::operator*(const Value& rhs) {
    double result = this->generic_compare(rhs, std::multiplies<double>());
    if (is_int(result)) {
        return Value(std::lround(result));
    } else {
        return Value(result);
    }
}

Value Value::operator/(const Value& rhs) {
    double result = this->generic_compare(rhs, std::divides<double>());
    if (is_int(result)) {
        return Value(std::lround(result));
    } else {
        return Value(result);
    }
}

Value Value::operator^(const Value& exp) {
    double result = std::pow(this->get_float(), exp.get_float());

    if (is_int(result)) {
        return Value(std::lround(result));
    } else {
        return Value(result);
    }
}

std::string Value::to_str() {
    switch (this->type) {
        case ValueTypes::INT:
            return std::to_string(this->i_val);
        case ValueTypes::FLOAT: {
            std::stringstream ss;
            ss << this->f_val;
            return ss.str();
        }
        case ValueTypes::STRING:
            return this->s_val;
        default:
            throw "Not Implemented Exception!";
    }
}