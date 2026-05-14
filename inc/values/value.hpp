#pragma once

#include "values/kind.hpp"
#include <string>
#include <optional>
#include <iostream>

namespace common::values {
    class Value {
    public:
        virtual ~Value() {}

        virtual void show(char end) const = 0;
        virtual bool isNull() const = 0;
        virtual std::string getTypeRepr() const = 0;
        virtual ValueKind getKind() const = 0;
    };
}