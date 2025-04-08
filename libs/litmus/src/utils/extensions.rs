pub trait InfallibleExt<Args, E>: FnOnce(Args) -> () {
    fn infallible(self) -> impl FnOnce(Args) -> Result<(), E>;
}

impl<T, Args, E> InfallibleExt<Args, E> for T
where
    T: FnOnce(Args) -> (),
{
    fn infallible(self) -> impl FnOnce(Args) -> Result<(), E> {
        move |args| {
            (self)(args);
            Ok(())
        }
    }
}
