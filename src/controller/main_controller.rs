use actix_files::NamedFile;
use actix_web::{get, Error};
use std::path::PathBuf;

#[get("/")]
pub async fn get_index() -> Result<NamedFile, Error> {
    let path: PathBuf = "public/index.html".into();
    Ok(NamedFile::open(path)?)
}
