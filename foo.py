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

# Define class A
class A:
    @classmethod
    def show(cls):
        print("Class A")
        # Check the class-level attributes using `cls.__dict__`
        for attr_name, attr_value in cls.__dict__.items():
            if not attr_name.startswith('__'):  # Skip special attributes like __doc__
                print(f"{attr_name}: {attr_value}")

# Define class B with some static attributes
class B:
    attr1 = 'value1'
    attr2 = 'value2'

# Create an instance of ClassAttributeCopier with target and source classes
attribute_copier = ClassAttributeCopier(B, A)

# Now call the instance, no need to pass classes again
attribute_copier()

# Now class A will have the attributes from class B
A.show()
