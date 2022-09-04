use std::env;

use anyhow::Result;
use clap::Parser;

use any2cast::config;
use any2cast::site::Site;

use log::info;

use std::path::PathBuf;

use actix_web::{
    get,
    web,
    App, HttpRequest, HttpResponse, HttpServer,
    Responder
};

use actix_files::NamedFile;

use actix_web::http::StatusCode;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(short, long, default_value_t = String::from("0.0.0.0"))]
    bind: String,

    #[clap(short, long, default_value_t = 8080)]
    port: u32,
}

#[get("/")]
async fn index(site: web::Data<Site<'_>>) -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(site.to_html().unwrap())
}

#[get("/p/{podcast}")]
async fn podcast(req: HttpRequest, site: web::Data<Site<'_>>, podcast: web::Path<String>) -> impl Responder {
    info!("Getting rss xml for podcast {}", podcast);

    let info = req.connection_info();
    let scheme = info.scheme();
    let host = info.host();

    match site.get_directory(&podcast) {
        Some(dir) => {
            let rss_xml = dir.to_rss_xml(&scheme, &host).unwrap();
            HttpResponse::Ok().body(rss_xml)
        },
        None => {
            HttpResponse::build(StatusCode::NOT_FOUND)
                .content_type("text/html; charset=utf-8")
                .body("")
        }
    }
}

#[get("/p/{podcast}/{media}")]
async fn media(_site: web::Data<Site<'_>>, p: web::Path<(String, String)>) -> impl Responder {
    let (podcast_name, media) = p.into_inner();
    info!("Getting mp3 for podcast {} file {}", &podcast_name, &media);

    let mut path = PathBuf::new();

    path.push(podcast_name.to_string());
    path.push(media.to_string());

    NamedFile::open(path).unwrap()
}

#[actix_web::main]
async fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    env_logger::init();

    config::init()?;

    let cli = Cli::parse();

    let cur_dir = env::current_dir()?;
    let bind = cli.bind;
    let port = cli.port as u16;

    HttpServer::new(move || {
        let mut site : Site = Site::new(
            cur_dir.to_str().unwrap().to_string(),
        ).unwrap();

        site.detect_directories().unwrap();
        site.prepare_static_files().unwrap();

        App::new()
            .app_data(web::Data::new(site))
            .service(index)
            .service(podcast)
            .service(media)
    })
        .bind((bind, port))?
        .run()
        .await?;


    Ok(())
}
