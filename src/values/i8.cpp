#include "values/i8.hpp"
using namespace common::values;

ValueI8::ValueI8()
    : value(std::nullopt) {
    
}

ValueI8::ValueI8(int8_t value)
    : value(value) {
    
}

void ValueI8::show(char end) const {
    if (this->value)
        std::cout << "u8<" << (int16_t)this->value.value() << '>' << end;
    else
        std::cout << "u8<null>" << end;
}

bool ValueI8::isNull() const {
    return !this->value.has_value();
}

std::string ValueI8::getTypeRepr() const {
    return "i8";
}

ValueKind ValueI8::getKind() const {
    return ValueKind::I8;
}

std::optional<int8_t> ValueI8::getValue() const {
    return this->value;
}