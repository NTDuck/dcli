pub unsafe trait SharedPointerType: Sized {
    fn new<T>(obj: T) -> Self;
}
