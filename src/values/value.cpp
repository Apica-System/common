#include "values/error.hpp"

using namespace common::values;

ValueError *Value::nullOperationError(const std::string &op, bool is_unary) {
    std::string error_message("Cannot perform ");
    error_message += (is_unary ? "unary" : "binary");
    error_message += " operation `";
    error_message += op;
    error_message += "` with a null value";

    return new ValueError(
        "OperationError",
        error_message
    );
}

ValueError *Value::unaryOperationError(const std::string &op, const std::string &operand) {
    std::string error_message("Unary operator `");
    error_message += op;
    error_message += "` is not defined for type <";
    error_message += operand;
    error_message += '>';

    return new ValueError(
        "OperationError",
        error_message
    );
}

ValueError *Value::binaryOperationError(const std::string &op, const std::string &left, const std::string &right) {
    std::string error_message("Binary operator `");
    error_message += op;
    error_message += "` is not defined for types <";
    error_message += left;
    error_message += "> and <";
    error_message += right;
    error_message += '>';
    
    return new ValueError(
        "OperationError",
        error_message
    );
}