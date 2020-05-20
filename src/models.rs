use serde::Serialize;
use serde::Deserialize;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize)]
pub struct RedirectionRequest {
   pub u: String
}

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="redirects")]
pub struct Redirects {
    pub caller_ip: String, 
    pub called_url: String
}