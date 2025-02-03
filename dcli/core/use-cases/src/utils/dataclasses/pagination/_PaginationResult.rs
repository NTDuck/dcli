#[derive(PartialEq, Eq)]
pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub page_size: usize,
    pub max_page_size: usize,
    pub page_number: usize,
    pub max_page_number: usize,
}

impl<T> ddd::ValueObject for PaginationResult<T>
where
    T: Clone + Eq,
{}

impl<T> Clone for PaginationResult<T>
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
