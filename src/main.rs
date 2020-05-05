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
        .mount(
            "/",
            routes![
                pages::index,
                pages::submit,
                pages::word_lists,
                static_content
            ],
        )
        .mount(
            "/api",
            routes![
                api::get_lists,
                api::get_list,
                api::add_words_to_list,
                api::get_all_words
            ],
        )
        .launch();
}
