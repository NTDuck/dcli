use axiom::interfaces::deriveDataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::dataclasses::pagination::PaginationProperties;
use crate::utils::dataclasses::pagination::PaginationRequest;

#[derive(deriveDataTransferObjectWithoutSerde, Serialize, Deserialize)]
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
