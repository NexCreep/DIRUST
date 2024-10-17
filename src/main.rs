pub mod entities;

use entities::files::Files;
use rocket::serde::json::Json;

use self::entities::file::File;

#[macro_use] extern crate rocket;

#[get("/files")]
fn list_files() -> Json<Files> {

    let file: File = File {
        name: String::from("hello"),
        format: String::from("txt")
    };

    let files: Vec<File> = vec![file];

    return Json(Files { files: files })
}

#[get("/folder/<destination>")]
fn get_dir_fs(destination: &str) -> String {

    let destination_chars: Vec<char> = destination.chars().collect();
    let mut m_destination_chars: Vec<char> = destination_chars.clone();

    for (i, destination_char) in destination_chars.iter().enumerate() {
        if destination_char.to_owned() == ':'  {
            m_destination_chars[i] = '/'
        }
    };

    let mut final_destination_chars: Vec<char> = vec!['/'];
    final_destination_chars.append(&mut m_destination_chars);

    return final_destination_chars.into_iter().collect()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![list_files, get_dir_fs])
}
