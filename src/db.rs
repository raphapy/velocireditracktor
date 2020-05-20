use crate::models::{Redirects};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn create_redirect(client: &Client, red: &Redirects) -> () {
    let statement = client.prepare("insert into redirects (caller_ip, called_url) values ($1, $2)").await.unwrap();

    client.query(&statement, &[ &red.caller_ip, &red.called_url])
        .await
        .expect("Error creating an entry of redirect");
}