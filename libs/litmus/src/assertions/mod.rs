pub fn ok() -> Result<(), libtest::Failed> { Ok(()) }

pub trait AssertionExt {
    fn ok(&self) -> Result<(), libtest::Failed> { Ok(()) }

    fn expect(
        &self,
        value: impl Into<Self>,
        err: impl Into<libtest::Failed>,
    ) -> Result<(), libtest::Failed>
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

impl<T> AssertionExt for T {}
