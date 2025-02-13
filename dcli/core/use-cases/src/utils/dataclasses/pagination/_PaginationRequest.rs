use std::usize;

use crate::utils::dataclasses::pagination::PaginationProperties;

#[derive(ddd::ValueObject)]
pub struct PaginationRequest {
    pub pageNumber: usize,
    pub maxPageSize: usize,
}

pub const UnboundedPaginationRequest: PaginationRequest = PaginationRequest {
    pageNumber: PaginationProperties::MinPageSize,
    maxPageSize: usize::MAX,
};
