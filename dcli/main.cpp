#include <iostream>

#include "spdlog/spdlog.h"

int main(void) {
    spdlog::info("`spdlog` in use");

    #ifdef NDEBUG
        std::cout << "Hello from tomfoolery!" << std::endl;
    #else
        std::cout << "Debug from tomfoolery!" << std::endl;
    #endif

    return EXIT_SUCCESS;
}
