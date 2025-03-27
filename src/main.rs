use actix_web::{web::Path, web::Data, get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
mod models;
mod db;
use crate::db::Database;
use crate::models::pizza::{ BuyPizzaRequest, UpdatePizzaURL };
use validator::Validate;
use validator::ValidationErrors;

//actic_Web get macro handler to get_pizzas
#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> impl Responder {
    let pizzas = db.get_all_pizzas().await;
    match pizzas {
        Some(found_pizzas) => HttpResponse::Ok().body(format!("{:?}", found_pizzas)),
        None => HttpResponse::InternalServerError().body("Error retrieving pizzas!"),
    }
}


//actix_web post macro handler to buy_pizza
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

//actix_web patch macro handler to update_pizza
#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating a pizza with the {uuid}"))
}

//actix_web macro
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = Database::init()
        .await
        .expect("error connecting to db");

    let db_data = Data::new(db);


    HttpServer::new( move || { // move db_data into closure because of type not implemeting the Copy
        // trait which is Actix requirement
        //register get, buy, upddte pizzas route
        App::new()
            .app_data(db_data.clone()) //clone to create multiple referencs to db
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    // bind to localhost 8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

//test endpoints

// curl -i -X POST http://localhost:8080/buypizza -H 'Content-Type: application/json' -d '{"pizza_name": "triple cheese"}'
// curl -i -X GET http://localhost:8080/pizzas
// curl -i -X PATCH http://localhost:8080/updatepizza/1234567890 



