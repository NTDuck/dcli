use rspec::suite::Suite;

use crate::environments::repositories::tasks::TaskRepositoryEnvironment;
use crate::environments::repositories::tasks::TaskRepositoryEnvironmentHandle;

pub struct TaskRepositorySuite;

impl TaskRepositorySuite {
    pub fn with<Handle: TaskRepositoryEnvironmentHandle>(env: TaskRepositoryEnvironment<Handle>) -> Suite<TaskRepositoryEnvironment<Handle>> {
        rspec::suite(std::any::type_name::<Handle::TaskRepository>().rsplit("::").next().unwrap(), env, |_ctx| {
            
        })
    }
}
