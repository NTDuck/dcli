use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub struct PaginationResponse<T> {
    pub items: Vec<T>,
    pub page_size: usize,
    pub max_page_size: usize,
    pub page_number: usize,
    pub max_page_number: usize,
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
            page_size: self.page_size.clone(),
            max_page_size: self.max_page_size.clone(),
            page_number: self.page_number.clone(),
            max_page_number: self.max_page_number.clone(),
        };
    }
}
