#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueString final : public Value {
    public:
        ValueString();
        ValueString(const std::string &value);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<std::string> getValue() const;
    private:
        std::optional<std::string> value;
    };
}