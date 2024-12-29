# dcli

## Prerequisites
- Conan 2.11.0
- GCC 14.2.0

## Build from source
```bash
$ conan profile detect --force
$ conan install . --build=missing

$ conan build . 

$ .\build\build\build\Release\bin\dcli.exe
```
