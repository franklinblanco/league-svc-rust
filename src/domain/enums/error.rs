
pub enum Error {
    CommunicatorError(String),
    UknownError,
    UnexpectedStatusCode(u16, u16, String),
    NetworkError(String),
    SerdeError
}