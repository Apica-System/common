#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueNull final : public Value {
    public:
        ValueNull();

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        common::bytecodes::ApicaTypeBytecode getKind() const override;

        std::optional<Value*> add(const Value *other) const override;
        std::optional<Value*> increment() override;
        std::optional<Value*> subtract(const Value *other) const override;
        std::optional<Value*> decrement() override;

        std::optional<Value*> unaryNot() const override;

        std::optional<Value*> convert(common::bytecodes::ApicaTypeBytecode to) const override;
        std::optional<Value*> autoConvert(common::bytecodes::ApicaTypeBytecode to) const override;
    };
}