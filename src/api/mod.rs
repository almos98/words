use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

use rocket::{http::Status, Data};
use rocket_contrib::json::JsonValue;

// Returns a list of all files found under the directory specified in create::LISTS_DIR constant.
#[get("/lists")]
pub fn get_lists() -> JsonValue {
    let lists = match fs::read_dir(crate::LISTS_DIR) {
        Ok(read) => read
            .map(|res| res.map(|entry| entry.file_name().into_string().unwrap()))
            .collect::<Result<Vec<String>, io::Error>>()
            .unwrap(),

        Err(e) => {
            eprintln!("Error while reading directory: {}", e);
            vec![]
        }
    };

    json!(lists)
}

// Returns the words of list <list_name>
#[get("/list/<list_name>")]
pub fn get_list(list_name: String) -> String {
    let f = OpenOptions::new()
        .read(true)
        .open(Path::new(crate::LISTS_DIR).join(&list_name));

    match f {
        Ok(mut file) => {
            let mut buffer = String::new();
            if let Err(e) = file.read_to_string(&mut buffer) {
                eprintln!("Error while reading list {}: {}", &list_name, e);
            }

            buffer
        }
        Err(e) => {
            eprintln!("Error while reading list {}: {}", &list_name, e);
            String::new()
        }
    }
    .replace("\n", "")
}

// Add the words to list <list_name>
#[post("/list/<list_name>", data = "<words>")]
pub fn add_words_to_list(list_name: String, words: Data) -> Status {
    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(Path::new(crate::LISTS_DIR).join(&list_name));

    match f {
        Ok(mut file) => {
            let mut stream = words.open();
            let mut buffer = String::new();

            if let Err(e) = stream.read_to_string(&mut buffer) {
                eprintln!("Error while reading data from request: {}", e);
            }

            if let Err(e) = writeln!(file, ",{}", buffer) {
                eprintln!("Error while writing list {}: {}", &list_name, e);
                return Status::InternalServerError;
            }

            Status::Ok
        }
        Err(e) => {
            eprintln!("Error while writing list {}: {}", &list_name, e);
            Status::InternalServerError
        }
    }
}

// Get words from some lists
#[get("/words?<lists>")]
pub fn get_words(lists: Option<String>) -> String {
    let lists_to_search: Vec<String> = match lists {
        None => get_lists()
            .as_array()
            .unwrap_or(&Vec::new())
            .into_iter()
            .map(|s| String::from(s.as_str().unwrap_or_default()))
            .collect(),
        Some(s) => String::from(s)
            .split(",")
            .map(|s| String::from(s))
            .collect(),
    };

    let word_list: Vec<String> = lists_to_search
        .into_iter()
        .map(|list_name| get_list(list_name))
        .collect();

    word_list.join(",").replace("\n", "")
}
