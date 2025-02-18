use axiom_derive::NewType;

#[derive(NewType)]
struct IntegerWrapper(i32);

#[test]
fn testIntegerWrapper() {
    // In production code, `IntegerWrapper` should be constructed
    // with custom constructor as per domain-level policies.
    let firstInteger = IntegerWrapper(2);
    let secondInteger = IntegerWrapper(3);

    let thirdInteger = *firstInteger + *secondInteger;
    assert_eq!(thirdInteger, 5);
}