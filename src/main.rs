use crate::error::PizzaError;
use crate::models::{BuyPizzaRequest, UpdatePizzaURL, Pizza};
use actix_web::web::Data;
use actix_web::{
    get, patch, post,
    web::{Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use db::database::Database;
use validator::Validate;
mod db;
mod models;
mod error;

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    let pizzas = db.get_all_pizzas().await;

    match pizzas {
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(PizzaError::NoPizzasFound)
    }
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> Result<Json<Pizza>,PizzaError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_pizza = db
                .add_pizza(Pizza::new(new_uuid.to_string(), pizza_name))
                .await;

            match new_pizza {
                Some(created) => {
                    Ok(Json(created))
                },
                None => Err(PizzaError::PizzaCreationFailure),
            }
        }
        Err(_) => Err(PizzaError::PizzaCreationFailure),
    }
}

#[patch("/updatepizza/{uuid}")]
async fn updatepizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    // let get_pizza_by_uuid = db.get

    HttpResponse::Ok().body(format!("Updating a Pizza with {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("error connecting to database");

    let db_data = actix_web::web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(updatepizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
