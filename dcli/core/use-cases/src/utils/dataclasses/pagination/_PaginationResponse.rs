use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct PaginationResponse<T> {
    pub items: Vec<T>,
    
    pub page_size: usize,
    pub max_page_size: usize,
    pub page_number: usize,
    pub max_page_number: usize,
}
