#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueU64 final : public Value {
    public:
        ValueU64();
        ValueU64(uint64_t value);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<uint64_t> getValue() const;
    private:
        std::optional<uint64_t> value;
    };
}