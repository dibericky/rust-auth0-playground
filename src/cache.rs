use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use redis::{Connection, Commands};

use crate::extractors::RedisConfig;


pub struct RedisCache {
    conn: Box<Connection>
}

impl RedisCache {
    pub async fn new(config: RedisConfig) -> Result<Self> {
        let str_conn = format!("redis://{}:{}@{}:{}", config.username, config.password, config.host, config.port);
        let client = redis::Client::open(str_conn)?;
        let conn = client.get_connection()?;
        Ok(Self {
            conn: Box::new(conn)
        })
    }
}

impl Cache for RedisCache {
    fn set(self: &mut Self, key:String,value:String) -> Result<()>  {
        let mut con = &mut *self.conn;
       // redis_conn.set(key, value);
       let _ : () = con.set("my_key", 42)?;
        Ok(())
    }

    fn get(self: &Self, key:String) -> String {
        todo!()
    }
}

#[async_trait]
pub trait Cache {
    fn set(self: &mut Self, key: String, value: String) -> Result<()>;
    fn get(self: &Self, key: String) -> String;
}