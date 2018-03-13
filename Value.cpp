#include "Value.h"

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
    Value val;

    switch (this->type) {
    case ValueTypes::INT:
        switch(rhs.type) {
            case ValueTypes::INT:
                val.type = ValueTypes::INT;
                val.i_val = this->i_val - rhs.i_val;
                return val;
            case ValueTypes::FLOAT:
                val.type = ValueTypes::FLOAT;
                val.i_val = this->i_val - rhs.f_val;
                return val;
            default:
                throw "ERR:DATA TYPE";
        };
    case ValueTypes::FLOAT:
        val.type = ValueTypes::FLOAT;
        switch(rhs.type) {
            case ValueTypes::INT:
                val.f_val = this->f_val - this->i_val;
                return val;
            case ValueTypes::FLOAT:
                val.f_val = this->f_val - this->f_val;
                return val;
            default:
                throw "ERR: DATA TYPE";
        }
    default:
        throw "ERR: DATA TYPE";
    }
}


Value Value::operator+(const Value& rhs) {
    Value val;

    if (this->type == ValueTypes::STRING && rhs.type == ValueTypes::STRING) {
        val.type = ValueTypes::STRING;
        val.s_val = rhs.s_val + this->s_val;
        return val;
    }
    // TODO: Add code to change from float to int when a whole number is detected
    //       eg f0.5 + f0.5 = i1
    //       Right now it just makes it a float if any part of it is a float.
    switch (this->type) {
        case ValueTypes::INT:
            switch(rhs.type) {
                case ValueTypes::INT:
                    val.type = ValueTypes::INT;
                    val.i_val = this->i_val + rhs.i_val;
                    return val;
                case ValueTypes::FLOAT:
                    val.type = ValueTypes::FLOAT;
                    val.f_val = this->i_val + rhs.f_val;
                    return val;
                default:
                    throw "ERR:DATA TYPE";
            };
        case ValueTypes::FLOAT:
            val.type = ValueTypes::FLOAT;
            switch(rhs.type) {
                case ValueTypes::INT:
                    val.f_val = this->f_val + this->i_val;
                    return val;
                case ValueTypes::FLOAT:
                    val.f_val = this->f_val + this->f_val;
                    return val;
                default:
                    throw "ERR: DATA TYPE";
            }
        default:
            throw "ERR: DATA TYPE";
    }
}

Value Value::operator*(const Value& rhs) {
    Value val;

    switch (this->type) {
    case ValueTypes::INT:
        switch(rhs.type) {
            case ValueTypes::INT:
                val.type = ValueTypes::INT;
                val.i_val = this->i_val * rhs.i_val;
                return val;
            case ValueTypes::FLOAT:
                val.type = ValueTypes::FLOAT;
                val.i_val = this->i_val * rhs.f_val;
                return val;
            default:
                throw "ERR:DATA TYPE";
        };
        break;
    case ValueTypes::FLOAT:
        val.type = ValueTypes::FLOAT;
        switch(rhs.type) {
            case ValueTypes::INT:
                val.f_val = this->f_val * this->i_val;
                return val;
            case ValueTypes::FLOAT:
                val.f_val = this->f_val * this->f_val;
                return val;
            default:
                throw "ERR: DATA TYPE";
        }
    default:
        throw "ERR: DATA TYPE";
    }
}

Value Value::operator/(const Value& rhs) {
    Value val;
    // No way to easily anticipate what type it will be, so just use a float
    val.type = ValueTypes::FLOAT;

    switch (this->type) {
    case ValueTypes::INT:
        switch(rhs.type) {
            case ValueTypes::INT:
                val.f_val = this->i_val / rhs.i_val;
                return val;
            case ValueTypes::FLOAT:
                val.f_val = this->i_val / rhs.f_val;
                return val;
            default:
                throw "ERR:DATA TYPE";
        };
    case ValueTypes::FLOAT:
        switch(rhs.type) {
            case ValueTypes::INT:
                val.f_val = this->f_val - this->i_val;
                return val;
            case ValueTypes::FLOAT:
                val.f_val = this->f_val - this->f_val;
                return val;
            default:
                throw "ERR: DATA TYPE";
        }
    default:
        throw "ERR: DATA TYPE";
    }
}

std::string Value::to_str() {
    switch (this->type) {
        case ValueTypes::INT:
            return std::to_string(this->i_val);
        case ValueTypes::FLOAT:
            return std::to_string(this->f_val);
        case ValueTypes::STRING:
            return this->s_val;
        default:
            throw "Not Implemented Exception!";
    }
}