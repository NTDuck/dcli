use axiom::behaviours::NewType;
use axiom::interfaces::ddd;
use axiom::interfaces::DataTransferObject;
use serde::Deserialize;
use serde::Serialize;

#[derive(ddd::ValueObject, NewType)]
pub struct TaskDescription(String);

impl TryFrom<String> for TaskDescription {
    type Error = TaskDescriptionError;
    
    fn try_from(description: String) -> Result<Self, Self::Error> {
        let description = Self::normalize(&description);
        Self::verify(&description)?;

        return Ok(TaskDescription(description));
    }
}

impl TaskDescription {
    const MIN_LENGTH: usize = 1;
    const MAX_LENGTH: usize = 1024;

    fn normalize(description: &str) -> String {
        let description = Self::removeTrailingAndLeadingWhitespaces(description);

        return description;
    }

    fn removeTrailingAndLeadingWhitespaces(description: &str) -> String {
        return description.trim()
            .to_string();
    }

    fn verify(description: &str) -> Result<(), TaskDescriptionError> {
        Self::ensureNoLengthUnderflow(description)?;
        Self::ensureNoLengthOverflow(description)?;

        return Ok(());
    }

    fn ensureNoLengthUnderflow(description: &str) -> Result<(), TaskDescriptionError> {
        if description.len() < Self::MIN_LENGTH {
            return Err(TaskDescriptionError::LengthUnderflow {
                actualLength: description.len(),
                minLengthRequired: Self::MIN_LENGTH,
            });
        }

        return Ok(());
    }

    fn ensureNoLengthOverflow(description: &str) -> Result<(), TaskDescriptionError> {
        if description.len() > Self::MAX_LENGTH {
            return Err(TaskDescriptionError::LengthOverflow {
                actualLength: description.len(),
                maxLengthAllowed: Self::MAX_LENGTH,
            });
        }

        return Ok(());
    }
}

#[derive(DataTransferObject, Serialize, Deserialize)]
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
