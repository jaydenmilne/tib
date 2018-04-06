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
            throw "OOPS: Tried to get_float for a non-floatable value...";
    }
    return -1;
}

Value Value::detect_type(Value val) const {
    // Lists are already done
    if (val.type == ValueTypes::LIST)
        return val;
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

Value Value::exp(const Value& exp) {
    Value val;
    switch(this->type) {
        case ValueTypes::LIST: 
            val.type = ValueTypes::LIST;
            switch(exp.type) {
                case ValueTypes::LIST:
                    // Check to make sure they are the same size
                    if (this->list.size() != exp.list.size())
                        throw "ERR:DIM MISMATCH";
                    for(std::vector<Value>::size_type i = 0; i != this->list.size(); i++) {
                            // Allows for recursion and nested lists
                            val.list.push_back(this->detect_type(this->list[i].exp(exp.list[i])));
                        }
                    return val;
                default:
                    // Loop over every element of base and apply operation
                    for (auto& item : this->list) {
                        val.list.push_back(item.exp(exp));
                    }
                    return val;
            }
        default:
            switch(exp.type) {
                case ValueTypes::LIST: {
                    val.type = ValueTypes::LIST;
                    // Loop over every element of base and apply operation
                    for (auto& item : exp.list) {
                        val.list.push_back(this->exp(item));
                    }
                    return val;

                }
                default:
                    // Both are not lists, free to just do get_float
                return this->detect_type(std::pow(this->get_float(), exp.get_float()));
            }
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
            return '"' + this->s_val + '"';
        case ValueTypes::LIST:
        {
            std::stringstream ss;
            ss << "{";
            for (std::vector<Value>::size_type i = 0; i < this->list.size(); i++) {
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