#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueChar final : public Value {
    public:
        ValueChar();
        ValueChar(uint32_t value);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<uint32_t> getValue() const;
    private:
        std::optional<uint32_t> value;
    };
}