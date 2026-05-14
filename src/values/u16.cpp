#include "values/u16.hpp"
using namespace common::values;

ValueU16::ValueU16()
    : value(std::nullopt) {
    
}

ValueU16::ValueU16(uint16_t value)
    : value(value) {
    
}

void ValueU16::show(char end) const {
    if (this->value)
        std::cout << "u16<" << this->value.value() << '>' << end;
    else
        std::cout << "u16<null>" << end;
}

bool ValueU16::isNull() const {
    return !this->value.has_value();
}

std::string ValueU16::getTypeRepr() const {
    return "u16";
}

ValueKind ValueU16::getKind() const {
    return ValueKind::U16;
}

std::optional<uint16_t> ValueU16::getValue() const {
    return this->value;
}