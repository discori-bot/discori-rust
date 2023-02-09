use std::collections::BTreeMap;

use surrealdb::{Datastore, Session, sql::{Object, thing, Datetime, Value, Array, parse}};

use crate::{prelude::*, utils::extract::{Extract, ExtractOrErr}};

use self::traits::{Creatable, Patchable};

mod extracts;
mod try_froms;

pub mod traits;

pub struct Database {
    ds: Datastore,
    session: Session,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let ds = Datastore::new("memory").await?;
        let session = Session::for_kv().with_db("discori").with_ns("discori");

        Ok(Database { ds, session })
    }
    
    pub async fn exec_get(&self, tid: &str) -> Result<Object> {
        let sql = "SELECT * FROM $th";
        let vars = BTreeMap::from([
            ("th".into(), thing(tid)?.into())
        ]);
        
        let responses = self.ds.execute(sql, &self.session, Some(vars), true).await?;
        let first_response = responses.into_iter().next().expect("no response returned from SELECT statement");
        
        Wrapper(first_response.result?.first()).try_into()
    }
    
    pub async fn exec_create(&self, tid: &str, data: impl Creatable) -> Result<String> {
        let sql = "CREATE $th CONTENT $data RETURN id";
        
        let mut data: Object = Wrapper(data.into()).try_into()?;
        let now = Datetime::default().timestamp_nanos();

        data.insert("created_at".into(), now.into());
        
        let vars = BTreeMap::from([
            ("th".into(), tid.into()),
            ("data".into(), data.into()),
        ]);
        
        let responses = self.ds.execute(sql, &self.session, Some(vars), false).await?;
        let first_response = responses.into_iter().next().map(|res| res.result).expect("id not returned from CREATE statement")?;
        
        if let Value::Object(mut val) = first_response.first() {
            val
                .extract_or_err::<String>("id")
                .map_err(|err| Error::DatabaseFailedToCreate(format!("exec_create on {tid}: {err}")))
        } else {
            Err(Error::DatabaseFailedToCreate(format!("exec_create on {tid}: nothing returned")))
        }
    }
    
    pub async fn exec_merge(&self, tid: &str, data: impl Patchable) -> Result<String> {
        let sql = "UPDATE $th MERGE $data RETURN id";
        
        let vars = BTreeMap::from([
            ("th".into(), thing(tid)?.into()),
            ("data".into(), data.into()),
        ]);
        
        let responses = self.ds.execute(sql, &self.session, Some(vars), true).await?;
        let first_response = responses.into_iter().next().map(|res| res.result).expect("id not returned from UDPATE MERGE statement")?;
        
        if let Value::Object(mut val) = first_response.first() {
            val
                .extract_or_err::<String>("id")
                .map_err(|err| Error::DatabaseFailedToCreate(format!("exec_merge on {tid}: {err}")))
        } else {
            Err(Error::DatabaseFailedToCreate(format!("exec_merge on {tid}: nothing returned")))
        }
    }
    
    pub async fn exec_soft_delete(&self, tid: &str) -> Result<String> {
        let sql= "UPDATE $th SET deleted_at = $now";
        let now = Datetime::default().timestamp_nanos();

        let vars = BTreeMap::from([
            ("deleted_at".into(), now.into()),
        ]);
        
        let responses = self.ds.execute(sql, &self.session, Some(vars), true).await?;
        let first_response = responses.into_iter().next().map(|v| v.result).expect("id not returned from UPDATE statement")?;
        
        if let Value::Object(mut val) = first_response.first() {
            val
                .extract_or_err::<String>("id")
                .map_err(|err| Error::DatabaseFailedToCreate(format!("exec_soft_delete on {tid}: {err}")))
        } else {
            Err(Error::DatabaseFailedToCreate(format!("exec_soft_delete on {tid}: nothing returned")))
        }
    }
    
    pub async fn exec_hard_delete(&self, tid: &str) -> Result<String> {
        let sql = "DELETE $th";
        let vars = BTreeMap::from([
            ("th".into(), tid.into()),
        ]);
        
        let responses = self.ds.execute(sql, &self.session, Some(vars), true).await?;
        let first_response = responses.into_iter().next().expect("no response returned from DELETE statement");
        
        // Return the error if there was any,
        first_response.result?;
        
        // and return the id if the query succeeded.
        Ok(tid.to_string())
    }
    
    pub async fn exec_select(&self, table: &str) -> Result<Vec<Object>> {
        let sql = "SELECT * FROM type::table($table) ORDER created_at DESC";
        let vars = BTreeMap::from([
            ("table".into(), table.into()),
        ]);
        
        let responses = self.ds.execute(sql, &self.session, Some(vars), true).await?;
        let first_response = responses.into_iter().next().expect("no response returned from SELECT statement");
        
        let arr: Array = Wrapper(first_response.result?).try_into()?;
        arr.into_iter().map(|v| Wrapper(v).try_into()).collect()
    }
}
