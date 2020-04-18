#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod pages;
mod api;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/static/<file..>")]
fn static_content(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    let cfg = rocket::config::Config::build(rocket::config::Environment::Development)
        .address("localhost")
        .port(3000)   
    .   unwrap();

    rocket::custom(cfg)
        .mount("/", routes![pages::index, 
                                          pages::submit,
                                          pages::word_lists, 
                                          static_content])
        .mount("/api", routes![api::add_words])
        .launch();
}