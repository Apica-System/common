#include "values/null.hpp"
using namespace common::values;

ValueNull::ValueNull() {

}

void ValueNull::show(char end) const {
    std::cout << "null<>" << end;
}

bool ValueNull::isNull() const {
    return true;
}

std::string ValueNull::getTypeRepr() const {
    return "null";
}

ValueKind ValueNull::getKind() const {
    return ValueKind::Null;
}