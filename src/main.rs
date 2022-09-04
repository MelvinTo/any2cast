use std::env;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use any2cast::config;
use any2cast::site::Site;

use std::sync::Mutex;

use log::info;

use std::path::PathBuf;

use actix_web::{
    get,
    middleware,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};

use actix_files::NamedFile;

use actix_web::http::{header, Method, StatusCode};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
#[clap(arg_required_else_help = true)]
struct Cli {
    #[clap(short, long)]
    server: String,

    #[clap(short, long)]
    port: u32,
}

#[get("/")]
async fn index(site: web::Data<Site<'_>>) -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(site.to_html().unwrap())
}

#[get("/p/{podcast}")]
async fn podcast(site: web::Data<Site<'_>>, podcast: web::Path<String>) -> impl Responder {
    info!("Getting rss xml for podcast {}", podcast);
    match site.get_directory(&podcast) {
        Some(dir) => {
            let rss_xml = dir.to_rss_xml().unwrap();
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
async fn media(site: web::Data<Site<'_>>, p: web::Path<(String, String)>) -> impl Responder {
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

    let link = format!("http://{}:{}", &cli.server, &cli.port);

    let cur_dir = env::current_dir()?;
    let port = cli.port as u16;

    HttpServer::new(move || {
        let mut site : Site = Site::new(
            cur_dir.to_str().unwrap().to_string(),
            &link
        ).unwrap();

        site.detect_directories().unwrap();
        site.prepare_static_files().unwrap();

        App::new()
            .app_data(web::Data::new(site))
            .service(index)
            .service(podcast)
            .service(media)
//            .route("/", web::get().to(index))
//            .route("/p/{podcast}", web::get().to(podcast))
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await?;


    Ok(())
}
