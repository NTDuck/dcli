use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;

use use_cases::gateways::pointers::SharedPointerHandle;

type Typed<T> = Arc<Mutex<T>>;
type Untyped = Typed<()>;

pub struct ArcMutexSharedPointerHandle {
    untyped: ManuallyDrop<Untyped>,
}

unsafe impl SharedPointerHandle for ArcMutexSharedPointerHandle {
    fn new<T>(obj: T) -> Self {
        let typed = Arc::new(Mutex::new(obj));
        let untyped = Self::into_untyped(typed);
        return Self::new_from_untyped(untyped);
    }
    
    fn unwrap<'a, T: 'a>(&'a self) -> impl Deref<Target = T> + 'a {
        return self.as_ref()
            .lock().unwrap();
    }
    
    fn unwrap_mut<'a, T: 'a>(&'a self) -> impl DerefMut<Target = T> + 'a {
        return self.as_ref()
            .lock().unwrap();
    }

    unsafe fn clone<T>(&self) -> Self {
        let untyped = Arc::clone(self.as_ref());
        return Self::new_from_untyped(untyped);
    }

    unsafe fn drop<T>(&mut self) {
        std::ptr::drop_in_place(self.as_mut::<T>());
    }
}

impl ArcMutexSharedPointerHandle {
    fn new_from_untyped(untyped: Untyped) -> Self {
        return Self {
            untyped: ManuallyDrop::new(untyped),
        };
    }

    fn into_untyped<T>(typed: Typed<T>) -> Untyped {
        return unsafe {
            std::mem::transmute(typed)
        };
    }

    fn as_ref<T>(&self) -> &Typed<T> {
        let raw_ref = (self.untyped.deref() as *const Untyped)
            .cast::<Typed<T>>();

        Self::static_size_check::<T>();

        return unsafe {
            &*raw_ref
        };
    }

    fn as_mut<T>(&mut self) -> &mut Typed<T> {
        let raw_mut = (self.untyped.deref_mut() as *mut Untyped)
            .cast::<Typed<T>>();

        Self::static_size_check::<T>();

        return unsafe {
            &mut *raw_mut
        };
    }

    fn static_size_check<T>() {
        std::hint::black_box(std::mem::transmute::<Untyped, Typed<T>>);
    }
}
