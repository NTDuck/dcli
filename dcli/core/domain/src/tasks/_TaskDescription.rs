use axiom::behaviours::NewType;
use axiom::interfaces::ddd;
use axiom::interfaces::DataTransferObjectWithoutSerde;
use serde::Deserialize;
use serde::Serialize;

#[derive(ddd::ValueObject, NewType)]
pub struct TaskDescription(String);

impl TryFrom<String> for TaskDescription {
    type Error = TaskDescriptionError;
    
    fn try_from(description: String) -> Result<Self, Self::Error> {
        let description = Self::removeTrailingAndLeadingWhitespaces(&description);

        Self::ensureNoLengthUnderflow(description)?;
        Self::ensureNoLengthOverflow(description)?;

        let description = description.to_string();
        return Ok(TaskDescription(description));
    }
}

impl TaskDescription {
    fn removeTrailingAndLeadingWhitespaces(description: &str) -> &str {
        return description.trim();
    }

    fn ensureNoLengthUnderflow(description: &str) -> Result<(), TaskDescriptionError> {
        if description.len() < MinLengthRequired {
            return Err(TaskDescriptionError::LengthUnderflow {
                actualLength: description.len(),
                minLengthRequired: MinLengthRequired,
            });
        }

        return Ok(());
    }

    fn ensureNoLengthOverflow(description: &str) -> Result<(), TaskDescriptionError> {
        if description.len() > MaxLengthAllowed {
            return Err(TaskDescriptionError::LengthOverflow {
                actualLength: description.len(),
                maxLengthAllowed: MaxLengthAllowed,
            });
        }

        return Ok(());
    }
}

const MinLengthRequired: usize = 1;
const MaxLengthAllowed: usize = 1024;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub enum TaskDescriptionError {
    LengthUnderflow {
        actualLength: usize,
        minLengthRequired: usize,
    },
    LengthOverflow {
        actualLength: usize,
        maxLengthAllowed: usize,
    },
}
