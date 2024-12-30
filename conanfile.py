import os

from conan import ConanFile
from conan.tools.build import can_run
from conan.tools.cmake import cmake_layout, CMake, CMakeToolchain


class Recipe(ConanFile):
    # Requirements
    requires = (
        # "ncurses/6.5",
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
        # "ncurses:shared": True,
        # "ncurses:with_static": False,
        # "ncurses:with_widec": True,
    }
    languages = "C++"

    # Build
    generators = ("CMakeDeps")   # CMakeToolchain configured independently

    __cmake_cxx_standard = 20
    __cmake_cxx_flags_release = [
        "-O3",
        "-Wall",
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
    # def init(self):
    #     self.__cmake = CMake(self)
    #     self.__toolchain = CMakeToolchain(self)

    # def configure(self):
    #     self.__cmake.configure()

    def generate(self):
        self.__toolchain = CMakeToolchain(self)
        self.__toolchain.generator = "Ninja"

        self.__toolchain.cache_variables.update({
            "CMAKE_TOOLCHAIN_FILE": self.__toolchain_file,
            "CMAKE_BUILD_TYPE": self.__build_type,
            "CMAKE_CXX_STANDARD": self.__cmake_cxx_standard,
            "CMAKE_EXPORT_COMPILE_COMMANDS": "ON",
        })
        
        if self.__build_type == "Release":
            self.__toolchain.preprocessor_definitions.update(self.__cmake_preprocessor_definitions_release)
            self.__toolchain.extra_cxxflags = self.__cmake_cxx_flags_release
            # strip ...
        else:
            self.__toolchain.preprocessor_definitions.update(self.__cmake_preprocessor_definitions_debug)
            self.__toolchain.extra_cxxflags = self.__cmake_cxx_flags_debug

        self.__toolchain.generate()

    def build(self):
        self.__cmake = CMake(self)
        self.__cmake.configure()
        self.__cmake.build()

        if can_run(self):
            self.__cmake.test()

    # def test(self):
    #     if can_run(self):
    #         self.__cmake.test()
