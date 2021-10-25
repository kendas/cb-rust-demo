use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct ErrorResponse {
    message: String,
    fields: Option<Vec<FieldValidationError>>,
}

impl ErrorResponse {
    pub fn with_validation_errors(message: String, errors: Vec<FieldValidationError>) -> Self {
        ErrorResponse {
            message,
            fields: Some(errors),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct FieldValidationError {
    name: String,
    error: String,
}

impl FieldValidationError {
    pub fn new(name: String, error: String) -> Self {
        FieldValidationError { name, error }
    }
}

pub trait Validated {
    fn validate(&self) -> Result<(), Vec<FieldValidationError>>;
}
