#pragma once

#include "values/value.hpp"

namespace common::values {
    class ValueError : public Value {
    public:
        ValueError();
        ValueError(const std::string &name);
        ValueError(const std::string &name, const std::string &details);

        void show(char end) const override;
        bool isNull() const override;
        std::string getTypeRepr() const override;
        common::bytecodes::ApicaTypeBytecode getKind() const override;

        virtual void addTrace(const std::string &trace);
        virtual std::string getErrorMessage() const;

        std::optional<Value*> add(const Value *other) const override;
        std::optional<Value*> increment() override;
        std::optional<Value*> leftIncrement() override;
        std::optional<Value*> subtract(const Value *other) const override;
        std::optional<Value*> decrement() override;
        std::optional<Value*> leftDecrement() override;
        std::optional<Value*> times(const Value *other) const override;

        std::optional<Value*> unaryNot() const override;
        std::optional<Value*> bitwiseNot() const override;

        std::optional<Value*> lessThan(const Value *other) const override;
        std::optional<Value*> lessOrEquals(const Value *other) const override;
        std::optional<Value*> greaterThan(const Value *other) const override;
        std::optional<Value*> greaterOrEquals(const Value *other) const override;

        std::optional<Value*> convert(common::bytecodes::ApicaTypeBytecode to) const override;
        std::optional<Value*> autoConvert(common::bytecodes::ApicaTypeBytecode to) const override;

        std::optional<std::string> getName() const;
        std::optional<std::string> getDetails() const;
    protected:
        std::optional<std::string> name;
        std::optional<std::string> details;
    };
}