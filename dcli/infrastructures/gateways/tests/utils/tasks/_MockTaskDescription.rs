use domain::TaskDescription;
use fake::Dummy;
use fake::faker::lorem::en::Sentence;

#[derive(Dummy)]
pub struct MockTaskDescription(#[dummy(faker = "Sentence(1..1024)")] String);

impl Into<TaskDescription> for MockTaskDescription {
    fn into(self) -> TaskDescription {
        return TaskDescription::try_from(self.0).unwrap();
    }
}
