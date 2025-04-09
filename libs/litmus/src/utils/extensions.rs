pub trait ResultExt<E> {
    fn ok(&self) -> Result<(), E> {
        Ok(())
    }

    fn expect(&self, value: impl Into<Self>, err: impl Into<E>) -> Result<(), E>
    where
        Self: Sized + PartialEq,
    {
        if *self != value.into() {
            Err(err.into())
        } else {
            Ok(())
        }
    }
}

impl<T> ResultExt<libtest::Failed> for T {}
