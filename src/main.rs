use std::env;
use dotenvy::dotenv;

mod db;

fn main() {
    dotenvy::dotenv().expect("Error loading .env");
    dotenv().ok();
    println!("{}", env::var("HOLA").expect("Error loading env var."));
    println!("Hello, world!");

    db::test();
}
