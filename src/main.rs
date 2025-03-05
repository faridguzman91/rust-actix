use actix_web::{
    get, patch, post, App, HttpResponse, HttpServer, Responder,
};

mod models;

//actic_Web get macro handler to get_pizzas
#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    //return 200 ok with string body
    HttpResponse::Ok().body("Pizzas available")
}

#[post("/buypizza")]
async fn buy_pizza() -> impl Responder {
    HttpResponse::Ok().body("Buying a pizza")
}



#[patch("/updatepizza/{uuid}")]
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("Updating a pizza!")
}

//actix_web macro
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        //register get, buy, upddte pizzas route
        App::new()
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    // bind to localhost 8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
