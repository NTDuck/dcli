use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

use crate::pointers::strategies::abc::PointerStrategy;

pub struct RcRefCellPointerStrategy;

impl PointerStrategy for RcRefCellPointerStrategy {
    type Typed<T> = Rc<RefCell<T>>;
    type Untyped = Self::Typed<()>;

    fn typed_from_obj<T>(obj: T) -> Self::Typed<T> {
        return Rc::new(RefCell::new(obj));
    }

    fn shallow_copy_from_typed<T>(typed: &Self::Typed<T>) -> Self::Typed<T> {
        return Rc::clone(typed);
    }

    fn unwrap<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl Deref<Target = T> + 'br {
        return typed.borrow();
    }

    fn unwrap_mut<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl DerefMut<Target = T> + 'br {
        return typed.borrow_mut();
    }

    fn into_untyped<T>(typed: Self::Typed<T>) -> Self::Untyped {
        return unsafe {
            std::mem::transmute(typed)
        };
    }
}
