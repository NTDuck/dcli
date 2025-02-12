use domain::Task;
use domain::TaskId;

pub trait TaskPool<const N: usize> {
    fn new(tasks: impl Iterator<Item = Task>) -> Self;

    fn getRandomTask(&self) -> &Task;

    fn getRandomId(&self) -> TaskId {
        let randomTask = self.getRandomTask();
        return randomTask.id;
    }

    fn contains(&self, taskId: TaskId) -> bool;

    fn size(&self) -> usize {
        return N;
    }
}
