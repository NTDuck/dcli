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

    struct Service {
        dep: ArcRwLockSharedPointer<Box<dyn Dep>>,
    }

    impl Service {
        fn do_something(&self) {
            self.dep.unwrap().read();
            self.dep.unwrap_mut().write();
        }
    }

    struct DepImpl;
    impl Dep for DepImpl {}

    let dep: ArcRwLockSharedPointer<Box<dyn Dep>> = ArcRwLockSharedPointer::new(Box::new(DepImpl));

    let serv = Service { dep };
}
