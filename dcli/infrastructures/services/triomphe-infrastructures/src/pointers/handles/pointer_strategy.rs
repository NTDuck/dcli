use std::ops::Deref;
use std::ops::DerefMut;

pub trait PointerStrategy: Sized {
    type Typed<T>;
    /// `PointerStrategy::Untyped = PointerStrategy::Typed<()>`
    type Untyped;
    
    fn into_typed<T>(obj: T) -> Self::Typed<T>;
    
    /// Rust does not assume binary compatibility
    /// so can't use `std::mem::transmute` here.
    /// 
    /// Cause undefined behaviour if `Self::Typed<T>` and `Self::Untyped`
    /// have different memory layouts.
    /// (e.g. if `Self::Typed<T>` stores type-specific metadata)
    fn into_untyped<T>(typed: Self::Typed<T>) -> Self::Untyped;

    fn shallow_clone<T>(typed: &Self::Typed<T>) -> Self::Typed<T>;

    fn as_ref<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl Deref<Target = T> + 'br;
    fn as_mut<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl DerefMut<Target = T> + 'br;

    fn check_binary_compatibility<T>() {
        std::hint::black_box(Self::into_untyped::<T>);
    }
}
