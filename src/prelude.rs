pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;
pub struct Wrapper<T>(pub T);
