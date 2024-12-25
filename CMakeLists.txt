cmake_minimum_required(VERSION 3.30)
project(dcli LANGUAGES CXX)

enable_testing()

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

find_package(doctest REQUIRED)

file(GLOB_RECURSE headers "${PROJECT_NAME}/*.hpp")
file(GLOB_RECURSE sources "${PROJECT_NAME}/*.cpp")
# file(GLOB_RECURSE modules "${PROJECT_NAME}/*.mpp")
file(GLOB_RECURSE unit_tests "${PROJECT_NAME}/*.test.cpp")
file(GLOB_RECURSE integration_tests "${PROJECT_NAME}/*/tests/*.cpp")

add_executable(${PROJECT_NAME})
target_sources(${PROJECT_NAME} PUBLIC ${headers} ${sources})
# target_sources(${PROJECT_NAME} PUBLIC FILE_SET modules TYPE CXX_MODULES FILES ${modules})

target_include_directories(${PROJECT_NAME} PRIVATE ${PROJECT_NAME})

add_executable(tests)
target_sources(${PROJECT_NAME} PUBLIC ${headers} ${sources})
# target_sources(tests PUBLIC FILE_SET modules TYPE CXX_MODULES FILES ${modules})
target_sources(tests PUBLIC ${unit_tests} ${integration_tests})

target_include_directories(tests PRIVATE ${DOCTEST_INCLUDE_DIR})
target_link_libraries(tests PRIVATE doctest::doctest)

add_test(NAME tests COMMAND tests)
