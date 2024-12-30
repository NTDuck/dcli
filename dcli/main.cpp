#include <iostream>

#include "spdlog/spdlog.h"

int main(void) {
    spdlog::info("`spdlog` in use");

    #ifdef NDEBUG
        spdlog::info("tomfoolery! Release.");
    #else
        spdlog::info("tomfoolery! Debug.");
    #endif

    return EXIT_SUCCESS;
}
