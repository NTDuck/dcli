use ddd::{self, Entity};

#[derive(Clone, PartialEq, Eq)]
struct TaskIdentifier(u128);

impl ddd::Identifier for TaskIdentifier {}

struct Task {
    identifier: TaskIdentifier,
    description: String,
    is_active: bool,
}

impl ddd::Entity for Task {
    type Identifier = TaskIdentifier;

    fn get_id(&self) -> &Self::Identifier {
        return &self.identifier;
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        let self_id = self.get_id();
        let other_id = other.get_id();

        return self_id == other_id;
    }
}

impl Eq for Task {}

fn main() {
    let task = create_task();
    assert!(task.is_active);
    assert!(!task.description.is_empty())
}

fn create_task() -> Task {
    let id = TaskIdentifier(4);

    return Task {
        identifier: id,
        description: String::from("tomfoolery"),
        is_active: true,
    };
}
