#[macro_use]
extern crate rocket;

use rocket::response::{content::RawHtml, status::NotFound};

use include_dir::{include_dir, Dir};
const PROJECT_DIR: Dir = include_dir!("dist");

#[get("/<path>")]
fn files(path: &str) -> Result<RawHtml<&str>, NotFound<&str>> {
    match PROJECT_DIR.get_file(path) {
        Some(file) => Ok(RawHtml(file.contents_utf8().unwrap())),
        None => Err(NotFound("Couldn't Find File")),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![files])
}
