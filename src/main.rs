// fn main() {
//     println!("Hello, world!");
// }

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::http::RawStr;

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/")]
fn index() -> &'static str {
    let output = "Hello, world!";
    output
}


fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, hello])
}

fn main() {
    rocket().launch();
}
