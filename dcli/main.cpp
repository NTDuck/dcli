#include <iostream>

int main(void) {
    #ifdef NDEBUG
        std::cout << "Hello from tomfoolery!" << std::endl;
    #else
        std::cout << "Debug from tomfoolery!" << std::endl;
    #endif

    return EXIT_SUCCESS;
}
