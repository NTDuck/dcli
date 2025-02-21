use axiom::interfaces::deriveDataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

#[derive(deriveDataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub pageNumber: usize,
    pub maxPageSize: usize,
}
