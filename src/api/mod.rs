use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use rocket::http;
use rocket_contrib::json::JsonValue;

fn path(file: &String) -> PathBuf {
    Path::new(crate::LISTS_DIR).join(file)
}

// Returns a list of all list names
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

// Create a new list
#[post("/list/<list_name>")]
pub fn create_list(list_name: String) -> http::Status {
    match fs::File::create(path(&list_name)) {
        Ok(_) => http::Status::Ok,
        Err(e) => {
            eprintln!("Error while creating list {}: {}", &list_name, e);
            http::Status::InternalServerError
        }
    }
}

// Get contents of a list
#[get("/list/<list_name>")]
pub fn get_list(list_name: String) -> String {
    let f = OpenOptions::new().read(true).open(path(&list_name));

    match f {
        Ok(mut file) => {
            let mut buffer = String::new();
            if let Err(e) = file.read_to_string(&mut buffer) {
                eprintln!("Error while readling list {}: {}", &list_name, e);
            }

            buffer
        }
        Err(e) => {
            eprintln!("Error while reading list {}: {}", &list_name, e);
            String::new()
        }
    }
    // Temporary until list sanitation
    .replace("\n", "")
}

// Update a list
#[put("/list/<list_name>", data = "<words>")]
pub fn update_list_no_append(list_name: String, words: rocket::Data) -> http::Status {
    update_list(list_name, words, false)
}

#[put("/list/<list_name>?append", data = "<words>")]
pub fn update_list_append(list_name: String, words: rocket::Data) -> http::Status {
    update_list(list_name, words, true)
}

pub fn update_list(list_name: String, words: rocket::Data, append: bool) -> http::Status {
    let f = OpenOptions::new()
        .write(true)
        .append(append)
        .create(false)
        .open(path(&list_name));

    match f {
        Ok(mut file) => {
            if !(append) {
                if let Err(e) = file.set_len(0) {
                    eprintln!("Error while clearing list {}: {}", &list_name, e);
                    return http::Status::InternalServerError;
                }
            }

            let mut stream = words.open();
            let mut buffer = String::new();

            if let Err(e) = stream.read_to_string(&mut buffer) {
                eprintln!("Error while reading data from request: {}", e);
                return http::Status::InternalServerError;
            }

            if let Err(e) = writeln!(file, ",{}", buffer) {
                eprintln!("Error while writing list {}: {}", &list_name, e);
                return http::Status::InternalServerError;
            }

            http::Status::Ok
        }
        Err(e) => {
            eprintln!("Error while writing list {}: {}", &list_name, e);
            http::Status::InternalServerError
        }
    }
}

// Remove a list
#[delete("/list/<list_name>")]
pub fn delete_list(list_name: String) -> http::Status {
    match fs::remove_file(path(&list_name)) {
        Ok(_) => http::Status::Ok,
        Err(e) => {
            eprintln!("Error while removing list {}: {}", &list_name, e);
            http::Status::InternalServerError
        }
    }
}

// Rename a list
#[put("/rename/<from>/<to>")]
pub fn rename_list(from: String, to: String) -> http::Status {
    // Don't want to overwrite the lists
    if path(&to).exists() {
        return http::Status::InternalServerError;
    }

    if let Err(e) = fs::rename(path(&from), path(&to)) {
        eprintln!("Error while renaming list {} to {}: {}", &from, &to, e);
        return http::Status::InternalServerError;
    }

    http::Status::Ok
}
