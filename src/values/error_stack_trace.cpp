#include "values/error_stack_trace.hpp"
#include "values/string.hpp"
#include "values/bool.hpp"
#include "values/type.hpp"

#include "elements.hpp"

using namespace common::values;

ValueErrorStackTrace::ValueErrorStackTrace(const std::string &name, const std::string &details)
    : ValueError(name, details) {
    
}

void ValueErrorStackTrace::addTrace(const std::string &trace) {
    this->stack_trace.push_back(trace);
}

std::string ValueErrorStackTrace::getErrorMessage() const {
    std::string error_message(this->name.value_or(""));
    if (this->details) {
        error_message += ": ";
        error_message += this->details.value();
    }

    error_message += "\nStack trace:";
    for (std::string trace : this->stack_trace) {
        error_message += '\n';
        error_message += trace;
    }

    return error_message;
}