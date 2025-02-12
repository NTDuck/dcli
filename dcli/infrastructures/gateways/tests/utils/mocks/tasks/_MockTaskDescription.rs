use domain::TaskDescription;
use fake::Dummy;
use fake::faker::lorem::en::Word;

#[derive(Dummy)]
pub struct MockTaskDescription(#[dummy(faker = "Word()")] String);

impl Into<TaskDescription> for MockTaskDescription {
    fn into(self) -> TaskDescription {
        return TaskDescription::try_from(self.0)
            .unwrap();
    }
}
