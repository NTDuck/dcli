#include <iostream>

#include "spdlog/spdlog.h"

int main(void) {
    #ifdef NDEBUG
        spdlog::info("tomfoolery! Release.");
    #else
        spdlog::info("tomfoolery! Debug.");
    #endif

    return EXIT_SUCCESS;
}
