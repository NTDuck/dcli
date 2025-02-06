use smart_pointers::ArcRwLockSharedPointer;

#[test]
fn constructible() {
    trait Dep {
        fn read(&self) {
            println!("reading...");
        }

        fn write(&mut self) {
            println!("writing...");
        }
    }

    struct DepImpl;
    impl Dep for DepImpl {}

    let dep: ArcRwLockSharedPointer<Box<dyn Dep>> = ArcRwLockSharedPointer::new(Box::new(DepImpl));

    dep.unwrap().read();
    dep.unwrap_mut().write();
}
