use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub page_number: usize,
    pub max_page_size: usize,
}

pub const UNBOUNDED_PAGINATION_REQUEST: PaginationRequest = PaginationRequest {
    page_number: MIN_PAGE_SIZE,
    max_page_size: usize::MAX,
};

pub const MIN_PAGE_SIZE: usize = 1;
