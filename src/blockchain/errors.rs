use exonum::blockchain::ExecutionError;

#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    /// AlreadyExists.
    #[fail(display = "AlreadyExists")]
    AlreadyExists = 1,
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}