#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueI64 final : public Value {
    public:
        ValueI64();
        ValueI64(int64_t value);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<int64_t> getValue() const;
    private:
        std::optional<int64_t> value;
    };
}