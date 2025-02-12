use std::ops::Deref;
use std::ops::DerefMut;

pub trait PointerStrategy: Sized {
    type Typed<T>;
    type Untyped;   // = Self::Typed<()>;
    
    fn intoTyped<T>(obj: T) -> Self::Typed<T>;
    
    /// Rust does not assume binary compatibility
    /// so can't use `std::mem::transmute` here.
    /// 
    /// Cause undefined behaviour if `Self::Typed<T>` and `Self::Untyped`
    /// have different memory layouts.
    /// (e.g. if `Self::Typed<T>` stores type-specific metadata)
    fn intoUntyped<T>(typed: Self::Typed<T>) -> Self::Untyped;

    fn shallowClone<T>(typed: &Self::Typed<T>) -> Self::Typed<T>;

    fn read<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl Deref<Target = T> + 'br;
    fn write<'br, T: 'br>(typed: &'br Self::Typed<T>) -> impl DerefMut<Target = T> + 'br;

    fn checkBinaryCompatibility<T>() {
        std::hint::black_box(Self::intoUntyped::<T>);
    }
}
