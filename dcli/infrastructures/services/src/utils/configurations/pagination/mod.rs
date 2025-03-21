use models::pagination::PaginationRequest;

pub const MIN_PAGE_SIZE: usize = 1;

pub const UNBOUNDED_PAGINATION_REQUEST: PaginationRequest = PaginationRequest {
    page_number: MIN_PAGE_SIZE,
    max_page_size: usize::MAX,
};
