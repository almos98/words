#![warn(unused_extern_crates)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod api;
mod pages;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

const STATIC_DIR: &'static str = "./static/";
const LISTS_DIR: &'static str = "./lists/";

#[get("/static/<file..>")]
fn static_content(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(STATIC_DIR).join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![pages::index, static_content])
        .mount(
            "/api",
            routes![
                api::get_lists,
                api::create_list,
                api::get_list,
                api::update_list_no_append,
                api::update_list_append,
                api::delete_list
            ],
        )
        .launch();
}
