#[cfg(test)] mod tests;

use rocket::fs::{FileServer, relative};
use rocket::post;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Write;
// If we wanted or needed to serve files manually, we'd use `NamedFile`. Always
// prefer to use `FileServer`!
mod manual {
    use std::path::{PathBuf, Path};
    use rocket::fs::NamedFile;

    #[rocket::get("/second/<path..>")]
    pub async fn second(path: PathBuf) -> Option<NamedFile> {
        let mut path = Path::new(super::relative!("static")).join(path);
        if path.is_dir() {
            path.push("index.html");
        }

        NamedFile::open(path).await.ok()
    }
}

#[post("/submit", data = "<data>")]
fn submit(data: String) -> String {
    
    println!("{}", format!("{}", data));
    let mut object: Value = serde_json::from_str(&data).unwrap();
    
    if let Some(name) = object.get_mut("name") {
        *name = "new name".into();
    }
    println!("{}", object["data"]);
    if object["data"] == String::from("Pineapple"){
        write_read_add("pineapple.txt");
    }else{
        write_read_add("npineapple.txt");
    }
    
    format!("You submitted: {}", data)
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![submit, manual::second])
        .mount("/", FileServer::from(relative!("static")))
}

fn write_read_add(file: &str) {

    
    //read file

    let file_path = file;
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //change file 
    let mut my_int: u8 = contents.parse::<u8>().unwrap();
    my_int = my_int + 1;

    //create file
    let mut file = File::create(file_path)
   .expect("Error encountered while creating file!");

   write!(&mut file, "{my_int}");
}