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

Value Value::detect_type(Value val) const {
    if (is_int(val.get_float())) {
        return Value(std::lround(val.get_float()));
    } else {
        return val;
    }
}

Value Value::detect_type(double val) const {
    if (is_int(val)) {
        return Value(std::lround(val));
    } else {
        return Value(val);
    }
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
    return this->detect_type(this->generic_compare(rhs, std::minus<double>()));
}


Value Value::operator+(const Value& rhs) {
    Value val;

    if (this->type == ValueTypes::STRING && rhs.type == ValueTypes::STRING) {
        val.type = ValueTypes::STRING;
        val.s_val = this->s_val + rhs.s_val;
        return val;
    }

    return this->detect_type(this->generic_compare(rhs, std::plus<double>()));
}

Value Value::operator*(const Value& rhs) {
    return this->detect_type(this->generic_compare(rhs, std::multiplies<double>()));
}

Value Value::operator/(const Value& rhs) {
    return this->detect_type(this->generic_compare(rhs, std::divides<double>()));
}

Value Value::operator^(const Value& exp) {
    return this->detect_type(std::pow(this->get_float(), exp.get_float()));
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
        case ValueTypes::LIST:
        {
            std::stringstream ss;
            ss << "{";
            for (std::vector<Value>::size_type i; i < this->list.size(); i++) {
                ss << this->list[i].to_str();
                if (i != this->list.size() - 1) {
                    ss << ", ";
                }
            }
            ss << "}";
            return ss.str();
        }
        default:
            throw "Not Implemented Exception!";
    }
}