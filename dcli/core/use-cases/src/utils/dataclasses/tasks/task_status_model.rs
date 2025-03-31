use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub enum TaskStatusModel {
    Pending,
    InProgress,
    Completed,
}
