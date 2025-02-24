use domain::utils::dataclasses::ids::Uuid;
use domain::Task;
use domain::TaskStatus;
use indexmap::IndexMap;
use use_cases::gateways::repositories::tasks::TaskRepository;
use use_cases::utils::dataclasses::pagination::PaginationProperties;
use use_cases::utils::dataclasses::pagination::PaginationRange;
use use_cases::utils::dataclasses::pagination::PaginationRequest;
use use_cases::utils::dataclasses::pagination::PaginationResponse;

pub struct InMemoryTaskRepository {
    tasksByIds: IndexMap<Uuid, Task>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        return Self {
            tasksByIds: IndexMap::new(),
        };
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn save(&mut self, task: Task) {
        self.tasksByIds.insert(task.id, task.clone());
    }

    fn remove(&mut self, taskId: Uuid) {
        self.tasksByIds.shift_remove(&taskId);
    }

    fn getById(&self, taskId: Uuid) -> Option<Task> {
        return self.tasksByIds.get(&taskId).cloned();
    }

    fn showChronologicallyOrdered(&self, paginationRequest: PaginationRequest) -> PaginationResponse<Task> {
        let orderedTasks = self.tasksByIds
            .values()
            .rev();
        return Self::paginate(orderedTasks, paginationRequest);
    }

    fn showChronologicallyOrderedByStatus(&self, status: TaskStatus, paginationRequest: PaginationRequest) -> PaginationResponse<Task> {
        let filteredAndOrderedTasks = self.tasksByIds
            .values()
            .filter(|task| task.status == status)
            .rev();
        return Self::paginate(filteredAndOrderedTasks, paginationRequest);
    }

    fn contains(&self, taskId: Uuid) -> bool {
        return self.tasksByIds.contains_key(&taskId);
    }

    fn clear(&mut self) {
        self.tasksByIds.clear();
    }

    fn clearByStatus(&mut self, status: TaskStatus) {
        self.tasksByIds
            .retain(|_, task| task.status != status);
    }
}

impl InMemoryTaskRepository {
    fn paginate<'a>(unpaginatedTasks: impl Iterator<Item = &'a Task>, paginationRequest: PaginationRequest) -> PaginationResponse<Task> {
        let paginationRange = PaginationRange::from(&paginationRequest);

        let unpaginatedTasks: Vec<_> = unpaginatedTasks.collect();
        let numberOfTasksBefore = unpaginatedTasks.len();

        let paginatedTasks: Vec<_> = unpaginatedTasks
            .into_iter()
            .skip(paginationRange.offset)
            .take(paginationRange.limit)
            .cloned()
            .collect();
        let numberOfTasksAfter = paginatedTasks.len();

        return PaginationResponse {
            items: paginatedTasks,
            pageSize: numberOfTasksAfter,
            maxPageSize: paginationRequest.maxPageSize,
            pageNumber: paginationRequest.pageNumber,
            maxPageNumber: Self::calcMaxPageNumber(numberOfTasksBefore, paginationRequest.maxPageSize),
        };
    }

    fn calcMaxPageNumber(numberOfTasks: usize, maxPageSize: usize) -> usize {
        return match numberOfTasks {
            0 => PaginationProperties::MinPageSize,
            _ => numberOfTasks.div_ceil(maxPageSize),
        };
    }
}
