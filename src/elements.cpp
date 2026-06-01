#include "elements.hpp"
#include "values/null.hpp"
#include "values/type.hpp"
#include "values/error.hpp"

using namespace common::elements;

Element::Element(uint8_t modifier, values::Value *value)
    : modifier(modifier), value(value) {
    
}

Element::~Element() {
    if (this->value) delete this->value;
}

Element Element::createNull() {
    return Element(ElementModifier::None, new values::ValueNull());
}

uint8_t Element::getModifier() const {
    return this->modifier;
}

void Element::addModifier(ElementModifier modifier) {
    this->modifier |= modifier;
}

common::values::Value *Element::getValue() const {
    return this->value;
}

bool Element::isErrorOrController() const {
    return this->modifier >= ElementModifier::Error && this->modifier <= ElementModifier::Return;
}

Element Element::add(const Element &other) const {
    if (this->value->isNull() || other.value->isNull()) {
        return Element(
            ElementModifier::Error,
            common::values::Value::nullOperationError("+", false)
        );
    }

    std::optional<common::values::Value*> result = this->value->add(other.getValue());
    return result.has_value()
        ? Element(
            ElementModifier::None,
            result.value()
        ) : Element(
            ElementModifier::Error,
            common::values::Value::binaryOperationError("+", this->value->getTypeRepr(), other.value->getTypeRepr())
        );
}

Element Element::increment() {
    if (this->value->isNull()) {
        return Element(
            ElementModifier::Error,
            common::values::Value::nullOperationError("right ++", true)
        );
    }

    std::optional<common::values::Value*> result = this->value->increment();
    return result.has_value()
        ? Element(
            ElementModifier::None,
            result.value()
        ) : Element(
            ElementModifier::Error,
            common::values::Value::unaryOperationError("right ++", this->value->getTypeRepr())
        );
}

Element Element::subtract(const Element &other) const {
    if (this->value->isNull() || other.value->isNull()) {
        return Element(
            ElementModifier::Error,
            common::values::Value::nullOperationError("-", false)
        );
    }

    std::optional<common::values::Value*> result = this->value->subtract(other.getValue());
    return result.has_value()
        ? Element(
            ElementModifier::None,
            result.value()
        ) : Element(
            ElementModifier::Error,
            common::values::Value::binaryOperationError("-", this->value->getTypeRepr(), other.value->getTypeRepr())
        );
}

Element Element::decrement() {
    if (this->value->isNull()) {
        return Element(
            ElementModifier::Error,
            common::values::Value::nullOperationError("right --", true)
        );
    }

    std::optional<common::values::Value*> result = this->value->decrement();
    return result.has_value()
        ? Element(
            ElementModifier::None,
            result.value()
        ) : Element(
            ElementModifier::Error,
            common::values::Value::unaryOperationError("right --", this->value->getTypeRepr())
        );
}

Element Element::unaryNot() const {
    std::optional<common::values::Value*> result = this->value->unaryNot();
    return result.has_value()
        ? Element(
            ElementModifier::None,
            result.value()
        ) : Element(
            ElementModifier::Error,
            common::values::Value::unaryOperationError("!", this->value->getTypeRepr())
        );
}

bool Element::shouldConvert(common::bytecodes::ApicaTypeBytecode to) {
    if (this->isErrorOrController() || to == common::bytecodes::ApicaTypeBytecode::Any)
        return false;
    
    return this->value->getKind() != to;
}

Element Element::convert(common::bytecodes::ApicaTypeBytecode to) {
    std::optional<common::values::Value*> converted = this->value->convert(to);
    return converted.has_value()
        ? Element(
            ElementModifier::None,
            converted.value()
        ) : Element(
            ElementModifier::Error,
            common::values::Value::binaryOperationError("as", this->value->getTypeRepr(), common::values::ValueType::getKindRepr(to))
        );
}

Element Element::autoConvert(common::bytecodes::ApicaTypeBytecode to) {
    std::optional<common::values::Value*> converted = this->value->autoConvert(to);
    return converted.has_value()
        ? Element(
            ElementModifier::None,
            converted.value()
        ) : Element(
            ElementModifier::Error,
            common::values::Value::binaryOperationError("auto-as", this->value->getTypeRepr(), common::values::ValueType::getKindRepr(to))
        );
}