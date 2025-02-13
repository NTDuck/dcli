use std::usize;

#[derive(ddd::ValueObject)]
pub struct PaginationRequest {
    pub pageNumber: usize,
    pub maxPageSize: usize,
}
