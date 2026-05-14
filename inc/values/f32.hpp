#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueF32 final : public Value {
    public:
        ValueF32();
        ValueF32(float value);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<float> getValue() const;
    private:
        std::optional<float> value;
    };
}