use actix_web::{ get, patch, post, App, HttpResponse, HttpServer, Responder };

//actic_Web get macro
#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available")
}

//actix_web macro
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
             .service(get_pizzas)
    })
        .bind("127.0.0.1:8080")?.run().await
}
