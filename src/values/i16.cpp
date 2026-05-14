#include "values/i16.hpp"
using namespace common::values;

ValueI16::ValueI16() {
    this->value = std::nullopt;
}

ValueI16::ValueI16(int16_t value) {
    this->value = value;
}

void ValueI16::show(char end) const {
    if (this->value)
        std::cout << "i16<" << this->value.value() << '>' << end;
    else
        std::cout << "i16<null>" << end;
}

bool ValueI16::isNull() const {
    return !this->value.has_value();
}

std::string ValueI16::getTypeRepr() const {
    return "i16";
}

ValueKind ValueI16::getKind() const {
    return ValueKind::I16;
}

std::optional<int16_t> ValueI16::getValue() const {
    return this->value;
}