use actix_files::NamedFile;
use actix_web::{get, HttpRequest, Result};
use std::path::PathBuf;

const ROOT_PATH : &str = "./public";
const HOST : &str = "127.0.0.1";
const PORT : &str = "8080";

mod api;
use api::get_data;

fn root() -> PathBuf {
    PathBuf::from(ROOT_PATH)
}

async fn serve_file(req: HttpRequest) -> Result<NamedFile> {
    let mut path = root();
    let filename: PathBuf = req.match_info().query("filename").parse().unwrap();
    path.push(filename);
    Ok(NamedFile::open(path)?)
}

#[get("/")]
async fn index() -> Result<NamedFile> {
    let mut path = PathBuf::from("./public");
    path.push("index.html");
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};
    
    HttpServer::new(|| App::new()
        .service(
            web::scope("/api")
                .service(get_data)
        )
        .service(index)
        .route("/{filename:.*}", web::get().to(serve_file)))
        .bind(format!("{}:{}", HOST, PORT))?
        .run()
        .await
}