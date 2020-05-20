use crate::models::{Status, Redirects, RedirectionRequest};
use actix_web::{web, Responder};
use actix_web::HttpRequest;
use crate::db;
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
                    .json(Status { status: "UP".to_string()})
}

pub async fn create_redirect(db_pool: web::Data<Pool>, req: HttpRequest, info: web::Query<RedirectionRequest>) -> impl Responder {
    let connectionInfo = req.connection_info();
    let client: Client = db_pool.get().await.expect("Error connecting to the database");
    let redirect: Redirects = Redirects {   caller_ip: connectionInfo.host().to_string(),
                                            called_url: info.u.clone()
                                        };

    db::create_redirect(&client, &redirect).await;

    web::HttpResponse::Found().with_header("Location", redirect.called_url)
}