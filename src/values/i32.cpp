#include "values/i32.hpp"
using namespace common::values;

ValueI32::ValueI32()
    : value(std::nullopt) {
    
}

ValueI32::ValueI32(int32_t value)
    : value(value) {
    
}

void ValueI32::show(char end) const {
    if (this->value)
        std::cout << "i32<" << this->value.value() << '>' << end;
    else
        std::cout << "i32<null>" << end;
}

bool ValueI32::isNull() const {
    return !this->value.has_value();
}

std::string ValueI32::getTypeRepr() const {
    return "i32";
}

ValueKind ValueI32::getKind() const {
    return ValueKind::I32;
}

std::optional<int32_t> ValueI32::getValue() const {
    return this->value;
}