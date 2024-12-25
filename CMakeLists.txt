cmake_minimum_required(VERSION 3.30)
project(dcli LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

find_package(doctest REQUIRED)

add_executable(${PROJECT_NAME} dcli/main.cpp)
target_link_libraries(${PROJECT_NAME} doctest::doctest)
