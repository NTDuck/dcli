use std::mem::ManuallyDrop;
use std::sync::Arc;
use std::sync::RwLock;

use use_cases::gateways::pointers::SharedPointerHandle;

type Typed<T> = Arc<RwLock<T>>;
type Untyped = Typed<()>;

pub struct ArcRwLockSharedPointerHandle {
    untyped: ManuallyDrop<Untyped>,
}

unsafe impl SharedPointerHandle for ArcRwLockSharedPointerHandle {
    fn new<T>(obj: T) -> Self {
        let typed = Arc::new(RwLock::new(obj));

        return Self {
            untyped: ManuallyDrop::new(Self::into_untyped(typed)),
        };
    }
    
    fn unwrap<'a, T: 'a>(&'a self) -> impl std::ops::Deref<Target = T> + 'a {
        return self.as_ref().read().unwrap();
    }
    
    fn unwrap_mut<'a, T: 'a>(&'a self) -> impl std::ops::DerefMut<Target = T> + 'a {
        return self.as_ref().write().unwrap();
    }

    unsafe fn clone<T>(&self) -> Self {
        let typed = Arc::clone(self.as_ref());

        return Self {
            untyped: ManuallyDrop::new(typed),
        };
    }

    unsafe fn drop<T>(&mut self) {
        ManuallyDrop::drop(&mut self.untyped);
    }
}

impl ArcRwLockSharedPointerHandle {
    fn into_untyped<T>(typed: Typed<T>) -> Untyped {
        return unsafe {
            std::mem::transmute(typed)
        };
    }

    // fn into_typed<T>(untyped: Untyped) -> Typed<T> {
    //     return unsafe {
    //         std::mem::transmute(untyped)
    //     };
    // }

    fn as_ref<T>(&self) -> &Typed<T> {
        // let typed_ptr: *const Typed<T> =
        //     (self.untyped.deref() as *const Untyped)
        //         .cast::<std::alloc::sync::Typed<T>>();

        // let _ = std::mem::transmute::<Untyped, Typed<T>>;

        // return unsafe { &*typed_ptr };
        let untyped: &Untyped = &self.untyped;

        return unsafe {
            std::mem::transmute(&untyped)
        };
    }
}
