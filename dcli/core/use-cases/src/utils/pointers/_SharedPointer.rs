use super::implementations::*;

/// A type alias for the underlying shared pointer implementation.
/// 
/// This alias adheres to the following contract:
/// ```
/// pub trait SharedPointer<T: ?Sized>: Clone {
///     fn new(object: T) -> Self;
///     
///     fn unwrap(&self) -> impl std::ops::Deref<Target = T> + '_;
///     fn unwrap_mut(&self) -> impl std::ops::DerefMut<Target = T> + '_;
/// }
/// ```
///
/// **Limitations:**
/// - Not `dyn`-compatible
/// - Not as flexible as `archery`'s `SharedPointer<P>` and `SharedPointerType`
/// - Implementations must reside within the same architectural layer,
/// causing conflicts with Clean Architecture principles.
pub type SharedPointer<T> = ArcMutexSharedPointer<T>;
