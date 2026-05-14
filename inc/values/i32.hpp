#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueI32 final : public Value {
    public:
        ValueI32();
        ValueI32(int32_t value);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<int32_t> getValue() const;
    private:
        std::optional<int32_t> value;
    };
}