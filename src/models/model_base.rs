use std::sync::Arc;

use surrealdb::sql::Object;

use crate::{database::{Database, traits::{Creatable, Patchable}}, prelude::*};

use super::results::ModelMutationResult;

pub async fn model_get<E>(db: Arc<Database>, id: &str) -> Result<E>
where
    E: TryFrom<Object, Error = Error>,
{
    db.exec_get(id).await?.try_into()
}

pub async fn model_create<E>(db: Arc<Database>, entity: &'static str, data: E) -> Result<ModelMutationResult>
where
    E: Creatable {
    let id = db.exec_create(entity, data).await?;
    let result = ModelMutationResult::from(id);
    
    Ok(result)
}

pub async fn model_update<E>(db: Arc<Database>, id: &str, data: E) -> Result<ModelMutationResult>
where
    E: Patchable {
    let id = db.exec_merge(id, data).await?;
    let result = ModelMutationResult::from(id);
    
    Ok(result)
}

pub async fn model_soft_delete(db: Arc<Database>, id: &str) -> Result<ModelMutationResult> {
    let id = db.exec_soft_delete(id).await?;
    let result = ModelMutationResult::from(id);
    
    Ok(result)
}

pub async fn model_hard_delete(db: Arc<Database>, id: &str) -> Result<ModelMutationResult> {
    let id = db.exec_hard_delete(id).await?;
    let result = ModelMutationResult::from(id);
    
    Ok(result)
} 

pub async fn model_list<E>(db: Arc<Database>, entity: &'static str) -> Result<Vec<E>>
where
    E: TryFrom<Object, Error = Error> {
    let result = db.exec_select(entity).await?;
    result.into_iter().map(|v| v.try_into()).collect::<Result<_>>()
}
