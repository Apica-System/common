#include "values/i64.hpp"
using namespace common::values;

ValueI64::ValueI64()
    : value(std::nullopt) {
    
}

ValueI64::ValueI64(int64_t value)
    : value(value) {
    
}

void ValueI64::show(char end) const {
    if (this->value)
        std::cout << "i64<" << this->value.value() << '>' << end;
    else
        std::cout << "i64<null>" << end;
}

bool ValueI64::isNull() const {
    return !this->value.has_value();
}

std::string ValueI64::getTypeRepr() const {
    return "i64";
}

ValueKind ValueI64::getKind() const {
    return ValueKind::I64;
}

std::optional<int64_t> ValueI64::getValue() const {
    return this->value;
}