# dcli

## Prerequisites
- Conan 2.11.0
- GCC 14.2.0

## Build from source
```bash
$ conan profile detect --force
```

```bash
$ conan install . --build=missing
$ conan build .
$ .\build\Release\bin\dcli.exe
```

```bash
$ conan install . --build=missing --settings=build_type=Debug
$ conan build . --settings=build_type=Debug
$ .\build\Debug\bin\dcli.exe
```
