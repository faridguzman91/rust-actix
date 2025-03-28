use actix_web::{
    get, patch, post, web::Data, web::Json, web::Path, App, HttpResponse, HttpServer, Responder,
};
use error::PizzaError;
use uuid::Uuid;
mod db;
mod models;
use crate::db::Database;
use crate::models::pizza::{BuyPizzaRequest, Pizza, UpdatePizzaURL};
use validator::Validate;
use validator::ValidationErrors;
mod error;

// Get all pizzas
#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    let pizzas = db.get_all_pizzas().await;
    match pizzas {
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(PizzaError::NoPizzasFound),
    }
}

// Buy a pizza
#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> impl Responder {
    let is_valid: Result<(), ValidationErrors> = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name: String = body.pizza_name.clone();

            let new_uuid = Uuid::new_v4().to_string();

            let new_pizza = db.add_pizza(Pizza::new(new_uuid, pizza_name)).await;

            match new_pizza {
                Some(created) => {
                    HttpResponse::Ok().body(format!("Created new pizza: {:?}", created))
                }
                None => HttpResponse::InternalServerError().body("Error buying pizza!"),
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Pizza name required!"), // ✅ 400 Bad Request
    }
}

// Update a pizza
#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating a pizza with the {uuid}"))
}

// Actix-web main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await.expect("error connecting to db");

    let db_data = Data::new(db);

    HttpServer::new(move || {
        // move db_data into closure because of Actix requirements
        App::new()
            .app_data(db_data.clone()) // Clone for multiple references to db
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Test Endpoints:
// curl -i -X POST http://localhost:8080/buypizza -H 'Content-Type: application/json' -d '{"pizza_name": "triple cheese"}'
// curl -i -X GET http://localhost:8080/pizzas
// curl -i -X PATCH http://localhost:8080/updatepizza/1234567890
