#pragma once

#include <cstdint>
#include <values/value.hpp>

namespace common::elements {
    enum ElementModifier : uint8_t {
        None =      0,

        Const =     1 << 0,
        Any =       1 << 1,
        Error =     1 << 2,
        Break =     1 << 3,
        Continue =  1 << 4,
        Return =    1 << 5
    };

    class Element final {
    public:
        Element(uint8_t modifier, values::Value *value);
        ~Element();

        uint8_t getModifier() const;
        void addModifier(ElementModifier modifier);

        values::Value *getValue() const;
        bool isErrorOrController() const;

        Element *add(const Element &other) const;
        Element *increment();
        Element *subtract(const Element &other) const;
        Element *decrement();

        Element *unaryNot() const;

        Element *checkConvert(common::bytecodes::ApicaTypeBytecode to);
        Element *convert(common::bytecodes::ApicaTypeBytecode to);
        Element *autoConvert(common::bytecodes::ApicaTypeBytecode to);
    private:
        uint8_t modifier;
        values::Value *value;
    };
}