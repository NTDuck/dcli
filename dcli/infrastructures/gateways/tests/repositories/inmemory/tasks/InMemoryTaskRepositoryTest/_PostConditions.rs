use domain::utils::dataclasses::ids::Uuid;
use domain::Task;

pub(crate) fn ExpectNone(retrievedTask: Option<Task>) {
    assert!(retrievedTask.is_none());
}

pub(crate) fn ExpectCorrectTask(retrievedTask: Option<Task>, givenTaskId: u128) {
    assert!(retrievedTask.is_some());
    assert_eq!(retrievedTask.unwrap().id, Uuid::new(givenTaskId));
}

pub(crate) fn ExpectFalse(predicate: bool) {
    assert!(!predicate);
}

pub(crate) fn ExpectTrue(predicate: bool) {
    assert!(predicate);
}
