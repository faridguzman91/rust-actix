use actix_web::{ get, patch, post, App, HttpResponse, HttpServer, Responder };

//actic_Web get macro handler to get_pizzas
#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    //return 200 ok with string body
    HttpResponse::Ok().body("Pizzas available")
}

//actix_web macro
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        //register get pizzas route
        App::new()
             .service(get_pizzas)
    })
        // bind to localhost 8080
        .bind("127.0.0.1:8080")?.run().await
}
