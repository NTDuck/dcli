use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub struct PaginationResponse<T> {
    pub items: Vec<T>,
    pub pageSize: usize,
    pub maxPageSize: usize,
    pub pageNumber: usize,
    pub maxPageNumber: usize,
}

impl<T> ddd::ValueObject for PaginationResponse<T>
where
    T: Debug + Clone + Eq,
{}

impl<T> Clone for PaginationResponse<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        return Self {
            items: self.items.clone(),
            pageSize: self.pageSize.clone(),
            maxPageSize: self.maxPageSize.clone(),
            pageNumber: self.pageNumber.clone(),
            maxPageNumber: self.maxPageNumber.clone(),
        };
    }
}
