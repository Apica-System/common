#pragma once

#include <cstdint>

namespace common {
    namespace values {
        enum ValueKind : uint64_t {
            Null,

            I8, I16, I32, I64,
            U8, U16, U32, U64,
            F32, F64,
            Bool,

            Char, String,

            Error, Type
        };
    }
}