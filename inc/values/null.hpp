#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueNull final : public Value {
    public:
        ValueNull();

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;
    };
}