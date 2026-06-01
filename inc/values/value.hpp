#pragma once

#include "bytecodes.hpp"
#include <string>
#include <optional>
#include <iostream>

namespace common::elements {
    class Element;
}

namespace common::values {
    class ValueError;

    class Value {
    public:
        virtual ~Value() {}

        static ValueError *nullOperationError(const std::string &op, bool is_unary);
        static ValueError *unaryOperationError(const std::string &op, const std::string &operand);
        static ValueError *binaryOperationError(const std::string &op, const std::string &left, const std::string &right);

        virtual void show(char end) const = 0;
        virtual bool isNull() const = 0;
        virtual std::string getTypeRepr() const = 0;
        virtual common::bytecodes::ApicaTypeBytecode getKind() const = 0;

        virtual std::optional<Value*> add(const Value *other) const = 0;
        virtual std::optional<Value*> increment() = 0;
        virtual std::optional<Value*> subtract(const Value *other) const = 0;
        virtual std::optional<Value*> decrement() = 0;

        virtual std::optional<Value*> unaryNot() const = 0;

        virtual std::optional<Value*> convert(common::bytecodes::ApicaTypeBytecode to) const = 0;
        virtual std::optional<Value*> autoConvert(common::bytecodes::ApicaTypeBytecode to) const = 0;
    };
}