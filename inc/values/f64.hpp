#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueF64 final : public Value {
    public:
        ValueF64();
        ValueF64(double value);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<double> getValue() const;
    private:
        std::optional<double> value;
    };
}