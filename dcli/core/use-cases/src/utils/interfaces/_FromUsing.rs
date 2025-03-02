use std::marker::PhantomData;

pub trait FromUsing<F, U> {
    fn from_using(from: F, using: U) -> Self;

    fn fromm(from: F) -> Using<Self, F, U>
    where
        Self: Sized,
    {
        return Using {
            from,
            _marker: PhantomData,
        };
    }
}

pub struct Using<T, F, U> {
    from: F,
    _marker: PhantomData<(T, U)>,
}

impl<T, F, U> Using<T, F, U>
where
    T: FromUsing<F, U>,
{
    pub fn using(self, using: U) -> T {
        return T::from_using(self.from, using);
    }
}
