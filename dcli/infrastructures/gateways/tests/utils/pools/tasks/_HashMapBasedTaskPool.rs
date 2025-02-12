use std::collections::HashMap;

use domain::Task;
use domain::TaskId;
use rand::Rng;

use crate::utils::pools::tasks::abc::TaskPool;

pub struct HashMapBasedTaskPool {
    tasksByIds: HashMap<TaskId, Task>,
}

impl<const N: usize> TaskPool<N> for HashMapBasedTaskPool {
    fn new(tasks: impl Iterator<Item = Task>) -> Self {
        return Self {
            tasksByIds: tasks
                .take(N)
                .map(|task| (task.id, task))
                .collect(),
        };
    }

    fn getRandomTask(&self) -> &Task {
        let index = rand::rng().random_range(0..N);
        return self.tasksByIds
            .values()
            .nth(index)
            .unwrap();
    }

    fn contains(&self, taskId: TaskId) -> bool {
        return self.tasksByIds.contains_key(&taskId);
    }
}
