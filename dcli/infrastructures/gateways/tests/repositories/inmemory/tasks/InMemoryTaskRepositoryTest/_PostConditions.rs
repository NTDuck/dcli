use domain::Task;
use domain::TaskId;

pub fn ExpectNone(retrievedTask: Option<Task>) {
    assert!(retrievedTask.is_none());
}

pub fn ExpectCorrectTask(retrievedTask: Option<Task>, givenTaskId: u128) {
    assert!(retrievedTask.is_some());
    assert_eq!(retrievedTask.unwrap().id, TaskId::from(givenTaskId));
}
