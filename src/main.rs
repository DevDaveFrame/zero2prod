use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use rand::Rng;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn flip_a_coin() -> impl Responder {
    let mut rng = rand::thread_rng();
    let coin = rng.gen_range(0..2);
    if coin == 0 {
        format!("Heads")
    } else {
        format!("Tails")
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/coin", web::get().to(flip_a_coin))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
