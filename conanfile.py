import os

from conan import ConanFile
from conan.tools.build import can_run
from conan.tools.cmake import cmake_layout, CMake, CMakeToolchain, CMakeDeps


class Recipe(ConanFile):
    # Requirements
    requires = (
        # "ncurses/6.5",
        "spdlog/1.15.0",
    )
    tool_requires = (
        "cmake/3.30.0",
        "ninja/1.12.0",
    )
    test_requires = (
        "doctest/2.4.11",
    )

    # Sources
    source_buildenv = True

    # Binary model
    package_type = "application"
    settings = ("os", "compiler", "build_type", "arch")
    options = {
        "shared": [True, False],
    }
    default_options = {
        "shared": True,
    }
    languages = "C++"

    # Build
    __cmake_cxx_standard = 20
    __cmake_generator = "Ninja"

    __cmake_cache_variables = {
        "CMAKE_EXPORT_COMPILE_COMMANDS": "ON",
    }

    __cmake_cxx_flags_release = [
        "-O3",
        "-Wall", "-Wextra"

        # "pkgconf -cflags spdlog pkgconf -libs spdlog -lws2_32",
    ]
    __cmake_cxx_flags_debug = [
        "-g", "-Og", "-Ofast", "-march=native", "-mfpmath=sse", "-freorder-blocks", 
        "-fpredictive-commoning", "-fno-threadsafe-statics", "-ffloat-store", 
        "-ffast-math", "-fno-rounding-math", "-fno-signaling-nans", "-fcx-limited-range", 
        "-fno-math-errno", "-funsafe-math-optimizations", "-fassociative-math", 
        "-freciprocal-math", "-ffinite-math-only", "-fno-signed-zeros", 
        "-fno-trapping-math", "-frounding-math", "-fsingle-precision-constant", 
        "-fcx-fortran-rules",
    ]
    __cmake_preprocessor_definitions_release = {
        "NDEBUG": None,
    }
    __cmake_preprocessor_definitions_debug = {
        "DEBUG": None,
    }
    
    @property
    def __build_type(self):
        return str(self.settings.build_type)
    
    @property
    def __toolchain_file(self):
        # Assuming CMake layout
        return os.path.join(
            self.build_folder, # self.build_type,
            "generators", "conan_toolchain.cmake",
        )

    # Layout
    def layout(self):
        cmake_layout(self)

    # Miscellaneous
    implements = ["auto_shared_fpic", "auto_header_only"]
    extension_properties = {
        "compatibility_cppstd": False,
    }

    # Methods
    def generate(self):
        self.__generate_cmake_toolchain()
        self.__generate_cmake_deps()

    def __generate_cmake_toolchain(self):
        toolchain = CMakeToolchain(self)
        toolchain.generator = self.__cmake_generator

        toolchain.cache_variables.update({
            "CMAKE_TOOLCHAIN_FILE": self.__toolchain_file,
            "CMAKE_BUILD_TYPE": self.__build_type,
            "CMAKE_CXX_STANDARD": self.__cmake_cxx_standard,
        })
        toolchain.cache_variables.update(self.__cmake_cache_variables)
        
        if self.__build_type == "Release":
            toolchain.preprocessor_definitions.update(self.__cmake_preprocessor_definitions_release)
            toolchain.extra_cxxflags = self.__cmake_cxx_flags_release
            # strip ...
        else:
            toolchain.preprocessor_definitions.update(self.__cmake_preprocessor_definitions_debug)
            toolchain.extra_cxxflags = self.__cmake_cxx_flags_debug

        toolchain.generate()

    def __generate_cmake_deps(self):
        deps = CMakeDeps(self)
        deps.generate()

    # TODO Add "--strip" to "cmake install"
    def build(self):
        cmake = CMake(self)

        cmake.configure()
        cmake.build()

        if can_run(self):
            cmake.test()
