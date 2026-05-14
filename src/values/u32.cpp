#include "values/u32.hpp"
using namespace common::values;

ValueU32::ValueU32() {
    this->value = std::nullopt;
}

ValueU32::ValueU32(uint32_t value) {
    this->value = value;
}

void ValueU32::show(char end) const {
    if (this->value)
        std::cout << "u32<" << this->value.value() << '>' << end;
    else
        std::cout << "u32<null>" << end;
}

bool ValueU32::isNull() const {
    return !this->value.has_value();
}

std::string ValueU32::getTypeRepr() const {
    return "u32";
}

ValueKind ValueU32::getKind() const {
    return ValueKind::U32;
}

std::optional<uint32_t> ValueU32::getValue() const {
    return this->value;
}