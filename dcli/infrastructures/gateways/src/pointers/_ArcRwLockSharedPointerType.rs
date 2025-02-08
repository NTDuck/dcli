use use_cases::gateways::pointers::SharedPointerType;

pub struct ArcRwLockSharedPointerType {
}

unsafe impl SharedPointerType for ArcRwLockSharedPointerType {
    fn new<T>(obj: T) -> Self {
        todo!()
    }
}

impl ArcRwLockSharedPointerType {}
