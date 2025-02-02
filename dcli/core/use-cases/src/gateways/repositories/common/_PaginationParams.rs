pub struct PaginationParams {
    pub offset: usize,
    pub limit: usize,
}

impl PaginationParams {
    pub fn new(page_number: usize, max_page_size: usize) -> Self {
        let offset = (page_number.saturating_sub(1)) * max_page_size;
        let limit = max_page_size;

        return Self { offset, limit, };
    }
}
