use domain::Task;
use domain::TaskId;

pub(crate) fn ExpectNone(retrievedTask: Option<Task>) {
    assert!(retrievedTask.is_none());
}

pub(crate) fn ExpectCorrectTask(retrievedTask: Option<Task>, givenTaskId: u128) {
    assert!(retrievedTask.is_some());
    assert_eq!(retrievedTask.unwrap().id, TaskId::from(givenTaskId));
}

pub(crate) fn ExpectFalse(predicate: bool) {
    assert!(!predicate);
}

pub(crate) fn ExpectTrue(predicate: bool) {
    assert!(predicate);
}
