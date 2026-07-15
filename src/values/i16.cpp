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

ValueI16::ValueI16()
    : value(std::nullopt) {
    
}

ValueI16::ValueI16(int16_t value)
    : value(value) {
    
}

void ValueI16::show(char end) const {
    if (this->value)
        std::cout << "i16<" << this->value.value() << '>' << end;
    else
        std::cout << "i16<null>" << end;
}

bool ValueI16::isNull() const {
    return !this->value.has_value();
}

std::string ValueI16::getTypeRepr() const {
    return "i16";
}

common::bytecodes::ApicaTypeBytecode ValueI16::getKind() const {
    return common::bytecodes::ApicaTypeBytecode::I16;
}

std::optional<Value*> ValueI16::add(const Value *other) const {
    switch (other->getKind()) {
        case common::bytecodes::ApicaTypeBytecode::I8: {
            const ValueI8 *i8 = static_cast<const ValueI8*>(other);
            return new ValueI16(this->value.value() + i8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I16: {
            const ValueI16 *i16 = static_cast<const ValueI16*>(other);
            return new ValueI16(this->value.value() + i16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I32: {
            const ValueI32 *i32 = static_cast<const ValueI32*>(other);
            return new ValueI32(this->value.value() + i32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I64: {
            const ValueI64 *i64 = static_cast<const ValueI64*>(other);
            return new ValueI64(this->value.value() + i64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U8: {
            const ValueU8 *u8 = static_cast<const ValueU8*>(other);
            return new ValueI16(this->value.value() + u8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U16: {
            const ValueU16 *u16 = static_cast<const ValueU16*>(other);
            return new ValueI16(this->value.value() + u16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U32: {
            const ValueU32 *u32 = static_cast<const ValueU32*>(other);
            return new ValueI32(this->value.value() + u32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U64: {
            const ValueU64 *u64 = static_cast<const ValueU64*>(other);
            return new ValueI64(this->value.value() + u64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F32: {
            const ValueF32 *f32 = static_cast<const ValueF32*>(other);
            return new ValueF32(this->value.value() + f32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F64: {
            const ValueF64 *f64 = static_cast<const ValueF64*>(other);
            return new ValueF64(this->value.value() + f64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Bool: {
            const ValueBool *boolean = static_cast<const ValueBool*>(other);
            return new ValueI16(this->value.value() + boolean->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Char: {
            const ValueChar *character = static_cast<const ValueChar*>(other);
            return new ValueI32(this->value.value() + character->getValue().value());
        }

        default: return std::nullopt;
    }
}

std::optional<Value*> ValueI16::increment() {
    return new ValueI16(this->value.value()++);
}

std::optional<Value*> ValueI16::leftIncrement() {
    return new ValueI16(++this->value.value());
}

std::optional<Value*> ValueI16::subtract(const Value *other) const {
    switch (other->getKind()) {
        case common::bytecodes::ApicaTypeBytecode::I8: {
            const ValueI8 *i8 = static_cast<const ValueI8*>(other);
            return new ValueI16(this->value.value() - i8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I16: {
            const ValueI16 *i16 = static_cast<const ValueI16*>(other);
            return new ValueI16(this->value.value() - i16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I32: {
            const ValueI32 *i32 = static_cast<const ValueI32*>(other);
            return new ValueI32(this->value.value() - i32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I64: {
            const ValueI64 *i64 = static_cast<const ValueI64*>(other);
            return new ValueI64(this->value.value() - i64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U8: {
            const ValueU8 *u8 = static_cast<const ValueU8*>(other);
            return new ValueI16(this->value.value() - u8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U16: {
            const ValueU16 *u16 = static_cast<const ValueU16*>(other);
            return new ValueI16(this->value.value() - u16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U32: {
            const ValueU32 *u32 = static_cast<const ValueU32*>(other);
            return new ValueI32(this->value.value() - u32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U64: {
            const ValueU64 *u64 = static_cast<const ValueU64*>(other);
            return new ValueI64(this->value.value() - u64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F32: {
            const ValueF32 *f32 = static_cast<const ValueF32*>(other);
            return new ValueF32(this->value.value() - f32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F64: {
            const ValueF64 *f64 = static_cast<const ValueF64*>(other);
            return new ValueF64(this->value.value() - f64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Bool: {
            const ValueBool *boolean = static_cast<const ValueBool*>(other);
            return new ValueI16(this->value.value() - boolean->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Char: {
            const ValueChar *character = static_cast<const ValueChar*>(other);
            return new ValueI32(this->value.value() - character->getValue().value());
        }

        default: return std::nullopt;
    }
}

std::optional<Value*> ValueI16::decrement() {
    return new ValueI16(this->value.value()--);
}

std::optional<Value*> ValueI16::leftDecrement() {
    return new ValueI16(--this->value.value());
}

std::optional<Value*> ValueI16::times(const Value *other) const {
    switch (other->getKind()) {
        case common::bytecodes::ApicaTypeBytecode::I8: {
            const ValueI8 *i8 = static_cast<const ValueI8*>(other);
            return new ValueI32(this->value.value() * i8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I16: {
            const ValueI16 *i16 = static_cast<const ValueI16*>(other);
            return new ValueI32(this->value.value() * i16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I32: {
            const ValueI32 *i32 = static_cast<const ValueI32*>(other);
            return new ValueI32(this->value.value() * i32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I64: {
            const ValueI64 *i64 = static_cast<const ValueI64*>(other);
            return new ValueI64(this->value.value() * i64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U8: {
            const ValueU8 *u8 = static_cast<const ValueU8*>(other);
            return new ValueI32(this->value.value() * u8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U16: {
            const ValueU16 *u16 = static_cast<const ValueU16*>(other);
            return new ValueI32(this->value.value() * u16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U32: {
            const ValueU32 *u32 = static_cast<const ValueU32*>(other);
            return new ValueI32(this->value.value() * u32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U64: {
            const ValueU64 *u64 = static_cast<const ValueU64*>(other);
            return new ValueI64(this->value.value() * u64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F32: {
            const ValueF32 *f32 = static_cast<const ValueF32*>(other);
            return new ValueF32(this->value.value() * f32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F64: {
            const ValueF64 *f64 = static_cast<const ValueF64*>(other);
            return new ValueF64(this->value.value() * f64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Bool: {
            const ValueBool *boolean = static_cast<const ValueBool*>(other);
            return new ValueI16(boolean->getValue().value() ? this->value.value() : 0);
        }

        case common::bytecodes::ApicaTypeBytecode::Char: {
            const ValueChar *character = static_cast<const ValueChar*>(other);
            return new ValueI32(this->value.value() * character->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::String: {
            if (this->value.value() <= 0)
                return new ValueString("");

            const ValueString *string = static_cast<const ValueString*>(other);
            std::string return_value;
            return_value.reserve(string->getValue().value().size() * this->value.value());

            for (int16_t i = 0; i < this->value.value(); i++)
                return_value += string->getValue().value();
            
            return new ValueString(return_value);
        }

        default: return std::nullopt;
    }
}

std::optional<Value*> ValueI16::unaryNot() const {
    return new ValueBool(this->value.has_value() ? !this->value.value() : true);
}

std::optional<Value*> ValueI16::bitwiseNot() const {
    return new ValueI16(~this->value.value());
}

std::optional<Value*> ValueI16::lessThan(const Value *other) const {
    switch (other->getKind()) {
        case common::bytecodes::ApicaTypeBytecode::I8: {
            const ValueI8 *i8 = static_cast<const ValueI8*>(other);
            return new ValueBool(this->value.value() < i8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I16: {
            const ValueI16 *i16 = static_cast<const ValueI16*>(other);
            return new ValueBool(this->value.value() < i16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I32: {
            const ValueI32 *i32 = static_cast<const ValueI32*>(other);
            return new ValueBool(this->value.value() < i32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I64: {
            const ValueI64 *i64 = static_cast<const ValueI64*>(other);
            return new ValueBool(this->value.value() < i64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U8: {
            const ValueU8 *u8 = static_cast<const ValueU8*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) < u8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U16: {
            const ValueU16 *u16 = static_cast<const ValueU16*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) < u16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U32: {
            const ValueU32 *u32 = static_cast<const ValueU32*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) < u32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U64: {
            const ValueU64 *u64 = static_cast<const ValueU64*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) < u64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F32: {
            const ValueF32 *f32 = static_cast<const ValueF32*>(other);
            return new ValueBool(this->value.value() < f32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F64: {
            const ValueF64 *f64 = static_cast<const ValueF64*>(other);
            return new ValueBool(this->value.value() < f64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Bool: {
            const ValueBool *boolean = static_cast<const ValueBool*>(other);
            return new ValueBool(this->value.value() < boolean->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Char: {
            const ValueChar *character = static_cast<const ValueChar*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) < character->getValue().value());
        }

        default: return std::nullopt;
    }
}

std::optional<Value*> ValueI16::lessOrEquals(const Value *other) const {
    switch (other->getKind()) {
        case common::bytecodes::ApicaTypeBytecode::I8: {
            const ValueI8 *i8 = static_cast<const ValueI8*>(other);
            return new ValueBool(this->value.value() <= i8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I16: {
            const ValueI16 *i16 = static_cast<const ValueI16*>(other);
            return new ValueBool(this->value.value() <= i16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I32: {
            const ValueI32 *i32 = static_cast<const ValueI32*>(other);
            return new ValueBool(this->value.value() <= i32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I64: {
            const ValueI64 *i64 = static_cast<const ValueI64*>(other);
            return new ValueBool(this->value.value() <= i64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U8: {
            const ValueU8 *u8 = static_cast<const ValueU8*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) <= u8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U16: {
            const ValueU16 *u16 = static_cast<const ValueU16*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) <= u16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U32: {
            const ValueU32 *u32 = static_cast<const ValueU32*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) <= u32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U64: {
            const ValueU64 *u64 = static_cast<const ValueU64*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) <= u64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F32: {
            const ValueF32 *f32 = static_cast<const ValueF32*>(other);
            return new ValueBool(this->value.value() <= f32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F64: {
            const ValueF64 *f64 = static_cast<const ValueF64*>(other);
            return new ValueBool(this->value.value() <= f64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Bool: {
            const ValueBool *boolean = static_cast<const ValueBool*>(other);
            return new ValueBool(this->value.value() <= boolean->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Char: {
            const ValueChar *character = static_cast<const ValueChar*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) <= character->getValue().value());
        }

        default: return std::nullopt;
    }
}

std::optional<Value*> ValueI16::greaterThan(const Value *other) const {
    switch (other->getKind()) {
        case common::bytecodes::ApicaTypeBytecode::I8: {
            const ValueI8 *i8 = static_cast<const ValueI8*>(other);
            return new ValueBool(this->value.value() > i8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I16: {
            const ValueI16 *i16 = static_cast<const ValueI16*>(other);
            return new ValueBool(this->value.value() > i16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I32: {
            const ValueI32 *i32 = static_cast<const ValueI32*>(other);
            return new ValueBool(this->value.value() > i32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I64: {
            const ValueI64 *i64 = static_cast<const ValueI64*>(other);
            return new ValueBool(this->value.value() > i64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U8: {
            const ValueU8 *u8 = static_cast<const ValueU8*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) > u8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U16: {
            const ValueU16 *u16 = static_cast<const ValueU16*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) > u16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U32: {
            const ValueU32 *u32 = static_cast<const ValueU32*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) > u32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U64: {
            const ValueU64 *u64 = static_cast<const ValueU64*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) > u64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F32: {
            const ValueF32 *f32 = static_cast<const ValueF32*>(other);
            return new ValueBool(this->value.value() > f32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F64: {
            const ValueF64 *f64 = static_cast<const ValueF64*>(other);
            return new ValueBool(this->value.value() > f64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Bool: {
            const ValueBool *boolean = static_cast<const ValueBool*>(other);
            return new ValueBool(this->value.value() > boolean->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Char: {
            const ValueChar *character = static_cast<const ValueChar*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) > character->getValue().value());
        }

        default: return std::nullopt;
    }
}

std::optional<Value*> ValueI16::greaterOrEquals(const Value *other) const {
    switch (other->getKind()) {
        case common::bytecodes::ApicaTypeBytecode::I8: {
            const ValueI8 *i8 = static_cast<const ValueI8*>(other);
            return new ValueBool(this->value.value() >= i8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I16: {
            const ValueI16 *i16 = static_cast<const ValueI16*>(other);
            return new ValueBool(this->value.value() >= i16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I32: {
            const ValueI32 *i32 = static_cast<const ValueI32*>(other);
            return new ValueBool(this->value.value() >= i32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::I64: {
            const ValueI64 *i64 = static_cast<const ValueI64*>(other);
            return new ValueBool(this->value.value() >= i64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U8: {
            const ValueU8 *u8 = static_cast<const ValueU8*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) >= u8->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U16: {
            const ValueU16 *u16 = static_cast<const ValueU16*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) >= u16->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U32: {
            const ValueU32 *u32 = static_cast<const ValueU32*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) >= u32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::U64: {
            const ValueU64 *u64 = static_cast<const ValueU64*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) >= u64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F32: {
            const ValueF32 *f32 = static_cast<const ValueF32*>(other);
            return new ValueBool(this->value.value() >= f32->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::F64: {
            const ValueF64 *f64 = static_cast<const ValueF64*>(other);
            return new ValueBool(this->value.value() >= f64->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Bool: {
            const ValueBool *boolean = static_cast<const ValueBool*>(other);
            return new ValueBool(this->value.value() >= boolean->getValue().value());
        }

        case common::bytecodes::ApicaTypeBytecode::Char: {
            const ValueChar *character = static_cast<const ValueChar*>(other);
            return new ValueBool(static_cast<uint16_t>(this->value.value()) >= character->getValue().value());
        }

        default: return std::nullopt;
    }
}

std::optional<Value*> ValueI16::convert(common::bytecodes::ApicaTypeBytecode to) const {
    if (this->value) {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::String:
                return new ValueString(std::to_string(this->value.value()));
            
            case common::bytecodes::ApicaTypeBytecode::Type:
                return new ValueType(common::bytecodes::ApicaTypeBytecode::I16);

            default: return std::nullopt;
        }
    } else {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::String:
                return new ValueString();
            
            case common::bytecodes::ApicaTypeBytecode::Type:
                return new ValueType(common::bytecodes::ApicaTypeBytecode::I16);

            default: return std::nullopt;
        }
    }
}

std::optional<Value*> ValueI16::autoConvert(common::bytecodes::ApicaTypeBytecode to) const {
    if (this->value) {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Any:
            case common::bytecodes::ApicaTypeBytecode::I16:
                return new ValueI16(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::I8:
                return new ValueI8(this->value.value());
            
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
            
            case common::bytecodes::ApicaTypeBytecode::F64:
                return new ValueF64(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::Bool:
                return new ValueBool(this->value.value());
            
            case common::bytecodes::ApicaTypeBytecode::Char:
                return new ValueChar(this->value.value());
            
            default: return std::nullopt;
        }
    } else {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Any:
            case common::bytecodes::ApicaTypeBytecode::I16:
                return new ValueI16();
            
            case common::bytecodes::ApicaTypeBytecode::I8:
                return new ValueI8();
            
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
            
            case common::bytecodes::ApicaTypeBytecode::F64:
                return new ValueF64();
            
            case common::bytecodes::ApicaTypeBytecode::Bool:
                return new ValueBool();
            
            case common::bytecodes::ApicaTypeBytecode::Char:
                return new ValueChar();
            
            default: return std::nullopt;
        }
    }
}

std::optional<int16_t> ValueI16::getValue() const {
    return this->value;
}