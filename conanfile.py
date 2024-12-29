import os
import inspect
from typing import Any

from conan import ConanFile
from conan.tools.build import can_run
from conan.tools.cmake import cmake_layout, CMake, CMakeToolchain


class NiladicClassMethodsAutoRunner(type):
    def __new__(cls, name: str, bases: tuple[type], cls_dict: dict[str, Any]) -> type:
        new_cls = super().__new__(cls, name, bases, cls_dict)

        cls.__runNiladicClassMethods(new_cls)
        return new_cls
    
    @classmethod
    def __runNiladicClassMethods(cls, new_cls: type):
        for _, attr_value in new_cls.__dict__.items():
            if cls.__isNiladicClassMethod(attr_value):
                cls.__runNiladicClassMethod(attr_value)
    
    @classmethod
    def __isNiladicClassMethod(cls, method: Any) -> bool:
        if not cls.__isClassMethod(method):
            return False

        signature = inspect.signature(method.__func__)
        parameters = signature.parameters

        return len(parameters) == 1   # `cls` automatically passed as first argument to any `classmethod`
    
    @classmethod
    def __isClassMethod(cls, method: Any) -> bool:
        return isinstance(method, classmethod)

    @classmethod
    def __runNiladicClassMethod(cls, method: classmethod):
        method.__func__(cls)

class ClassAttributeCopier:
    def __init__(self, source_cls: type, target_cls: type):
        self.source_cls = source_cls
        self.target_cls = target_cls

    def __call__(self):
        for attr_name in self.source_cls.__dict__:
            if self.__class__.__isPublicAttribute(attr_name):
                self.copy_class_attribute(attr_name)

    @classmethod
    def __isPublicAttribute(cls, attr_name: str) -> bool:
        return not any([cls.__isProtectedAttribute(attr_name), cls.__isPrivateAttribute(attr_name)])
    
    @classmethod
    def __isProtectedAttribute(cls, attr_name: str) -> bool:
        return attr_name.startswith("_")
    
    @classmethod
    def __isPrivateAttribute(cls, attr_name: str) -> bool:
        return attr_name.startswith("__")
    
    def copy_class_attribute(self, attr_name: str):
        attr_value = getattr(self.source_cls, attr_name)
        setattr(self.target_cls, attr_name, attr_value)


class PackageReference:
    name: str = "dcli"
    version: str = "0.0.1-indev"
    user: str | None = None
    channel: str | None = None

class Metadata:
    description: str | None = None
    license: str | None = "BSD-3-Clause"
    author: str | None = None
    topics: tuple[str] | None = None
    homepage: str | None = None
    url: str | None = "https://github.com/NTDuck/dcli.git"


class Recipe(ConanFile, metaclass=NiladicClassMethodsAutoRunner):
    @classmethod
    def set_package_reference(cls):
        ClassAttributeCopier(cls, PackageReference)

    @classmethod
    def set_metadata(cls):
        ClassAttributeCopier(cls, Metadata)

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
    
    # Source
    source_buildenv = True

    # Binary model
    package_type = "application"
    settings = ( "os", "compiler", "build_type", "arch" )
    options = {
        "shared": [ True, False ]
    }
    default_options = {
        "shared": True,
        # "ncurses:shared": True,
        # "ncurses:with_static": False,
        # "ncurses:with_widec": True,
    }
    languages = "C++"

    # Build
    generators = ( "CMakeDeps" )   # CMakeToolchain configured independently
    build_policy = "missing"

    # Folders and layout
    # source_folder = 
    build_folder = "build"
    no_copy_source = True

    # Layout
    def layout(self):
        cmake_layout(self)

    implements = [ "auto_shared_fpic" ]

    # def configure(self):
    #     pass

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
        else:
            toolchain.preprocessor_definitions.update(self.debug_cmake_preprocessor_definitions)
            toolchain.extra_cxxflags = self.debug_cmake_cxx_flags

        toolchain.generate()

    def build(self):
        cmake = CMake(self)

        cmake.configure(
            # cli_args=[
            #     f"-DCMAKE_TOOLCHAIN_FILE={self.toolchain_file}",
            # ]
        )

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
        print(f"Toolchain file is {os.path.join(
            self.build_folder,
            self.build_type,
            "generators",
            "conan_toolchain.cmake",
        )}")
        return os.path.join(
            self.build_folder,
            self.build_type,
            "generators",
            "conan_toolchain.cmake",
        )
