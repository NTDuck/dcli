use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::dataclasses::pagination::MinPageSize;
use crate::utils::dataclasses::pagination::PaginationRequest;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct PaginationRange {
    pub offset: usize,
    pub limit: usize,
}

impl From<&PaginationRequest> for PaginationRange {
    fn from(paginationRequest: &PaginationRequest) -> Self {
        return Self {
            offset: paginationRequest.pageNumber
                .saturating_sub(MinPageSize)
                .saturating_mul(paginationRequest.maxPageSize),
            limit: paginationRequest.maxPageSize,
        };
    }
}
