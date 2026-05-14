#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueError final : public Value {
    public:
        ValueError();
        ValueError(const std::string &name);
        ValueError(const std::string &name, const std::string &details);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        ValueKind getKind() const override;

        std::optional<std::string> getName() const;
        std::optional<std::string> getDetails() const;
    private:
        std::optional<std::string> name;
        std::optional<std::string> details;
    };
}