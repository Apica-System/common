#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueType final : public Value {
    public:
        ValueType();
        ValueType(ValueKind type_kind);

        static std::string getKindRepr(ValueKind kind);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<ValueKind> getTypeKind() const;
    private:
        std::optional<ValueKind> type_kind;
    };
}