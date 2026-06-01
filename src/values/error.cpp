#include "values/error.hpp"
#include "values/bool.hpp"
#include "values/type.hpp"
#include "values/string.hpp"
#include "elements.hpp"

using namespace common::values;

ValueError::ValueError() 
    : name(std::nullopt), details(std::nullopt) {
    
}

ValueError::ValueError(const std::string &name)
    : name(name), details(std::nullopt) {
    
}

ValueError::ValueError(const std::string &name, const std::string &details)
    : name(name), details(details) {
    
}

void ValueError::show(char end) const {
    if (this->name) {
        std::cout << "error<" << this->name.value();
        if (this->details)
            std::cout << ": " << this->details.value();
        
        std::cout << '>' << end;
    } else {
        std::cout << "error<null>" << end;
    }
}

bool ValueError::isNull() const {
    return !this->name.has_value();
}

std::string ValueError::getTypeRepr() const {
    return "error";
}

common::bytecodes::ApicaTypeBytecode ValueError::getKind() const {
    return common::bytecodes::ApicaTypeBytecode::Error;
}

std::optional<Value*> ValueError::add(const Value *) const {
    return std::nullopt;
}

std::optional<Value*> ValueError::increment() {
    return std::nullopt;
}

std::optional<Value*> ValueError::subtract(const Value *) const {
    return std::nullopt;
}

std::optional<Value*> ValueError::decrement() {
    return std::nullopt;
}

std::optional<Value*> ValueError::unaryNot() const {
    return new ValueBool(!this->name.has_value());
}

std::optional<Value*> ValueError::convert(common::bytecodes::ApicaTypeBytecode to) const {
    if (this->name) {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Bool:
                return new ValueBool(true);
            
            case common::bytecodes::ApicaTypeBytecode::String: {
                std::string result("error<");
                result += this->name.value();
                if (this->details) {
                    result += ": ";
                    result += this->details.value();
                }
                result += '>';

                return new ValueString(result);
            }

            case common::bytecodes::ApicaTypeBytecode::Type:
                return new ValueType(common::bytecodes::ApicaTypeBytecode::Error);

            default: return std::nullopt;
        }
    } else {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Bool:
                return new ValueBool();
            
            case common::bytecodes::ApicaTypeBytecode::String:
                return new ValueString();

            case common::bytecodes::ApicaTypeBytecode::Type:
                return new ValueType(common::bytecodes::ApicaTypeBytecode::Error);

            default: return std::nullopt;
        }
    }
}

std::optional<Value*> ValueError::autoConvert(common::bytecodes::ApicaTypeBytecode to) const {
    if (this->name) {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Any:
            case common::bytecodes::ApicaTypeBytecode::Error: {
                if (this->details)
                    return new ValueError(this->name.value(), this->details.value());
                else
                    return new ValueError(this->name.value());
            }

            default: return std::nullopt;
        }
    } else {
        switch (to) {
            case common::bytecodes::ApicaTypeBytecode::Error:
                return new ValueError();

            default: return std::nullopt;
        }
    }
}

std::optional<std::string> ValueError::getName() const {
    return this->name;
}

std::optional<std::string> ValueError::getDetails() const {
    return this->details;
}