use std::fmt::Debug;

use axiom::interfaces::DataTransferObject;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginationResponse<T> {
    pub items: Vec<T>,
    pub pageSize: usize,
    pub maxPageSize: usize,
    pub pageNumber: usize,
    pub maxPageNumber: usize,
}

impl<T> DataTransferObject for PaginationResponse<T>
where
    T: DataTransferObject,
{}
