use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;
use use_cases::dataclasses::pagination::PaginationRequest;
use use_cases::dataclasses::pagination::MIN_PAGE_NUMBER;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct PaginationRange {
    pub offset: usize,
    pub limit: usize,
}

impl From<&PaginationRequest> for PaginationRange {
    fn from(pagination_request: &PaginationRequest) -> Self {
        return Self {
            offset: pagination_request.page_number
                .saturating_sub(MIN_PAGE_NUMBER)
                .saturating_mul(pagination_request.max_page_size),
            limit: pagination_request.max_page_size,
        };
    }
}
