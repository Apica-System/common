#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueU8 final : public Value {
    public:
        ValueU8();
        ValueU8(uint8_t value);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<uint8_t> getValue() const;
    private:
        std::optional<uint8_t> value;
    };
}