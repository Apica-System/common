#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueU16 final : public Value {
    public:
        ValueU16();
        ValueU16(uint16_t value);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<uint16_t> getValue() const;
    private:
        std::optional<uint16_t> value;
    };
}