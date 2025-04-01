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
        let description =
            Self::remove_trailing_and_leading_whitespaces(&description);

        Self::ensure_no_length_underflow(description)?;
        Self::ensure_no_length_overflow(description)?;

        let description = description.to_string();
        Ok(TaskDescription(description))
    }
}

impl TaskDescription {
    fn remove_trailing_and_leading_whitespaces(description: &str) -> &str {
        description.trim()
    }

    fn ensure_no_length_underflow(
        description: &str,
    ) -> Result<(), TaskDescriptionError> {
        if description.len() < MIN_LENGTH_REQUIRED {
            return Err(TaskDescriptionError::LengthUnderflow {
                actual_length: description.len(),
                min_length_required: MIN_LENGTH_REQUIRED,
            });
        }

        Ok(())
    }

    fn ensure_no_length_overflow(
        description: &str,
    ) -> Result<(), TaskDescriptionError> {
        if description.len() > MAX_LENGTH_ALLOWED {
            return Err(TaskDescriptionError::LengthOverflow {
                actual_length: description.len(),
                max_length_allowed: MAX_LENGTH_ALLOWED,
            });
        }

        Ok(())
    }
}

const MIN_LENGTH_REQUIRED: usize = 1;
const MAX_LENGTH_ALLOWED: usize = 1024;

#[derive(DataTransferObjectWithoutSerde, Serialize, Deserialize)]
pub enum TaskDescriptionError {
    LengthUnderflow {
        actual_length: usize,
        min_length_required: usize,
    },
    LengthOverflow {
        actual_length: usize,
        max_length_allowed: usize,
    },
}
