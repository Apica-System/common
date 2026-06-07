#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueType final : public Value {
    public:
        ValueType();
        ValueType(common::bytecodes::ApicaTypeBytecode type_kind);

        static std::string getKindRepr(common::bytecodes::ApicaTypeBytecode kind);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        common::bytecodes::ApicaTypeBytecode getKind() const override;

        std::optional<Value*> add(const Value *other) const override;
        std::optional<Value*> increment() override;
        std::optional<Value*> leftIncrement() override;
        std::optional<Value*> subtract(const Value *other) const override;
        std::optional<Value*> decrement() override;
        std::optional<Value*> leftDecrement() override;

        std::optional<Value*> unaryNot() const override;
        std::optional<Value*> bitwiseNot() const override;

        std::optional<Value*> convert(common::bytecodes::ApicaTypeBytecode to) const override;
        std::optional<Value*> autoConvert(common::bytecodes::ApicaTypeBytecode to) const override;

        std::optional<common::bytecodes::ApicaTypeBytecode> getTypeKind() const;
    private:
        std::optional<common::bytecodes::ApicaTypeBytecode> type_kind;
    };
}