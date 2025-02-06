use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

pub struct RcRefCellSharedPointer<T: ?Sized>(Rc<RefCell<T>>);

#[allow(dead_code)]
impl<T> RcRefCellSharedPointer<T> {
    pub fn new(object: T) -> Self {
        return Self(Rc::new(RefCell::new(object)));
    }

    pub fn unwrap(&self) -> impl Deref<Target = T> + '_ {
        return self.0.borrow();
    }

    pub fn unwrap_mut(&self) -> impl DerefMut<Target = T> + '_ {
        return self.0.borrow_mut();
    }
}

impl<T> Clone for RcRefCellSharedPointer<T> {
    fn clone(&self) -> Self {
        return Self(Rc::clone(&self.0));
    }
}
