# dcli

## Prerequisites
- Conan 2.11.0
- GCC 14.2.0

## Build from source
```bash
$ conan profile detect --force
$ conan install . --build=missing
$ conan build .
$ conan 
```

```bash
$ conan profile detect --force
$ conan install . -pr=".conan/profiles/release" -of=build -b=missing
$ conan install . -pr=".conan/profiles/debug" -of=build -b=missing
```

> ### Windows
> #### Release
```bash
$ .\build\build\Release\generators\conanbuild.ps1

$ cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release
$ cmake --build build --config Release

$ .\build\build\Release\bin\dcli.exe
$ .\build\build\Release\bin\dcli_unit_tests.exe
$ .\build\build\Release\bin\dcli_integration_tests.exe

$ .\build\build\Release\generators\deactivate_conanbuild.ps1
```

> #### Debug
```bash
$ .\build\build\Debug\generators\conanbuild.ps1

$ cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Debug
$ cmake --build build --config Debug

$ .\build\build\Debug\bin\dcli.exe
$ .\build\build\Debug\bin\dcli_unit_tests.exe
$ .\build\build\Debug\bin\dcli_integration_tests.exe

$ .\build\build\Debug\generators\deactivate_conanbuild.ps1
```
