use actix_web::{App, HttpResponse, HttpServer, web};
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("http://localhost:8080") }),
            )
            .route(
                "/hello",
                web::get().to(|| async { HttpResponse::Ok().body("Hello, World!") }),
            )
            .route(
                "/json",
                web::get().to(|| async {
                    HttpResponse::Ok().json(serde_json::json!({"message": "Hello, JSON!"}))
                }),
            )
            .route(
                "/echo/{text}",
                web::get().to(|path: web::Path<String>| async move {
                    HttpResponse::Ok().body(format!("Echo: {}", path.into_inner()))
                }),
            )
            .route(
                "/person",
                web::post().to(|body: web::Json<Person>| async move {
                    HttpResponse::Ok().body(format!("Received: {}, {}", body.name, body.age))
                }),
            )
            .route(
                "/update/{id}",
                web::put().to(|path: web::Path<u32>, body: web::Json<Person>| async move {
                    HttpResponse::Ok().body(format!("Updated ID: {}, Name: {}, Age: {}", path.into_inner(), body.name, body.age))
                }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use serde::Deserialize;
#[derive(Deserialize)]
struct Person {
    name: String,
    age: u32,
}
