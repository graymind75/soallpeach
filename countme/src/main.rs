#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::config::{Config, Environment};
use rocket::{State, Data};
use rocket::response::status;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::Read;


#[post("/", data = "<data>")]
fn sum_route(counter: State<HitCount>, data: Data) -> status::Accepted<String> {
    let mut str_data = String::new();
    data.open().read_to_string(&mut str_data).expect("error in reading data!");
    let int_number: usize = str_data.parse().expect("data is not a number!");
    let result = int_number + counter.count.load(Ordering::SeqCst);
    counter.count.store(result, Ordering::SeqCst);
    status::Accepted(Some(format!("{}", result)))
}

#[get("/count")]
fn count_route(counter: State<HitCount>) -> status::Accepted<String> {
    status::Accepted(Some(format!("{}", counter.count.load(Ordering::SeqCst))))
}

struct HitCount {
    count: AtomicUsize
}

fn main() {
    let config = Config::build(Environment::Production)
        .address("localhost")
        .secret_key("sa4KVi6JOzGizfmXxckcUyrYXTU4IGgKFXHVrZeH050=")
        .port(80)
        .finalize();
    match config {
        Ok(config)=> {
            rocket::custom(config)
                .manage(HitCount{count: AtomicUsize::new(0)})
                .mount("/", routes![sum_route, count_route]).launch();
        }
        _ => {panic!("there is a problem!")}
    }
}
