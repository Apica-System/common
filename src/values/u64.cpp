#include "values/u64.hpp"
using namespace common::values;

ValueU64::ValueU64() {
    this->value = std::nullopt;
}

ValueU64::ValueU64(uint64_t value) {
    this->value = value;
}

void ValueU64::show(char end) const {
    if (this->value)
        std::cout << "u64<" << this->value.value() << '>' << end;
    else
        std::cout << "u64<null>" << end;
}

bool ValueU64::isNull() const {
    return !this->value.has_value();
}

std::string ValueU64::getTypeRepr() const {
    return "u64";
}

ValueKind ValueU64::getKind() const {
    return ValueKind::U64;
}

std::optional<uint64_t> ValueU64::getValue() const {
    return this->value;
}