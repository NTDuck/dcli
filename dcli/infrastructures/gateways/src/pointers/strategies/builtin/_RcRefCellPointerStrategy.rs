use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

use crate::pointers::handles::PointerStrategy;

pub struct RcRefCellPointerStrategy;

impl PointerStrategy for RcRefCellPointerStrategy {
    type Typed<T> = Rc<RefCell<T>>;
    type Untyped = Self::Typed<()>;

    fn intoTyped<T>(obj: T) -> Self::Typed<T> {
        return Rc::new(RefCell::new(obj));
    }
    
    fn intoUntyped<T>(typed: Self::Typed<T>) -> Self::Untyped {
        return unsafe {
            std::mem::transmute::<Self::Typed<T>, Self::Untyped>(typed)
        };
    }

    fn shallowClone<T>(typed: &Self::Typed<T>) -> Self::Typed<T> {
        return Rc::clone(typed);
    }

    fn read<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl Deref<Target = T> + 'br {
        return typed.borrow();
    }

    fn write<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl DerefMut<Target = T> + 'br {
        return typed.borrow_mut();
    }
}
