use actix_web::{web, HttpResponse, Responder};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

type Pool = bb8::Pool<PostgresConnectionManager<NoTls>>;

async fn handle_req_1(pool: &Pool) {
    let conn = pool.get().await.unwrap();
    conn.query_one("", &[]).await.unwrap();
}

async fn handle_req_2(pool: &Pool) {
    handle_req_1(pool).await;
}

async fn handle_req_3(pool: &Pool) {
    handle_req_2(pool).await;
}

async fn handle_req_4(pool: &Pool) {
    handle_req_3(pool).await;
}

async fn handle_req_5(pool: &Pool) {
    handle_req_4(pool).await;
}

async fn handle_req_final(pool: web::Data<Pool>) -> impl Responder {
    handle_req_5(pool.get_ref()).await;
    HttpResponse::Ok().finish()
}

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("").route("/api", web::post().to(handle_req_final)),
    );
}
