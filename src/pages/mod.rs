use rocket::response::NamedFile;

#[get("/")]
pub fn index() -> Option<NamedFile> {
    NamedFile::open("www/index.html").ok()
}
