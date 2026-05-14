#include "values/string.hpp"
using namespace common::values;

ValueString::ValueString()
    : value(std::nullopt) {
    
}

ValueString::ValueString(const std::string &value)
    : value(value) {
    
}

void ValueString::show(char end) const {
    if (this->value)
        std::cout << "string<" << this->value.value() << '>' << end;
    else
        std::cout << "string<null>" << end;
}

bool ValueString::isNull() const {
    return !this->value.has_value();
}

std::string ValueString::getTypeRepr() const {
    return "string";
}

ValueKind ValueString::getKind() const {
    return ValueKind::String;
}

std::optional<std::string> ValueString::getValue() const {
    return this->value;
}