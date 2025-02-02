pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub page_size: usize,
    pub max_page_size: usize,
    pub page_number: usize,
    pub max_page_number: usize,
}
