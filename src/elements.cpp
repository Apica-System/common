#include "elements.hpp"
#include "values/null.hpp"
using namespace common::elements;

Element::Element(ElementModifier modifier, values::Value *value)
    : modifier(modifier), value(value) {
    
}

Element::~Element() {
    if (this->value) delete this->value;
}

Element Element::createNull() {
    return Element(ElementModifier::None, new values::ValueNull());
}

ElementModifier Element::getModifier() const {
    return this->modifier;
}

common::values::Value *Element::getValue() const {
    return this->value;
}

bool Element::isErrorOrController() const {
    return this->modifier >= ElementModifier::Error && this->modifier <= ElementModifier::Return;
}