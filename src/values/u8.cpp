#include "values/u8.hpp"
using namespace common::values;

ValueU8::ValueU8() {
    this->value = std::nullopt;
}

ValueU8::ValueU8(uint8_t value) {
    this->value = value;
}

void ValueU8::show(char end) const {
    if (this->value)
        std::cout << "u8<" << (uint16_t)this->value.value() << '>' << end;
    else
        std::cout << "u8<null>" << end;
}

bool ValueU8::isNull() const {
    return !this->value.has_value();
}

std::string ValueU8::getTypeRepr() const {
    return "u8";
}

ValueKind ValueU8::getKind() const {
    return ValueKind::U8;
}

std::optional<uint8_t> ValueU8::getValue() const {
    return this->value;
}