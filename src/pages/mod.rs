use rocket::response::NamedFile;

#[get("/")]
pub fn index() -> Option<NamedFile> {
    NamedFile::open("www/index.html").ok()
}

#[get("/submit")]
pub fn submit() -> Option<NamedFile> {
    NamedFile::open("www/submit.html").ok()
}

#[get("/word-lists")]
pub fn word_lists() -> Option<NamedFile> {
    NamedFile::open("wordlist").ok()
}