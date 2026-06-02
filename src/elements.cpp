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

Element *Element::add(const Element &other) const {
    if (this->value->isNull() || other.value->isNull()) {
        return new Element(
            ElementModifier::Error,
            common::values::Value::nullOperationError("+", false)
        );
    }

    std::optional<common::values::Value*> result = this->value->add(other.getValue());
    return result.has_value()
        ? new Element(
            ElementModifier::None,
            result.value()
        ) : new Element(
            ElementModifier::Error,
            common::values::Value::binaryOperationError("+", this->value->getTypeRepr(), other.value->getTypeRepr())
        );
}

Element *Element::increment() {
    if (this->value->isNull()) {
        return new Element(
            ElementModifier::Error,
            common::values::Value::nullOperationError("right ++", true)
        );
    }

    std::optional<common::values::Value*> result = this->value->increment();
    return result.has_value()
        ? new Element(
            ElementModifier::None,
            result.value()
        ) : new Element(
            ElementModifier::Error,
            common::values::Value::unaryOperationError("right ++", this->value->getTypeRepr())
        );
}

Element *Element::subtract(const Element &other) const {
    if (this->value->isNull() || other.value->isNull()) {
        return new Element(
            ElementModifier::Error,
            common::values::Value::nullOperationError("-", false)
        );
    }

    std::optional<common::values::Value*> result = this->value->subtract(other.getValue());
    return result.has_value()
        ? new Element(
            ElementModifier::None,
            result.value()
        ) : new Element(
            ElementModifier::Error,
            common::values::Value::binaryOperationError("-", this->value->getTypeRepr(), other.value->getTypeRepr())
        );
}

Element *Element::decrement() {
    if (this->value->isNull()) {
        return new Element(
            ElementModifier::Error,
            common::values::Value::nullOperationError("right --", true)
        );
    }

    std::optional<common::values::Value*> result = this->value->decrement();
    return result.has_value()
        ? new Element(
            ElementModifier::None,
            result.value()
        ) : new Element(
            ElementModifier::Error,
            common::values::Value::unaryOperationError("right --", this->value->getTypeRepr())
        );
}

Element *Element::unaryNot() const {
    std::optional<common::values::Value*> result = this->value->unaryNot();
    return result.has_value()
        ? new Element(
            ElementModifier::None,
            result.value()
        ) : new Element(
            ElementModifier::Error,
            common::values::Value::unaryOperationError("!", this->value->getTypeRepr())
        );
}

Element *Element::checkConvert(common::bytecodes::ApicaTypeBytecode to) {
    if (this->isErrorOrController() || this->value->getKind() == to)
        return this;
    
    if (to == common::bytecodes::ApicaTypeBytecode::Any) {
        this->modifier |= ElementModifier::Any;
        return this;
    }
    
    return this->autoConvert(to);
}

Element *Element::convert(common::bytecodes::ApicaTypeBytecode to) {
    std::optional<common::values::Value*> auto_converted = this->value->autoConvert(to);
    if (auto_converted)
        return new Element(ElementModifier::None, auto_converted.value());

    std::optional<common::values::Value*> converted = this->value->convert(to);
    return converted.has_value()
        ? new Element(
            ElementModifier::None,
            converted.value()
        ) : new Element(
            ElementModifier::Error,
            common::values::Value::binaryOperationError("as", this->value->getTypeRepr(), common::values::ValueType::getKindRepr(to))
        );
}

Element *Element::autoConvert(common::bytecodes::ApicaTypeBytecode to) {
    std::optional<common::values::Value*> converted = this->value->autoConvert(to);
    return converted.has_value()
        ? new Element(
            ElementModifier::None,
            converted.value()
        ) : new Element(
            ElementModifier::Error,
            common::values::Value::binaryOperationError("auto-as", this->value->getTypeRepr(), common::values::ValueType::getKindRepr(to))
        );
}