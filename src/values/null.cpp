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
#include "values/null.hpp"
#include "values/error.hpp"
#include "elements.hpp"

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

common::bytecodes::ApicaTypeBytecode ValueNull::getKind() const {
    return common::bytecodes::ApicaTypeBytecode::Null;
}

std::optional<Value*> ValueNull::add(const Value *) const {
    return std::nullopt;
}

std::optional<Value*> ValueNull::increment() {
    return std::nullopt;
}

std::optional<Value*> ValueNull::subtract(const Value *) const {
    return std::nullopt;
}

std::optional<Value*> ValueNull::decrement() {
    return std::nullopt;
}

std::optional<Value*> ValueNull::unaryNot() const {
    return new ValueBool(true);
}

std::optional<Value*> ValueNull::convert(common::bytecodes::ApicaTypeBytecode) const {
    return std::nullopt; // null is AUTOMATICALLY converted
}

std::optional<Value*> ValueNull::autoConvert(common::bytecodes::ApicaTypeBytecode to) const {
    switch (to) {
        case common::bytecodes::ApicaTypeBytecode::Any:
        case common::bytecodes::ApicaTypeBytecode::Null:
            return new ValueNull();

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

        case common::bytecodes::ApicaTypeBytecode::F64:
            return new ValueF64();

        case common::bytecodes::ApicaTypeBytecode::Bool:
            return new ValueBool();

        case common::bytecodes::ApicaTypeBytecode::Char:
            return new ValueChar();

        case common::bytecodes::ApicaTypeBytecode::String:
            return new ValueString();

        case common::bytecodes::ApicaTypeBytecode::Error:
            return new ValueError();

        case common::bytecodes::ApicaTypeBytecode::Type:
            return new ValueType(common::bytecodes::ApicaTypeBytecode::Null);
        
        default: return std::nullopt;
    }
}