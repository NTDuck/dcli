use smart_pointers::ArcRwLockSharedPointer;

#[test]
fn constructible() {
    trait Dep {}

    struct DepImpl;
    impl Dep for DepImpl {}

    let dep: ArcRwLockSharedPointer<Box<dyn Dep>> = ArcRwLockSharedPointer::new(Box::new(DepImpl));
}
