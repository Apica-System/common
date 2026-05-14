#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueI16 final : public Value {
    public:
        ValueI16();
        ValueI16(int16_t value);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<int16_t> getValue() const;
    private:
        std::optional<int16_t> value;
    };
}