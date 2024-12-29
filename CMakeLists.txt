cmake_minimum_required(VERSION 3.30)
project(dcli LANGUAGES CXX)

enable_testing()

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Conan integration
file(GLOB GENERATOR_DIRS "${CMAKE_BINARY_DIR}/*/generators")
list(APPEND CMAKE_PREFIX_PATH ${GENERATOR_DIRS})

# Dependendies
# find_package(Curses REQUIRED)
find_package(doctest REQUIRED)

# Output directories
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY
  ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY
  ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY
  ${CMAKE_BINARY_DIR}/bin)

# Source files
file(GLOB_RECURSE HEADERS "${PROJECT_NAME}/*.hpp")
file(GLOB_RECURSE SOURCES "${PROJECT_NAME}/*.cpp")
file(GLOB_RECURSE MODULES "${PROJECT_NAME}/*.mpp")
file(GLOB_RECURSE UNIT_TESTS "${PROJECT_NAME}/*.test.cpp")
file(GLOB_RECURSE INTEGRATION_TESTS "${PROJECT_NAME}/*/tests/*.cpp")

list(REMOVE_ITEM SOURCES ${UNIT_TESTS})
list(REMOVE_ITEM SOURCES ${INTEGRATION_TESTS})

# Targets
set(SHARED_LIBRARY_TARGET ${PROJECT_NAME}_lib)
set(MAIN_EXECUTABLE_TARGET ${PROJECT_NAME})
set(UNIT_TESTS_TARGET ${PROJECT_NAME}_unit_tests)
set(INTEGRATION_TESTS_TARGET ${PROJECT_NAME}_integration_tests)

# Shared library
if (HEADERS OR SOURCES OR MODULES)
  add_library(${SHARED_LIBRARY_TARGET} STATIC)

  if (HEADERS OR SOURCES)
    target_sources(${SHARED_LIBRARY_TARGET}
      PRIVATE
        ${HEADERS} ${SOURCES}
    )
        # Curses::Curses)
  endif()

  if (MODULES)
    target_sources(${SHARED_LIBRARY_TARGET}
      PRIVATE
        FILE_SET MODULES TYPE CXX_MODULES
          FILES ${MODULES})
  endif()

  target_include_directories(${SHARED_LIBRARY_TARGET}
    PUBLIC
      ${CMAKE_CURRENT_SOURCE_DIR}/${PROJECT_NAME})
endif()

# Main executable
if (SOURCES)
  add_executable(${MAIN_EXECUTABLE_TARGET})
  target_sources(${MAIN_EXECUTABLE_TARGET}
    PRIVATE
      ${SOURCES})
  target_link_libraries(${MAIN_EXECUTABLE_TARGET}
    PRIVATE
      ${SHARED_LIBRARY_TARGET})
endif()

# Unit tests
if (UNIT_TESTS)
  add_executable(${UNIT_TESTS_TARGET})
  target_sources(${UNIT_TESTS_TARGET}
    PRIVATE
      ${UNIT_TESTS})
  target_link_libraries(${UNIT_TESTS_TARGET}
    PRIVATE
      ${SHARED_LIBRARY_TARGET}
      doctest::doctest)
  target_include_directories(${UNIT_TESTS_TARGET}
    PRIVATE
      ${DOCTEST_INCLUDE_DIR})

  add_test(NAME unit_tests
    COMMAND ${UNIT_TESTS_TARGET})
endif()

# Integration tests
if (INTEGRATION_TESTS)
  add_executable(${INTEGRATION_TESTS_TARGET})
  target_sources(${INTEGRATION_TESTS_TARGET}
    PRIVATE
      ${INTEGRATION_TESTS})
  target_link_libraries(${INTEGRATION_TESTS_TARGET}
    PRIVATE
      ${SHARED_LIBRARY_TARGET}
      doctest::doctest)
  target_include_directories(${INTEGRATION_TESTS_TARGET}
    PRIVATE
      ${DOCTEST_INCLUDE_DIR})

  add_test(NAME integration_tests
    COMMAND ${INTEGRATION_TESTS_TARGET})
endif()
