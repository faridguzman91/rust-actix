use serde::{ Deserialize, Serialize };
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct BuypizzaRequest {
    #[validate(length( min = 1, message="pizza name required"))]
    pub pizza_name: String
}

