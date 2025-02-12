use domain::Task;
use rand::Rng;
use rayon::prelude::*;

use crate::utils::pools::tasks::abc::TaskPool;

pub struct ArrayBasedTaskPool<const N: usize> {
    tasks: [Task; N],
}

impl<const N: usize> TaskPool<N> for ArrayBasedTaskPool<N> {
    fn new(mut tasks: impl Iterator<Item = Task>) -> Self {
        return Self {
            tasks: std::array::from_fn(|_|
                tasks.next().unwrap()),
        };
    }

    fn getRandomTask(&self) -> &Task {
        let index = rand::rng().random_range(0..N);
        return &self.tasks[index];
    }

    fn contains(&self, taskId: domain::TaskId) -> bool {
        return self.tasks
            .par_iter()
            .any(|task| task.id == taskId);
    }
}
