use std::ops::Deref;
use std::ops::DerefMut;

pub trait PointerStrategy {
    type Typed<T>;
    type Untyped;   // = Self::Typed<()>;

    fn typed_from_obj<T>(obj: T) -> Self::Typed<T>;
    fn shallow_copy_from_typed<T>(typed: &Self::Typed<T>) -> Self::Typed<T>;

    fn unwrap<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl Deref<Target = T> + 'br;
    fn unwrap_mut<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl DerefMut<Target = T> + 'br;

    /// Rust does not assume binary compatibility
    /// so can't use `std::mem::transmute` here.
    /// 
    /// Cause undefined behaviour if `Self::Typed<T>` and `Self::Untyped`
    /// have different memory layouts.
    /// (e.g. if `Self::Typed<T>` stores type-specific metadata)
    fn into_untyped<T>(typed: Self::Typed<T>) -> Self::Untyped;
}
