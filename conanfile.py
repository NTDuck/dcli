import os
from typing import Any

from conan import ConanFile
from conan.tools.build import can_run
from conan.tools.cmake import cmake_layout, CMake, CMakeToolchain


class PackageReference:
    name: str = "dcli"
    version: str = "0.0.1-indev"
    # user: str | None
    # channel: str | None

class Metadata:
    # description: str | None
    license: str | None = "BSD-3-Clause"
    # author: str | None
    # topics: tuple[str] | None
    # homepage: str | None
    url: str | None = "https://github.com/NTDuck/dcli.git"

class Requirements:
    requires: tuple[str] = (
        # "ncurses/6.5",
    )
    tool_requires: tuple[str] = (
        "cmake/3.30.0",
        "ninja/1.12.0",
    )
    test_requires: tuple[str] = (
        "doctest/2.4.11",
    )
    # python_requires: tuple[str]
    # python_requires_extend: tuple[str]

class Sources:
    # exports: tuple[str]
    # exports_sources: tuple[str]
    source_buildenv = True

class BinaryModel:
    package_type: str | None = "application"
    settings: tuple[str] = ( "os", "compiler", "build_type", "arch" )
    options: dict[str, str | Any] = {
        "shared": [ True, False ],
    }
    default_options: dict[str, str | Any] = {
        "shared": True,
        # "ncurses:shared": True,
        # "ncurses:with_static": False,
        # "ncurses:with_widec": True,
    }
    # default_build_options: dict[str, str | Any]
    # options_description: dict[str, str]
    languages = "C++"
    
    # package_id_embed_mode: str
    # package_id_non_embed_mode: str
    # package_id_python_mode: str
    # package_id_unknown_mode: str

class Build:
    generators = ( "CMakeDeps" )   # CMakeToolchain configured independently
    build_policy = "missing"

    def generate(self):
        toolchain = CMakeToolchain(self)

        toolchain.generator = "Ninja"

        toolchain.cache_variables.update({
            "CMAKE_TOOLCHAIN_FILE": self.toolchain_file,
            "CMAKE_BUILD_TYPE": self.build_type,
            "CMAKE_CXX_STANDARD": 20,
            "CMAKE_EXPORT_COMPILE_COMMANDS": "ON",
        })
        
        if self.build_type == "Release":
            toolchain.preprocessor_definitions.update(self.release_cmake_preprocessor_definitions)
            toolchain.extra_cxxflags = self.release_cmake_cxx_flags
            # strip ...
        else:
            toolchain.preprocessor_definitions.update(self.debug_cmake_preprocessor_definitions)
            toolchain.extra_cxxflags = self.debug_cmake_cxx_flags

        toolchain.generate()

    @property
    def release_cmake_cxx_flags(self):
        return [
            "-O3", "-Wall",
        ]
    
    @property
    def debug_cmake_cxx_flags(self):
        return [
            "-g", "-Og", "-Ofast", "-march=native", "-mfpmath=sse", "-freorder-blocks", 
            "-fpredictive-commoning", "-fno-threadsafe-statics", "-ffloat-store", 
            "-ffast-math", "-fno-rounding-math", "-fno-signaling-nans", "-fcx-limited-range", 
            "-fno-math-errno", "-funsafe-math-optimizations", "-fassociative-math", 
            "-freciprocal-math", "-ffinite-math-only", "-fno-signed-zeros", 
            "-fno-trapping-math", "-frounding-math", "-fsingle-precision-constant", 
            "-fcx-fortran-rules",
        ]

    @property
    def release_cmake_preprocessor_definitions(self):
        return {
            "NDEBUG": None,
        }
    
    @property
    def debug_cmake_preprocessor_definitions(self):
        return {
            "DEBUG": None,
        }
    
    @property
    def build_type(self):
        return str(self.settings.build_type)
    
    @property
    def toolchain_file(self):
        # Assuming CMake layout
        return os.path.join(
            self.build_folder,
            # self.build_type,
            "generators",
            "conan_toolchain.cmake",
        )

class FoldersAndLayout:
    pass

class Layout:
    pass

class Miscellaneous:
    pass

class Recipe(
    PackageReference, Metadata, Requirements, Sources, 
    BinaryModel, Build, FoldersAndLayout, Layout, Miscellaneous,
    ConanFile   # Placed at the end to avoid inheritence conflict
):
    # Folders and layout
    # source_folder = 
    # build_folder = "build"
    no_copy_source = True

    # Layout
    def layout(self):
        cmake_layout(self)

    implements = [ "auto_shared_fpic" ]

    # def configure(self):
    #     pass

    # def generate(self):
    #     toolchain = CMakeToolchain(self)

    #     toolchain.generator = "Ninja"

    #     toolchain.cache_variables.update({
    #         "CMAKE_TOOLCHAIN_FILE": self.toolchain_file,
    #         "CMAKE_BUILD_TYPE": self.build_type,
    #         "CMAKE_CXX_STANDARD": 20,
    #         "CMAKE_EXPORT_COMPILE_COMMANDS": "ON",
    #     })
        
    #     if self.build_type == "Release":
    #         toolchain.preprocessor_definitions.update(self.release_cmake_preprocessor_definitions)
    #         toolchain.extra_cxxflags = self.release_cmake_cxx_flags
    #         # strip ...
    #     else:
    #         toolchain.preprocessor_definitions.update(self.debug_cmake_preprocessor_definitions)
    #         toolchain.extra_cxxflags = self.debug_cmake_cxx_flags

    #     toolchain.generate()

    def build(self):
        cmake = CMake(self)

        cmake.configure()
        cmake.build()

        if can_run(self):
            cmake.test()

    # def test(self):
    #     pass

    def package(self):
        cmake = CMake(self)
        cmake.install()

    @property
    def release_cmake_cxx_flags(self):
        return [
            "-O3", "-Wall",
        ]
    
    @property
    def debug_cmake_cxx_flags(self):
        return [
            "-g", "-Og", "-Ofast", "-march=native", "-mfpmath=sse", "-freorder-blocks", 
            "-fpredictive-commoning", "-fno-threadsafe-statics", "-ffloat-store", 
            "-ffast-math", "-fno-rounding-math", "-fno-signaling-nans", "-fcx-limited-range", 
            "-fno-math-errno", "-funsafe-math-optimizations", "-fassociative-math", 
            "-freciprocal-math", "-ffinite-math-only", "-fno-signed-zeros", 
            "-fno-trapping-math", "-frounding-math", "-fsingle-precision-constant", 
            "-fcx-fortran-rules",
        ]

    @property
    def release_cmake_preprocessor_definitions(self):
        return {
            "NDEBUG": None,
        }
    
    @property
    def debug_cmake_preprocessor_definitions(self):
        return {
            "DEBUG": None,
        }
    
    @property
    def build_type(self):
        return str(self.settings.build_type)
    
    @property
    def toolchain_file(self):
        # Assuming CMake layout
        return os.path.join(
            self.build_folder,
            # self.build_type,
            "generators",
            "conan_toolchain.cmake",
        )
