#include "values/bool.hpp"
using namespace common::values;

ValueBool::ValueBool()
    : value(std::nullopt) {

}

ValueBool::ValueBool(bool value)
    : value(value) {
    
}

void ValueBool::show(char end) const {
    if (this->value)
        std::cout << "bool<" << (this->value.value() ? "true" : "false") << '>' << end;
    else
        std::cout << "bool<null>" << end;
}

bool ValueBool::isNull() const {
    return !this->value.has_value();
}

std::string ValueBool::getTypeRepr() const {
    return "bool";
}

ValueKind ValueBool::getKind() const {
    return ValueKind::Bool;
}

std::optional<bool> ValueBool::getValue() const {
    return this->value;
}