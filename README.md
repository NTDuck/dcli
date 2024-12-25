# dcli

## Build from source
```
conan profile detect --force
conan install . --output-folder=build --build=missing
```

> ### Windows
```
.\build\build\Release\generators\conanbuild.ps1

cmake -S . -B build -G "MinGW Makefiles" -DCMAKE_TOOLCHAIN_FILE="build\Release\generators\conan_toolchain.cmake" -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release

.\build\dcli.exe

.\build\build\Release\generators\deactivate_conanbuild.ps1
```