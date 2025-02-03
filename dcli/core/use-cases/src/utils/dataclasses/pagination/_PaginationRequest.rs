#[derive(ddd::ValueObject)]
pub struct PaginationRequest {
    pub page_number: usize,
    pub max_page_size: usize,
}
