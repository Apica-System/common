#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueBool final : public Value {
    public:
        ValueBool();
        ValueBool(bool value);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<bool> getValue() const;
    private:
        std::optional<bool> value;
    };
}