use std::io::{self, Read, Write};
use std::fs::OpenOptions;

use rocket::Data;
use rocket::response::Debug;

#[put("/words", data = "<data>")]
pub fn add_words(data: Data) -> Result<&'static str, Debug<io::Error>> {
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("wordlist")
        .unwrap();
    
    let mut stream = data.open();
    let mut buffer = String::new();
    if let Err(e) = stream.read_to_string(&mut buffer) {
        eprintln!("Couldn't read the stream: {}", e);
    }

    if let Err(e) = writeln!(f, "{},", buffer) {
        eprintln!("Couldn't write to file: {}", e);
    }
    Ok("Words!")
}