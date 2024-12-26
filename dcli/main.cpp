#include <fmt/core.h>

int main(void) {
    #ifdef NDEBUG
        fmt::print("Hello from tomfoolery!\n");
    #else
        fmt::print("Debug from tomfoolery!\n");
    #endif

    return EXIT_SUCCESS;
}