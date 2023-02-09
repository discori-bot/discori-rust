use std::{collections::BTreeMap, sync::Arc};

use surrealdb::sql::{Object, Value};

use crate::{utils::extract::{Extract, ExtractOrErr}, prelude::{Error, Result}, database::{Database, traits::{Creatable, Patchable}}};

use super::{model_base::{model_get, model_create, model_update, model_soft_delete, model_list}, results::ModelMutationResult};

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub current_streak: i64,
    pub max_streak: i64,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl TryFrom<Object> for User {
    type Error = Error;
    fn try_from(mut value: Object) -> Result<Self> {
        let user = User {
            id: value.extract_or_err("id")?,
            current_streak: value.extract_or_err::<i64>("current_streak")?,
            max_streak: value.extract_or_err::<i64>("max_streak")?,
            created_at: value.extract_or_err::<i64>("created_at")?.to_string(),
            deleted_at: value.extract::<i64>("deleted_at")?.map(|v| v.to_string()),
        };
        
        Ok(user)
    }
}

#[derive(Debug)]
pub struct UserForCreate {
    pub id: String,
}

impl Creatable for UserForCreate {}

impl From<UserForCreate> for Value {
    fn from(value: UserForCreate) -> Self {
        BTreeMap::from([
            ("id".into(), value.into())
        ]).into()
    }
}

#[derive(Debug)]
pub struct UserForUpdate {
    pub current_streak: Option<i64>,
    pub max_streak: Option<i64>,
}

impl Patchable for UserForUpdate {}

impl From<UserForUpdate> for Value {
    fn from(value: UserForUpdate) -> Self {
        let mut data = BTreeMap::new();

        if let Some(val) = value.current_streak {
            data.insert("current_streak".into(), val.into());
        }

        if let Some(val) = value.max_streak {
            data.insert("max_streak".into(), val.into());
        }
        
        data.into()
    }
}

pub struct UserModel;

impl UserModel {
    const ENTITY: &'static str = "user";

    pub async fn list(db: Arc<Database>) -> Result<Vec<User>> {
        model_list(db, UserModel::ENTITY).await
    }
    
    pub async fn get(db: Arc<Database>, id: &str) -> Result<User> {
        model_get(db, id).await
    }
    
    pub async fn create(db: Arc<Database>, data: UserForCreate) -> Result<ModelMutationResult> {
        model_create(db, UserModel::ENTITY, data).await
    }
    
    pub async fn update(db: Arc<Database>, id: &str, data: UserForUpdate) -> Result<ModelMutationResult> {
        model_update(db, id, data).await
    }
    
    pub async fn delete(db: Arc<Database>, id: &str) -> Result<ModelMutationResult> {
        model_soft_delete(db, id).await
    }
}
