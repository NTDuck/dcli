use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

use crate::pointers::handles::PointerStrategy;

pub struct RcRefCellPointerStrategy;

impl PointerStrategy for RcRefCellPointerStrategy {
    type Typed<T> = Rc<RefCell<T>>;
    type Untyped = Self::Typed<()>;

    fn into_typed<T>(obj: T) -> Self::Typed<T> { Rc::new(RefCell::new(obj)) }

    fn into_untyped<T>(typed: Self::Typed<T>) -> Self::Untyped {
        unsafe { std::mem::transmute::<Self::Typed<T>, Self::Untyped>(typed) }
    }

    fn shallow_clone<T>(typed: &Self::Typed<T>) -> Self::Typed<T> {
        Rc::clone(typed)
    }

    fn as_ref<'br, T: 'br>(
        typed: &'br Self::Typed<T>,
    ) -> impl Deref<Target = T> + 'br {
        typed.borrow()
    }

    fn as_mut<'br, T: 'br>(
        typed: &'br Self::Typed<T>,
    ) -> impl DerefMut<Target = T> + 'br {
        typed.borrow_mut()
    }
}
