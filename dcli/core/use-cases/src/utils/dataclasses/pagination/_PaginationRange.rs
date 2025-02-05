use crate::utils::dataclasses::pagination::PaginationProperties;
use crate::utils::dataclasses::pagination::PaginationRequest;

#[derive(ddd::ValueObject)]
pub struct PaginationRange {
    pub offset: usize,
    pub limit: usize,
}

impl From<&PaginationRequest> for PaginationRange {
    fn from(pagination_request: &PaginationRequest) -> Self {
        return Self {
            offset: pagination_request.page_number
            .saturating_sub(PaginationProperties::MIN_PAGE_SIZE)
            * pagination_request.max_page_size,
            limit: pagination_request.max_page_size,
        };
    }
}