#include "values/char.hpp"
using namespace common::values;

ValueChar::ValueChar() {
    this->value = std::nullopt;
}

ValueChar::ValueChar(uint32_t value) {
    this->value = value;
}

void ValueChar::show(char end) const {
    if (this->value) {
        char text[5] = { 0 };
        unsigned char length = 0;
        uint32_t value = this->value.value();

        if (value & 0xFF000000) text[length++] = (value >> 24) & 0xFF;
        if (value & 0x00FF0000) text[length++] = (value >> 16) & 0xFF;
        if (value & 0x0000FF00) text[length++] = (value >> 8) & 0xFF;
        if (value & 0x000000FF) text[length++] = value & 0xFF;

        std::cout << "char<" << text << '>' << end;
    } else {
        std::cout << "char<null>" << end;
    }
}

bool ValueChar::isNull() const {
    return !this->value.has_value();
}

std::string ValueChar::getTypeRepr() const {
    return "char";
}

ValueKind ValueChar::getKind() const {
    return ValueKind::Char;
}

std::optional<uint32_t> ValueChar::getValue() const {
    return this->value;
}