#include "values/type.hpp"
using namespace common::values;

ValueType::ValueType() {
    this->type_kind = std::nullopt;
}

ValueType::ValueType(ValueKind type_kind) {
    this->type_kind = type_kind;
}

std::string ValueType::getKindRepr(ValueKind kind) {
    switch (kind) {
        case ValueKind::Null: return "null";

        case ValueKind::I8: return "i8";
        case ValueKind::I16: return "i16";
        case ValueKind::I32: return "i32";
        case ValueKind::I64: return "i64";
        case ValueKind::U8: return "u8";
        case ValueKind::U16: return "u16";
        case ValueKind::U32: return "u32";
        case ValueKind::U64: return "u64";
        case ValueKind::F32: return "f32";
        case ValueKind::F64: return "f64";
        case ValueKind::Bool: return "bool";

        case ValueKind::Char: return "char";
        case ValueKind::String: return "string";

        case ValueKind::Error: return "error";
        case ValueKind::Type: return "type";

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

ValueKind ValueType::getKind() const {
    return ValueKind::Type;
}

std::optional<ValueKind> ValueType::getTypeKind() const {
    return this->type_kind;
}