use surrealdb::sql::Object;

use crate::{utils::extract::ExtractImpl, prelude::{Wrapper, Result}};

impl ExtractImpl<String> for Object {
    fn extract_impl(&mut self, key: &str) -> Result<Option<String>> {
        let v = self.remove(key).map(|v| Wrapper(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(err)) => Err(err),
        }
    }
}

impl ExtractImpl<i64> for Object {
    fn extract_impl(&mut self, key: &str) -> Result<Option<i64>> {
        let v = self.remove(key).map(|v| Wrapper(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(err)) => Err(err),
        }
    }
}

impl ExtractImpl<bool> for Object {
    fn extract_impl(&mut self, key: &str) -> Result<Option<bool>> {
        let v = self.remove(key).map(|v| Wrapper(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(err)) => Err(err),
        }
    }
}
