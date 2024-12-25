# dcli

## Prerequisites
- Conan 2.11.0
- GCC 14.2.0
- [Optional] CMake 3.31.3
- [Optional] GNU Make 4.4.1 (MinGW)

## Build from source
```
$ conan profile detect --force
$ conan install . --output-folder=build --build=missing
```

> ### Windows
```
$ conan profile show
Host profile:
[settings]
arch=x86_64
build_type=Release
compiler=gcc
compiler.cppstd=gnu20
compiler.libcxx=libstdc++11
compiler.version=14
os=Windows
[conf]
tools.cmake.cmaketoolchain:generator=MinGW Makefiles
tools.env.virtualenv:powershell=powershell.exe

Build profile:
[settings]
arch=x86_64
build_type=Release
compiler=gcc
compiler.cppstd=gnu20
compiler.libcxx=libstdc++11
compiler.version=14
os=Windows
[conf]
tools.cmake.cmaketoolchain:generator=MinGW Makefiles
tools.env.virtualenv:powershell=powershell.exe
```

```
$ .\build\build\Release\generators\conanbuild.ps1

$ cmake -S . -B build -G "MinGW Makefiles" -DCMAKE_TOOLCHAIN_FILE="build\Release\generators\conan_toolchain.cmake" -DCMAKE_BUILD_TYPE=Release
$ cmake --build build --config Release

$ .\build\dcli.exe

$ .\build\build\Release\generators\deactivate_conanbuild.ps1
```