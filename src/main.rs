
#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use serde::Serialize;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, word, word_length, word_first, word_length_first]).attach(Template::fairing())
}

#[derive(Serialize)]
struct Response {
    word: String
}

#[get("/word")]
fn word() -> Json<Response> {
    let query = "SELECT word FROM dictionary ORDER BY RANDOM() LIMIT 1;";
    generate_response(query)
}

#[get("/word/length/<length>")]
fn word_length(length: u8) -> Json<Response> {
    let query = format!("SELECT word FROM dictionary WHERE LENGTH(word) = {} ORDER BY RANDOM() LIMIT 1;", length);
    generate_response(query.as_str())
}

#[get("/word/start/<str>")]
fn word_first(str: String) -> Json<Response> {
    let query = format!("SELECT word FROM dictionary WHERE word LIKE '{}%' ORDER BY RANDOM() LIMIT 1;", str);
    generate_response(query.as_str())
}

#[get("/word/length/<length>/start/<str>")]
fn word_length_first(length: u8, str: String) -> Json<Response> {
    let query = format!("SELECT word FROM dictionary WHERE word LIKE '{}%' AND LENGTH(word) = {} ORDER BY RANDOM() LIMIT 1;", str, length);
    generate_response(query.as_str())
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context!{})
}

fn generate_response(query: &str) -> Json<Response> {
    let connection = sqlite::open("dictionary.db").unwrap();
    let mut response = Response{word: String::from("no such word")};
    connection
        .iterate(query, |pairs| {
            for &(a, value) in pairs.iter() {
                response = Response{word: String::from(value.unwrap())};
            }
            true
        })
        .unwrap();
    Json(response)
}

