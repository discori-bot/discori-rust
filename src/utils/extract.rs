use crate::prelude::{Result, Error};

/// Removes and returns the value for a given type and key.
/// Returns Result(None) if the key doesn't exist.
/// Returns an Err if the type is a mismatch.
pub trait ExtractImpl<T> {
    fn extract_impl(&mut self, key: &str) -> Result<Option<T>>;
}

/// Turbofish friendly implementation of the `ExtractImpl` trait.
/// Note: not to be implemented directly!
pub trait Extract {
    fn extract<T>(&mut self, key: &str) -> Result<Option<T>>
        where Self: ExtractImpl<T>;
}

/// Blanket implementation that will call the
/// underlying `ExtractImpl` implementation.
impl<A> Extract for A {
    fn extract<T>(&mut self, key: &str) -> Result<Option<T>>
            where Self: ExtractImpl<T> {
        self.extract_impl(key)
    }
}

/// Removes and returns the value for a given type and key.
/// Returns an Err if the key doesn't exist or the type is mismatch.
/// Note: not to be implemented directly!
pub trait ExtractOrErr {
    fn extract_or_err<T>(&mut self, key: &str) -> Result<T>
        where Self: ExtractImpl<T>;
}

/// Blanket implementation that will call the
/// underlying `ExtractImpl` implementation 
/// and return an Err if the key doesn't exist.
impl<A> ExtractOrErr for A {
    fn extract_or_err<T>(&mut self, key: &str) -> Result<T>
            where Self: ExtractImpl<T> {
        let val = self.extract_impl(key)?;
        val.ok_or_else(|| Error::PropertyNotFound(key.to_string()))
    }
}
