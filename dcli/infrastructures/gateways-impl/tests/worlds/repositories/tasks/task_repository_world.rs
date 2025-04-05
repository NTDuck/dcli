use std::ops::RangeInclusive;

use domain::tasks::Task;
use rayon::prelude::*;
use use_cases::{dataclasses::pagination::UNBOUNDED_PAGINATION_REQUEST, gateways::repositories::tasks::TaskRepository};

use crate::params::Param;

pub struct TaskRepositoryWorld<Handle: TaskRepositoryWorldHandle> {
    pub task_repository: Handle::TaskRepository,
}

pub trait TaskRepositoryWorldHandle {
    type TaskRepository: TaskRepository;

    fn new() -> Self::TaskRepository;
}

impl<Handle: TaskRepositoryWorldHandle> TaskRepositoryWorld<Handle> {
    pub fn given_empty_repository() -> Self {
        Self {
            task_repository: Handle::new(),
        }
    }

    pub fn given_repository_with_task(idx: usize) -> Self {
        let mut world = Self::given_empty_repository();

        let task = Param(idx).into();
        world.task_repository.save(task);

        world
    }

    pub fn given_repository_with_task_range(idx_range: RangeInclusive<usize>) -> Self {
        let mut world = Self::given_empty_repository();

        idx_range
            .map(|idx| Param(idx).into())
            .for_each(|task| world.task_repository.save(task));

        world
    }

    pub fn when_adding_task(&mut self, idx: usize) {
        let task = Param(idx).into();
        self.task_repository.save(task);
    }

    pub fn when_removing_task(&mut self, idx: usize) {
        let task_id = Param(idx).into();
        self.task_repository.remove(task_id);
    }

    pub fn then_repository_is_empty(&self) {
        let pagination_request = UNBOUNDED_PAGINATION_REQUEST;
        let pagination_response = self.task_repository.show_reverse_chronologically_ordered(pagination_request);
    
        assert!(pagination_response.items.is_empty());
        assert!(pagination_response.page_size == 0);
    }

    pub fn then_repository_contains_only_task(&self, idx: usize) {
        let task: Task = Param(idx).into();

        assert!(self.task_repository.contains(task.id));
        assert!(self.task_repository.get_by_id(task.id) == Some(task.clone()));
    
        let pagination_request = UNBOUNDED_PAGINATION_REQUEST;
        let pagination_response = self.task_repository.show_reverse_chronologically_ordered(pagination_request);
    
        assert!(pagination_response.items == vec![task]);
        assert!(pagination_response.page_size == 1);
    }

    pub fn then_repository_contains_task_range_except(&self, idx_range: RangeInclusive<usize>, idx_except: usize) {
        let task_except = Param(idx_except).into();

        let tasks = idx_range
            .into_par_iter()
            .map(|idx| Param(idx).into())
            .filter(|task| *task != task_except)
            .collect::<Vec<Task>>();

        assert!(tasks
            .par_iter()
            .map(|task| task.id)
            .all(|task_id| self.task_repository.contains(task_id)));
        assert!(tasks
            .par_iter()
            .cloned()
            .all(|task| self.task_repository.get_by_id(task.id) == Some(task)));

        let pagination_request = UNBOUNDED_PAGINATION_REQUEST;
        let pagination_response = self.task_repository.show_reverse_chronologically_ordered(pagination_request);

        assert!(pagination_response.items == tasks);
        assert!(pagination_response.page_size == tasks.len());
    }
}
