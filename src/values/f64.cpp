#include "values/f64.hpp"
using namespace common::values;

ValueF64::ValueF64() {
    this->value = std::nullopt;
}

ValueF64::ValueF64(double value) {
    this->value = value;
}

void ValueF64::show(char end) const {
    if (this->value)
        std::cout << "f64<" << this->value.value() << '>' << end;
    else
        std::cout << "f64<null>" << end;
}

bool ValueF64::isNull() const {
    return !this->value.has_value();
}

std::string ValueF64::getTypeRepr() const {
    return "f64";
}

ValueKind ValueF64::getKind() const {
    return ValueKind::F64;
}

std::optional<double> ValueF64::getValue() const {
    return this->value;
}