#pragma once

#include <cstdint>

namespace common::bytecodes {
    enum ApicaBytecode : uint64_t {
        EndOfFile =         0x00000000,
        EndOfBlock =        0x00000001,
        Entrypoint =        0x00000002,
        BuiltinFuncCall =   0x00000003,
        Literal =           0x00000004,
        Compound =          0x00000005,
        VarConstCall =      0x00000006,
        VarDecl =           0x00000007,
        ConstDecl =         0x00000008,
        Add =               0x00000009, // +
        Subtract =          0x0000000A, // -
        Assign =            0x0000000B, // =
        Increment =         0x0000000C, // elt++
        Decrement =         0x0000000D, // elt--
        LessThan =          0x0000000E, // <
        Equals =            0x0000000F, // ==
        Not =               0x00000010, // !
        As =                0x00000011, // as
        LeftIncrement =     0x00000012, // ++elt
        LeftDecrement =     0x00000013, // --elt
        AddAssign =         0x00000014, // +=
        SubtractAssign =    0x00000015, // -=
        Multiply =          0x00000016, // *
        MultiplyAssign =    0x00000017, // *=
        Divide =            0x00000018, // /
        DivideAssign =      0x00000019, // /=
        NotEquals =         0x0000001A, // !=
        SpecialOp =         0x0000001B, // @
        SpecialOpAssign =   0x0000001C, // @=
        BitwiseAnd =        0x0000001D, // &
        LogicalAnd =        0x0000001E, // &&
        BitwiseAndAssign =  0x0000001F, // &=
        BitwiseOr =         0x00000020, // |
        LogicalOr =         0x00000021, // ||
        BitwiseOrAssign =   0x00000022, // |=
        BitwiseNot =        0x00000023, // ~
        BitwiseXor =        0x00000024, // ^
        BitwiseXorAssign =  0x00000025, // ^=
        LessOrEquals =      0x00000026, // <=
        GreaterThan =       0x00000027, // >
        GreaterOrEquals =   0x00000028, // >=
        Modulo =            0x00000029, // %
        ModuloEquals =      0x0000002A, // %=
        Access =            0x0000002B, // .
        ConditionalAccess = 0x0000002C, // ?.
        Break =             0x0000002D,
        Continue =          0x0000002E,
        BlankReturn =       0x0000002F,
        FilledReturn =      0x00000030,
        QuestionOperation = 0x00000031,
        If =                0x00000032,
        IfElse =            0x00000033,
        While =             0x00000034,

        BYTECODE_LAST =     While
    };

    enum ApicaEntrypointBytecode : uint64_t {
        Init =              0x00000000,
        Update =            0x00000001,
        Quit =              0x00000002,

        ENTRYPOINT_LAST =   Quit
    };

    enum ApicaBuiltinFunctionBytecode : uint64_t {
        QuitApp =           0x00000000,
        LogInfo =           0x00000001,
        LognInfo =          0x00000002,
        LogSuccess =        0x00000003,
        LognSuccess =       0x00000004,
        LogWarning =        0x00000005,
        LognWarning =       0x00000006,
        LogError =          0x00000007,
        LognError =         0x00000008,
        LoadApp =           0x00000009,
        SetTitle =          0x0000000A,
        SetResizable =      0x0000000B,
        IsKeyReleased =     0x0000000C,
        IsKeyJustPressed =  0x0000000D,
        IsKeyPressed =      0x0000000E,

        BUILTIN_FUNC_LAST = IsKeyPressed
    };

    enum ApicaSpecificationBytecode : uint64_t {
        EndOfSpecification =    0x00000000,
        Title =                 0x00000001,
        Id =                    0x00000002,
        LoggerActivation =      0x00000003,
        WindowWidth =           0x00000004,
        WindowHeight =          0x00000005,
        Version =               0x00000006,
        IdCount =               0x00000007,

        SPECIFICATION_LAST =    IdCount
    };

    enum ApicaTypeBytecode : uint64_t {
        Null =      0x00000000,
        Any =       0x00000001,
        I8 =        0x00000002,
        I16 =       0x00000003,
        I32 =       0x00000004,
        I64 =       0x00000005,
        U8 =        0x00000006,
        U16 =       0x00000007,
        U32 =       0x00000008,
        U64 =       0x00000009,
        F32 =       0x0000000A,
        F64 =       0x0000000B,
        Bool =      0x0000000C,
        Char =      0x0000000D,
        String =    0x0000000E,
        Error =     0x0000000F,
        Type =      0x00000010,

        TYPE_LAST = Type
    };
}