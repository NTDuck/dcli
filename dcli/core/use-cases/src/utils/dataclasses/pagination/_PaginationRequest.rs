use axiom::interfaces::SerdelessDataTransferObject;
use serde::Deserialize;
use serde::Serialize;

#[derive(SerdelessDataTransferObject, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub pageNumber: usize,
    pub maxPageSize: usize,
}
