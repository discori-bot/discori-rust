use crate::prelude::*;
use surrealdb::sql::{Array, Object, Value};

impl TryFrom<Wrapper<Value>> for Object {
	type Error = Error;
	fn try_from(val: Wrapper<Value>) -> Result<Object> {
		match val.0 {
			Value::Object(obj) => Ok(obj),
			_ => Err(Error::ValueNotOfType("Object")),
		}
	}
}

impl TryFrom<Wrapper<Value>> for Array {
	type Error = Error;
	fn try_from(val: Wrapper<Value>) -> Result<Array> {
		match val.0 {
			Value::Array(obj) => Ok(obj),
			_ => Err(Error::ValueNotOfType("Array")),
		}
	}
}

impl TryFrom<Wrapper<Value>> for i64 {
	type Error = Error;
	fn try_from(val: Wrapper<Value>) -> Result<i64> {
		match val.0 {
			Value::Number(obj) => Ok(obj.as_int()),
			_ => Err(Error::ValueNotOfType("i64")),
		}
	}
}

impl TryFrom<Wrapper<Value>> for bool {
	type Error = Error;
	fn try_from(val: Wrapper<Value>) -> Result<bool> {
		match val.0 {
			Value::False => Ok(false),
			Value::True => Ok(true),
			_ => Err(Error::ValueNotOfType("bool")),
		}
	}
}

impl TryFrom<Wrapper<Value>> for String {
	type Error = Error;
	fn try_from(val: Wrapper<Value>) -> Result<String> {
		match val.0 {
			Value::Strand(strand) => Ok(strand.as_string()),
			Value::Thing(thing) => Ok(thing.to_string()),
			_ => Err(Error::ValueNotOfType("String")),
		}
	}
}
