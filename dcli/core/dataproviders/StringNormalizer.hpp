#pragma once

#include <string>

template <typename T>
class StringNormalizer {
public:
    virtual std::string& normalize(std::string& s) = 0;
    
    virtual ~StringNormalizer() = default;
};
