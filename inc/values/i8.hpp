#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueI8 final : public Value {
    public:
        ValueI8();
        ValueI8(int8_t value);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<int8_t> getValue() const;
    private:
        std::optional<int8_t> value;
    };
}