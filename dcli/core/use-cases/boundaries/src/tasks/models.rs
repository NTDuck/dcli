use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub struct TaskModel {
    pub id: u64,
    pub description: String,
    pub status: TaskStatusModel,
    pub created_at: String,
}

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub enum TaskStatusModel {
    Pending,
    InProgress,
    Completed,
}
