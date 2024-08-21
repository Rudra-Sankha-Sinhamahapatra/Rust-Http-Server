use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};
use rand::Rng;  
use env_logger::Env;

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    id: i64,
    name: String,
    rollno: i64,
    marks: i64,
    grade: String,  
}

#[derive(Debug, Deserialize, Serialize)]
struct Message1 {
    text: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Rust Web server!")
}

#[post("/echo")]
async fn echo(message: web::Json<Message>) -> impl Responder {
    HttpResponse::Ok().json(message.into_inner())
}

#[post("/reverse")]
async fn reverse(message: web::Json<Message1>) -> impl Responder {
    let reversed_text: String = message.text.chars().rev().collect();
    HttpResponse::Ok().json(Message1 {
        text: reversed_text,
    })
}

#[get("/random")]
async fn random() -> impl Responder {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(1..=100);
    HttpResponse::Ok().json(Message1 {
        text: random_number.to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("actix_web=info"));

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::new("%a %{User-Agent}i %r %s %b"))
            .service(hello)
            .service(echo)
            .service(reverse)
            .service(random)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
