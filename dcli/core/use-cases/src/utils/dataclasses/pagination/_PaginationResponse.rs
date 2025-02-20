use std::fmt::Debug;

use axiom::interfaces::DataTransferObject;
use serde::Deserialize;
use serde::Serialize;

#[derive(DataTransferObject, Serialize, Deserialize)]
pub struct PaginationResponse<T>
where
    // Viable workaround since `T: DataTransferObject` does not work properly
    T: Debug + Send + Sync + Clone + Serialize + for<'de2> Deserialize<'de2>,
{
    pub items: Vec<T>,
    pub pageSize: usize,
    pub maxPageSize: usize,
    pub pageNumber: usize,
    pub maxPageNumber: usize,
}
