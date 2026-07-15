#include "values/bool.hpp"
#include "values/i8.hpp"
#include "values/i16.hpp"
#include "values/i32.hpp"
#include "values/i64.hpp"
#include "values/u8.hpp"
#include "values/u16.hpp"
#include "values/u32.hpp"
#include "values/u64.hpp"
#include "values/f32.hpp"
#include "values/f64.hpp"
#include "values/char.hpp"
#include "values/string.hpp"
#include "values/type.hpp"
#include "values/error.hpp"
#include "elements.hpp"

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

common::bytecodes::ApicaTypeBytecode ValueString::getKind() const {
    return common::bytecodes::ApicaTypeBytecode::String;
}

std::optional<Value*> ValueString::add(const Value *other) const {
    switch (other->getKind()) {
        case common::bytecodes::ApicaTypeBytecode::I8: {
            const ValueI8 *i8 = static_cast<const ValueI8*>(other);
            return new ValueString(this->value.value() + std::to_string(static_cast<int16_t>(i8->getValue().value())));
        }

        case common::bytecodes::ApicaTypeBytecode::I16: {
            const ValueI16 *i16 = static_cast<const ValueI16*>(other);
            return new ValueString(this->value.value() + std::to_string(i16->getValue().value()));
        }

        case common::bytecodes::ApicaTypeBytecode::I32: {
            const ValueI32 *i32 = static_cast<const ValueI32*>(other);
            return new ValueString(this->value.value() + std::to_string(i32->getValue().value()));
        }

        case common::bytecodes::ApicaTypeBytecode::I64: {
            const ValueI64 *i64 = static_cast<const ValueI64*>(other);
            return new ValueString(this->value.value() + std::to_string(i64->getValue().value()));
        }

        case common::bytecodes::ApicaTypeBytecode::U8: {
            const ValueU8 *u8 = static_cast<const ValueU8*>(other);
            return new ValueString(this->value.value() + std::to_string(static_cast<uint16_t>(u8->getValue().value())));
        }

        case common::bytecodes::ApicaTypeBytecode::U16: {
            const ValueU16 *u16 = static_cast<const ValueU16*>(other);
            return new ValueString(this->value.value() + std::to_string(u16->getValue().value()));
        }

        case common::bytecodes::ApicaTypeBytecode::U32: {
            const ValueU32 *u32 = static_cast<const ValueU32*>(other);
            return new ValueString(this->value.value() + std::to_string(u32->getValue().value()));
        }

        case common::bytecodes::ApicaTypeBytecode::U64: {
            const ValueU64 *u64 = static_cast<const ValueU64*>(other);
            return new ValueString(this->value.value() + std::to_string(u64->getValue().value()));
        }

        case common::bytecodes::ApicaTypeBytecode::F32: {
            const ValueF32 *f32 = static_cast<const ValueF32*>(other);
            return new ValueString(this->value.value() + std::to_string(f32->getValue().value()));
        }

        case common::bytecodes::ApicaTypeBytecode::F64: {
            const ValueF64 *f64 = static_cast<const ValueF64*>(other);
            return new ValueString(this->value.value() + std::to_string(f64->getValue().value()));
        }

        case common::bytecodes::ApicaTypeBytecode::Bool: {
            const ValueBool *boolean = static_cast<const ValueBool*>(other);
            return new ValueString(this->value.value() + (boolean->getValue().value() ? "true" : "false"));
        }

        case common::bytecodes::ApicaTypeBytecode::Char: {
            const ValueChar *character = static_cast<const ValueChar*>(other);
            char text[5] = { 0 };
            unsigned char length = 0;
            uint32_t value = character->getValue().value();

            if (value & 0xFF000000) text[length++] = (value >> 24) & 0xFF;
            if (value & 0x00FF0000) text[length++] = (value >> 16) & 0xFF;
            if (value & 0x0000FF00) text[length++] = (value >> 8) & 0xFF;
            if (value & 0x000000FF) text[length++] = value & 0xFF;
            return new ValueString(this->value.value() + text);
        }

        case common::bytecodes::ApicaTypeBytecode::String: {
            const ValueString *str = static_cast<const ValueString*>(other);
            return new ValueString(this->value.value() + str->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Error: {
            const ValueError *error = static_cast<const ValueError*>(other);
            if (error->getDetails())
                return new ValueString(this->value.value() + error->getName().value() + ": " + error->getDetails().value());
            else
                return new ValueString(this->value.value() + error->getName().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Type: {
            const ValueType *type = static_cast<const ValueType*>(other);
            return new ValueString(this->value.value() + '<' + ValueType::getKindRepr(type->getTypeKind().value()) + '>');
        }

        default: return std::nullopt;
    }
}

std::optional<Value*> ValueString::increment() {
    return std::nullopt;
}

std::optional<Value*> ValueString::leftIncrement() {
    return std::nullopt;
}

std::optional<Value*> ValueString::subtract(const Value *) const {
    return std::nullopt;
}

std::optional<Value*> ValueString::decrement() {
    return std::nullopt;
}

std::optional<Value*> ValueString::leftDecrement() {
    return std::nullopt;
}

std::optional<Value*> ValueString::times(const Value *other) const {
    switch (other->getKind()) {
        case common::bytecodes::ApicaTypeBytecode::I8: {
            int8_t count = static_cast<const ValueI8*>(other)->getValue().value();
            if (count <= 0)
                return new ValueString("");

            std::string return_value;
            return_value.reserve(this->value.value().size() * count);

            for (int8_t i = 0; i < count; i++)
                return_value += this->value.value();
            
            return new ValueString(return_value);
        }

        case common::bytecodes::ApicaTypeBytecode::I16: {
            int16_t count = static_cast<const ValueI16*>(other)->getValue().value();
            if (count <= 0)
                return new ValueString("");

            std::string return_value;
            return_value.reserve(this->value.value().size() * count);

            for (int16_t i = 0; i < count; i++)
                return_value += this->value.value();
            
            return new ValueString(return_value);
        }

        case common::bytecodes::ApicaTypeBytecode::I32: {
            int32_t count = static_cast<const ValueI32*>(other)->getValue().value();
            if (count <= 0)
                return new ValueString("");

            std::string return_value;
            return_value.reserve(this->value.value().size() * count);

            for (int32_t i = 0; i < count; i++)
                return_value += this->value.value();
            
            return new ValueString(return_value);
        }

        case common::bytecodes::ApicaTypeBytecode::I64: {
            int64_t count = static_cast<const ValueI64*>(other)->getValue().value();
            if (count <= 0)
                return new ValueString("");

            std::string return_value;
            return_value.reserve(this->value.value().size() * count);

            for (int64_t i = 0; i < count; i++)
                return_value += this->value.value();
            
            return new ValueString(return_value);
        }

        case common::bytecodes::ApicaTypeBytecode::U8: {
            uint8_t count = static_cast<const ValueU8*>(other)->getValue().value();
            std::string return_value;
            return_value.reserve(this->value.value().size() * count);

            for (uint8_t i = 0; i < count; i++)
                return_value += this->value.value();
            
            return new ValueString(return_value);
        }

        case common::bytecodes::ApicaTypeBytecode::U16: {
            uint16_t count = static_cast<const ValueU16*>(other)->getValue().value();
            std::string return_value;
            return_value.reserve(this->value.value().size() * count);

            for (uint16_t i = 0; i < count; i++)
                return_value += this->value.value();
            
            return new ValueString(return_value);
        }

        case common::bytecodes::ApicaTypeBytecode::U32: {
            uint32_t count = static_cast<const ValueU32*>(other)->getValue().value();
            std::string return_value;
            return_value.reserve(this->value.value().size() * count);

            for (uint32_t i = 0; i < count; i++)
                return_value += this->value.value();
            
            return new ValueString(return_value);
        }

        case common::bytecodes::ApicaTypeBytecode::U64: {
            uint64_t count = static_cast<const ValueU64*>(other)->getValue().value();
            std::string return_value;
            return_value.reserve(this->value.value().size() * count);

            for (uint64_t i = 0; i < count; i++)
                return_value += this->value.value();
            
            return new ValueString(return_value);
        }

        default: return std::nullopt;
    }
}

std::optional<Value*> ValueString::unaryNot() const {
    return new ValueBool(this->value.has_value() ? this->value.value().empty() : true);
}

std::optional<Value*> ValueString::bitwiseNot() const {
    return std::nullopt;
}

std::optional<Value*> ValueString::lessThan(const Value *) const {
    return std::nullopt;
}

std::optional<Value*> ValueString::lessOrEquals(const Value *) const {
    return std::nullopt;
}

std::optional<Value*> ValueString::greaterThan(const Value *) const {
    return std::nullopt;
}

std::optional<Value*> ValueString::greaterOrEquals(const Value *) const {
    return std::nullopt;
}

std::optional<Value*> ValueString::convert(common::bytecodes::ApicaTypeBytecode to) const {
    if (this->value) {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Bool:
                return new ValueBool(!this->value.value().empty());
            
            case common::bytecodes::ApicaTypeBytecode::Type:
                return new ValueType(common::bytecodes::ApicaTypeBytecode::String);
            
            default: return std::nullopt;
        }
    } else {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Bool:
                return new ValueBool();
            
            case common::bytecodes::ApicaTypeBytecode::Type:
                return new ValueType(common::bytecodes::ApicaTypeBytecode::String);
            
            default: return std::nullopt;
        }
    }
}

std::optional<Value*> ValueString::autoConvert(common::bytecodes::ApicaTypeBytecode to) const {
    if (this->value) {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Any:
            case common::bytecodes::ApicaTypeBytecode::String:
                return new ValueString(this->value.value());
            
            default: return std::nullopt;
        }
    } else {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Any:
            case common::bytecodes::ApicaTypeBytecode::String:
                return new ValueString();

            default: return std::nullopt;
        }
    }
}

std::optional<std::string> ValueString::getValue() const {
    return this->value;
}