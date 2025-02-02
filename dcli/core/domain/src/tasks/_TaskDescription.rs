use std::ops::Deref;

#[derive(Clone)]
pub struct TaskDescription(String);

impl TaskDescription {
    const MIN_LENGTH: usize = 1;
    const MAX_LENGTH: usize = 1024;

    fn ensure_no_trailing_and_leading_whitespaces(description: String) -> String {
        return description.trim().to_string();
    }

    fn ensure_min_length(description: &String) -> Result<(), TaskDescriptionError> {
        if description.len() < Self::MIN_LENGTH {
            return Err(TaskDescriptionError::TooShort {
                actual_length: description.len(),
                min_length_required: Self::MIN_LENGTH,
            });
        }

        return Ok(());
    }

    fn ensure_max_length(description: &String) -> Result<(), TaskDescriptionError> {
        if description.len() > Self::MAX_LENGTH {
            return Err(TaskDescriptionError::TooLong {
                actual_length: description.len(),
                max_length_allowed: Self::MAX_LENGTH,
            });
        }

        return Ok(());
    }
}

impl Deref for TaskDescription {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl TryFrom<String> for TaskDescription {
    type Error = TaskDescriptionError;
    
    fn try_from(description: String) -> Result<Self, Self::Error> {
        let description = Self::ensure_no_trailing_and_leading_whitespaces(description);

        Self::ensure_min_length(&description)?;
        Self::ensure_max_length(&description)?;

        return Ok(TaskDescription(description));
    }
}

pub enum TaskDescriptionError {
    TooShort {
        actual_length: usize,
        min_length_required: usize,
    },
    TooLong {
        actual_length: usize,
        max_length_allowed: usize,
    },
}
