use rocket::serde::Serialize;

use super::file::File;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Files {
    pub files: Vec<File>
}