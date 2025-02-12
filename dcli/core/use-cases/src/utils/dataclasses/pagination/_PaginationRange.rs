use crate::utils::dataclasses::pagination::PaginationProperties;
use crate::utils::dataclasses::pagination::PaginationRequest;

#[derive(ddd::ValueObject)]
pub struct PaginationRange {
    pub offset: usize,
    pub limit: usize,
}

impl From<&PaginationRequest> for PaginationRange {
    fn from(paginationRequest: &PaginationRequest) -> Self {
        return Self {
            offset: paginationRequest.pageNumber
                .saturating_sub(PaginationProperties::MinPageSize)
            * paginationRequest.maxPageSize,
            limit: paginationRequest.maxPageSize,
        };
    }
}
