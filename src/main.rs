use actix_web::{get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
mod models;
use crate::models::pizza::BuyPizzaRequest;
use validator::Validate;
use validator::ValidationErrors;

//actic_Web get macro handler to get_pizzas
#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    //return 200 ok with string body
    HttpResponse::Ok().body("Pizzas available")
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid: Result<(), ValidationErrors> = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name: String = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Pizza entered is {pizza_name}"))
        }
        Err(_) => HttpResponse::Ok().body(format!("Pizza name required!")),
    }
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
