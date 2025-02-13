use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaginationResponse<T: ddd::ValueObject> {
    pub items: Vec<T>,
    pub pageSize: usize,
    pub maxPageSize: usize,
    pub pageNumber: usize,
    pub maxPageNumber: usize,
}

impl<T> ddd::ValueObject for PaginationResponse<T>
where
    T: ddd::ValueObject
{}
