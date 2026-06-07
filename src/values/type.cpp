#include "values/type.hpp"
#include "values/bool.hpp"
#include "values/string.hpp"
#include "elements.hpp"

using namespace common::values;

ValueType::ValueType()
    : type_kind(std::nullopt) {
    
}

ValueType::ValueType(common::bytecodes::ApicaTypeBytecode type_kind)
    : type_kind(type_kind) {
    
}

std::string ValueType::getKindRepr(common::bytecodes::ApicaTypeBytecode kind) {
    switch (kind) {
        case common::bytecodes::ApicaTypeBytecode::Null: return "null";

        case common::bytecodes::ApicaTypeBytecode::I8: return "i8";
        case common::bytecodes::ApicaTypeBytecode::I16: return "i16";
        case common::bytecodes::ApicaTypeBytecode::I32: return "i32";
        case common::bytecodes::ApicaTypeBytecode::I64: return "i64";
        case common::bytecodes::ApicaTypeBytecode::U8: return "u8";
        case common::bytecodes::ApicaTypeBytecode::U16: return "u16";
        case common::bytecodes::ApicaTypeBytecode::U32: return "u32";
        case common::bytecodes::ApicaTypeBytecode::U64: return "u64";
        case common::bytecodes::ApicaTypeBytecode::F32: return "f32";
        case common::bytecodes::ApicaTypeBytecode::F64: return "f64";
        case common::bytecodes::ApicaTypeBytecode::Bool: return "bool";

        case common::bytecodes::ApicaTypeBytecode::Char: return "char";
        case common::bytecodes::ApicaTypeBytecode::String: return "string";

        case common::bytecodes::ApicaTypeBytecode::Error: return "error";
        case common::bytecodes::ApicaTypeBytecode::Type: return "type";

        default: return "???";
    }
}

void ValueType::show(char end) const {
    if (this->type_kind) {
        std::cout << "type<" << ValueType::getKindRepr(this->type_kind.value()) << '>' << end;
    } else {
        std::cout << "type<null>" << end;
    }
}

bool ValueType::isNull() const {
    return !this->type_kind.has_value();
}

std::string ValueType::getTypeRepr() const {
    return "type";
}

common::bytecodes::ApicaTypeBytecode ValueType::getKind() const {
    return common::bytecodes::ApicaTypeBytecode::Type;
}

std::optional<Value*> ValueType::add(const Value *) const {
    return std::nullopt;
}

std::optional<Value*> ValueType::increment() {
    return std::nullopt;
}

std::optional<Value*> ValueType::leftIncrement() {
    return std::nullopt;
}

std::optional<Value*> ValueType::subtract(const Value *) const {
    return std::nullopt;
}

std::optional<Value*> ValueType::decrement() {
    return std::nullopt;
}

std::optional<Value*> ValueType::leftDecrement() {
    return std::nullopt;
}

std::optional<Value*> ValueType::unaryNot() const {
    return new ValueBool(this->type_kind.has_value() ? this->type_kind.value() == common::bytecodes::ApicaTypeBytecode::Null : true);
}

std::optional<Value*> ValueType::bitwiseNot() const {
    return std::nullopt;
}

std::optional<Value*> ValueType::convert(common::bytecodes::ApicaTypeBytecode to) const {
    if (this->type_kind) {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Bool:
                return new ValueBool(this->type_kind.value() != common::bytecodes::ApicaTypeBytecode::Null);
            
            case common::bytecodes::ApicaTypeBytecode::String: {
                std::string result("<");
                result += ValueType::getKindRepr(this->type_kind.value());
                result += '>';

                return new ValueString(result);
            }

            default: return std::nullopt;
        }
    } else {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Bool:
                return new ValueBool();
            
            case common::bytecodes::ApicaTypeBytecode::String:
                return new ValueString();
            
            default: return std::nullopt;
        }
    }
}

std::optional<Value*> ValueType::autoConvert(common::bytecodes::ApicaTypeBytecode to) const {
    if (this->type_kind) {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Any:
                return new ValueType(this->type_kind.value());
            
            case common::bytecodes::ApicaTypeBytecode::Type:
                return new ValueType(common::bytecodes::ApicaTypeBytecode::Type);

            default: return std::nullopt;
        }
    } else {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Any:
                return new ValueType();
            
            case common::bytecodes::ApicaTypeBytecode::Type:
                return new ValueType(common::bytecodes::ApicaTypeBytecode::Type);

            default: return std::nullopt;
        }
    }
}

std::optional<common::bytecodes::ApicaTypeBytecode> ValueType::getTypeKind() const {
    return this->type_kind;
}