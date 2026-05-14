#include "values/error.hpp"
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

ValueKind ValueError::getKind() const {
    return ValueKind::Error;
}

std::optional<std::string> ValueError::getName() const {
    return this->name;
}

std::optional<std::string> ValueError::getDetails() const {
    return this->details;
}