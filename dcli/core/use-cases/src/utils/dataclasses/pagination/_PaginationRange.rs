use crate::utils::dataclasses::pagination::PaginationRequest;

use super::PaginationProperties;

pub struct PaginationRange {
    pub limit: usize,
    pub offset: usize,
}

impl From<&PaginationRequest> for PaginationRange {
    fn from(pagination_request: &PaginationRequest) -> Self {
        return Self {
            limit: pagination_request.page_number
                .saturating_sub(PaginationProperties::MIN_PAGE_SIZE)
                * pagination_request.max_page_size,
            offset: pagination_request.max_page_size,
        };
    }
}
