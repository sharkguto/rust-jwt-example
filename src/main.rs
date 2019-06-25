// fn main() {
//     println!("Hello, world!");
// }

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    let output = "Hello, world!";
    output
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
