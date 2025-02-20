use axiom::interfaces::DataTransferObject;
use serde::Deserialize;
use serde::Serialize;

#[derive(DataTransferObject, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub pageNumber: usize,
    pub maxPageSize: usize,
}
