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
#include "elements.hpp"

using namespace common::values;

ValueF64::ValueF64()
    : value(std::nullopt) {
    
}

ValueF64::ValueF64(double value)
    : value(value) {
    
}

void ValueF64::show(char end) const {
    if (this->value)
        std::cout << "f64<" << this->value.value() << '>' << end;
    else
        std::cout << "f64<null>" << end;
}

bool ValueF64::isNull() const {
    return !this->value.has_value();
}

std::string ValueF64::getTypeRepr() const {
    return "f64";
}

common::bytecodes::ApicaTypeBytecode ValueF64::getKind() const {
    return common::bytecodes::ApicaTypeBytecode::F64;
}

std::optional<Value*> ValueF64::add(const Value *other) const {
    switch (other->getKind()) {
        case common::bytecodes::ApicaTypeBytecode::I8: {
            const ValueI8 *i8 = static_cast<const ValueI8*>(other);
            return new ValueF64(this->value.value() + i8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I16: {
            const ValueI16 *i16 = static_cast<const ValueI16*>(other);
            return new ValueF64(this->value.value() + i16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I32: {
            const ValueI32 *i32 = static_cast<const ValueI32*>(other);
            return new ValueF64(this->value.value() + i32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I64: {
            const ValueI64 *i64 = static_cast<const ValueI64*>(other);
            return new ValueF64(this->value.value() + i64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U8: {
            const ValueU8 *u8 = static_cast<const ValueU8*>(other);
            return new ValueF64(this->value.value() + u8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U16: {
            const ValueU16 *u16 = static_cast<const ValueU16*>(other);
            return new ValueF64(this->value.value() + u16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U32: {
            const ValueU32 *u32 = static_cast<const ValueU32*>(other);
            return new ValueF64(this->value.value() + u32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U64: {
            const ValueU64 *u64 = static_cast<const ValueU64*>(other);
            return new ValueF64(this->value.value() + u64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F32: {
            const ValueF32 *f32 = static_cast<const ValueF32*>(other);
            return new ValueF64(this->value.value() + f32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F64: {
            const ValueF64 *f64 = static_cast<const ValueF64*>(other);
            return new ValueF64(this->value.value() + f64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Bool: {
            const ValueBool *boolean = static_cast<const ValueBool*>(other);
            return new ValueF64(this->value.value() + boolean->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Char: {
            const ValueChar *character = static_cast<const ValueChar*>(other);
            return new ValueF64(this->value.value() + character->getValue().value());
        }

        default: return std::nullopt;
    }
}

std::optional<Value*> ValueF64::increment() {
    return new ValueF64(this->value.value()++);
}

std::optional<Value*> ValueF64::leftIncrement() {
    return new ValueF64(++this->value.value());
}

std::optional<Value*> ValueF64::subtract(const Value *other) const {
    switch (other->getKind()) {
        case common::bytecodes::ApicaTypeBytecode::I8: {
            const ValueI8 *i8 = static_cast<const ValueI8*>(other);
            return new ValueF64(this->value.value() - i8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I16: {
            const ValueI16 *i16 = static_cast<const ValueI16*>(other);
            return new ValueF64(this->value.value() - i16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I32: {
            const ValueI32 *i32 = static_cast<const ValueI32*>(other);
            return new ValueF64(this->value.value() - i32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I64: {
            const ValueI64 *i64 = static_cast<const ValueI64*>(other);
            return new ValueF64(this->value.value() - i64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U8: {
            const ValueU8 *u8 = static_cast<const ValueU8*>(other);
            return new ValueF64(this->value.value() - u8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U16: {
            const ValueU16 *u16 = static_cast<const ValueU16*>(other);
            return new ValueF64(this->value.value() - u16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U32: {
            const ValueU32 *u32 = static_cast<const ValueU32*>(other);
            return new ValueF64(this->value.value() - u32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U64: {
            const ValueU64 *u64 = static_cast<const ValueU64*>(other);
            return new ValueF64(this->value.value() - u64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F32: {
            const ValueF32 *f32 = static_cast<const ValueF32*>(other);
            return new ValueF64(this->value.value() - f32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F64: {
            const ValueF64 *f64 = static_cast<const ValueF64*>(other);
            return new ValueF64(this->value.value() - f64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Bool: {
            const ValueBool *boolean = static_cast<const ValueBool*>(other);
            return new ValueF64(this->value.value() - boolean->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Char: {
            const ValueChar *character = static_cast<const ValueChar*>(other);
            return new ValueF64(this->value.value() - character->getValue().value());
        }

        default: return std::nullopt;
    }
}
std::optional<Value*> ValueF64::decrement() {
    return new ValueF64(this->value.value()--);
}

std::optional<Value*> ValueF64::leftDecrement() {
    return new ValueF64(--this->value.value());
}

std::optional<Value*> ValueF64::unaryNot() const {
    return new ValueBool(this->value.has_value() ? !this->value.value() : true);
}

std::optional<Value*> ValueF64::bitwiseNot() const {
    return std::nullopt;
}

std::optional<Value*> ValueF64::convert(common::bytecodes::ApicaTypeBytecode to) const {
    if (this->value) {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Char:
                return new ValueChar(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::String:
                return new ValueString(std::to_string(this->value.value()));
            
            case common::bytecodes::ApicaTypeBytecode::Type:
                return new ValueType(common::bytecodes::ApicaTypeBytecode::F64);

            default: return std::nullopt;
        }
    } else {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Char:
                return new ValueChar();
            
            case common::bytecodes::ApicaTypeBytecode::String:
                return new ValueString();
            
            case common::bytecodes::ApicaTypeBytecode::Type:
                return new ValueType(common::bytecodes::ApicaTypeBytecode::F64);

            default: return std::nullopt;
        }
    }
}

std::optional<Value*> ValueF64::autoConvert(common::bytecodes::ApicaTypeBytecode to) const {
    if (this->value) {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Any:
            case common::bytecodes::ApicaTypeBytecode::F64:
                return new ValueF64(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::I8:
                return new ValueI8(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::I16:
                return new ValueI16(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::I32:
                return new ValueI32(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::I64:
                return new ValueI64(this->value.value());
                
            case common::bytecodes::ApicaTypeBytecode::U8:
                return new ValueU8(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::U16:
                return new ValueU16(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::U32:
                return new ValueU32(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::U64:
                return new ValueU64(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::F32:
                return new ValueF32(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::Bool:
                return new ValueBool(this->value.value());
            
            default: return std::nullopt;
        }
    } else {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Any:
            case common::bytecodes::ApicaTypeBytecode::F64:
                return new ValueF64();
            
            case common::bytecodes::ApicaTypeBytecode::I8:
                return new ValueI8();
            
            case common::bytecodes::ApicaTypeBytecode::I16:
                return new ValueI16();
            
            case common::bytecodes::ApicaTypeBytecode::I32:
                return new ValueI32();
            
            case common::bytecodes::ApicaTypeBytecode::I64:
                return new ValueI64();
                
            case common::bytecodes::ApicaTypeBytecode::U8:
                return new ValueU8();
            
            case common::bytecodes::ApicaTypeBytecode::U16:
                return new ValueU16();
            
            case common::bytecodes::ApicaTypeBytecode::U32:
                return new ValueU32();
            
            case common::bytecodes::ApicaTypeBytecode::U64:
                return new ValueU64();
            
            case common::bytecodes::ApicaTypeBytecode::F32:
                return new ValueF32();
            
            case common::bytecodes::ApicaTypeBytecode::Bool:
                return new ValueBool();
            
            default: return std::nullopt;
        }
    }
}

std::optional<double> ValueF64::getValue() const {
    return this->value;
}