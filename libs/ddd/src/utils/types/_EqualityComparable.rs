pub trait EqualityComparable: PartialEq + Eq {}

impl<T> EqualityComparable for T
where T: PartialEq + Eq {}
