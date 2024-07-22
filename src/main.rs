#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, NamedFile, relative};
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize)]
struct Customer {
    name: String,
    email: String,
    phone: String,
}

#[derive(Serialize)]
struct AnonymizedCustomer {
    name: String,
    email: String,
    phone: String,
}

#[post("/anonymize", format = "json", data = "<customer>")]
fn anonymize(customer: Json<Customer>) -> Json<AnonymizedCustomer> {
    let anonymized = AnonymizedCustomer {
        name: anonymize_name(&customer.name),
        email: anonymize_email(&customer.email),
        phone: anonymize_phone(&customer.phone),
    };
    Json(anonymized)
}

fn anonymize_name(_name: &str) -> String {
    "Anonymous".to_string()
}

fn anonymize_email(_email: &str) -> String {
    "anonymous@example.com".to_string()
}

fn anonymize_phone(_phone: &str) -> String {
    "000-000-0000".to_string()
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(PathBuf::from("static/index.html")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, anonymize])
        .mount("/static", FileServer::from(relative!("static")))
}
