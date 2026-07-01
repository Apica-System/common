#pragma once

#include "values/error.hpp"
#include <vector>

namespace common::values {
    class ValueErrorStackTrace final : public ValueError {
    public:
        ValueErrorStackTrace(const std::string &name, const std::string &details);

        void addTrace(const std::string &trace);

        std::string getErrorMessage() const override;
    private:
        std::vector<std::string> stack_trace;
    };
}