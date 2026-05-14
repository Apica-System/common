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
        Element(ElementModifier modifier, values::Value *value);
        ~Element();

        static Element createNull();

        ElementModifier getModifier() const;
        values::Value *getValue() const;
        bool isErrorOrController() const;
    private:
        ElementModifier modifier;
        values::Value *value;
    };
}