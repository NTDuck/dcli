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

class Requirements:
    requires: tuple[str] = ()
    tool_requires: tuple[str] = (
        "cmake/3.30.0",
        "ninja/1.12.0",
    )
    test_requires: tuple[str] = (
        "doctest/2.4.11",
    )
    python_requires: tuple[str] = ()
    python_requires_extend: tuple[str] = ()

class Recipe(PackageReference, Metadata, Requirements):
    pass

# Check if the class attributes are inherited
print(Recipe.name)  # Should print "dcli"
print(Recipe.license)  # Should print "BSD-3-Clause"
print(Recipe.tool_requires)  # Should print the tuple with "cmake" and "ninja"
