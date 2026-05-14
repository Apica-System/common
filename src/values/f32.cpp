#include "values/f32.hpp"
using namespace common::values;

ValueF32::ValueF32() {
    this->value = std::nullopt;
}

ValueF32::ValueF32(float value) {
    this->value = value;
}

void ValueF32::show(char end) const {
    if (this->value)
        std::cout << "f32<" << this->value.value() << '>' << end;
    else
        std::cout << "f32<null>" << end;
}

bool ValueF32::isNull() const {
    return !this->value.has_value();
}

std::string ValueF32::getTypeRepr() const {
    return "f32";
}

ValueKind ValueF32::getKind() const {
    return ValueKind::F32;
}

std::optional<float> ValueF32::getValue() const {
    return this->value;
}