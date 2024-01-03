use casper_types::ApiError;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum NoirError{
    InvalidProof = 0
}
impl From<NoirError> for ApiError{
    fn from(e: NoirError) -> Self{
        ApiError::User(e as u16)
    }
}