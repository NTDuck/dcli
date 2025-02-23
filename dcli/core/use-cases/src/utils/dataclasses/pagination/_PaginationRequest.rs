use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub pageNumber: usize,
    pub maxPageSize: usize,
}

pub const UnboundedPaginationRequest: PaginationRequest = PaginationRequest {
    pageNumber: MinPageSize,
    maxPageSize: usize::MAX,
};

pub const MinPageSize: usize = 1;
