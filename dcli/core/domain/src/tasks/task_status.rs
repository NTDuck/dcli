use axiom::interfaces::ddd;

#[derive(ddd::ValueObject)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}
