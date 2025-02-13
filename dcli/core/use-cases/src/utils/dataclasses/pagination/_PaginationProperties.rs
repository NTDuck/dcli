use crate::utils::dataclasses::pagination::PaginationRequest;

pub struct PaginationProperties;

impl PaginationProperties {
    pub const MinPageSize: usize = 1;

    pub const UnboundedPaginationRequest: PaginationRequest = PaginationRequest {
        pageNumber: PaginationProperties::MinPageSize,
        maxPageSize: usize::MAX,
    };
}
