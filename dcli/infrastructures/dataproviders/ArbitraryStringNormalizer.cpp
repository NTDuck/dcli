#include <cctype>
#include <string>
#include <utility>

#include <dcli/core/dataproviders/StringNormalizer.hpp>

class ArbitraryStringNormalizer : public StringNormalizer<ArbitraryStringNormalizer> {
public:
    std::string& normalize(std::string& s) override {
        trim(s);

        return s;
    }

private:
    /// @see https://stackoverflow.com/a/27788112
    static std::string& trim(std::string& s) noexcept {
        if (s.empty())
            return s;

        auto const chars = s.c_str();

        std::size_t frontIndex = 0;
        while (frontIndex < s.length() && std::isspace(int(chars[frontIndex])))
            ++frontIndex;

        std::size_t backIndex = s.length();
        while (backIndex > frontIndex && std::isspace(int(chars[backIndex - 1])))
            --backIndex;

        if (frontIndex == 0 && backIndex < s.length())
            s.resize(backIndex - frontIndex);
        else if (backIndex <= frontIndex)
            s.clear();
        else 
            s = std::move(std::string(s.begin() + frontIndex, s.begin() + backIndex));

        return s;
    }
};
