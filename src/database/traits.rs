use surrealdb::sql::Value;

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}
