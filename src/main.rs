use actix_web::{ get, post, patch, Responder, HttpResponse };

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available")
}

fn main() {
    println!("Hello, world!");
}
