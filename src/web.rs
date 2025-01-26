use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::gram::{Trigram, Hexagram};
use crate::gaodaotext::*;

#[derive(Serialize, Deserialize)]
struct HexagramRequest {
    up: u8,
    down: u8,
    yao: u8,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../assets/web/index.html"))
}

async fn logo() -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(include_bytes!("../assets/images/book-cover.jpg").to_vec())
}

async fn get_hexagram_gua(req: web::Json<HexagramRequest>) -> impl Responder {
    let up = Trigram::from_order(req.up);
    let down = Trigram::from_order(req.down);

    let hexagram = Hexagram::from_up_down(up, down);

    let html = markdown::to_html(&get_gua_oracle_md(&hexagram).unwrap());

    HttpResponse::Ok().json(serde_json::json!({
        "html": html,
        "unicode": hexagram.unicode,
        "name": hexagram.long_name,
        "order": hexagram.order,
    }))
}

async fn get_hexagram_gua_alt(req: web::Json<HexagramRequest>) -> impl Responder {
    let up = Trigram::from_order(req.up);
    let down = Trigram::from_order(req.down);
    let yao = req.yao;

    let hexagram = Hexagram::from_up_down(up, down).get_change(yao);

    let html = markdown::to_html(&get_gua_oracle_md(&hexagram).unwrap());

    HttpResponse::Ok().json(serde_json::json!({
        "html": html,
        "unicode": hexagram.unicode,
        "name": hexagram.long_name,
        "order": hexagram.order,
    }))
}
async fn get_hexagram_yao(req: web::Json<HexagramRequest>) -> impl Responder {
    let up = Trigram::from_order(req.up);
    let down = Trigram::from_order(req.down);
    let yao = req.yao;

    let hexagram = Hexagram::from_up_down(up, down);

    let html = markdown::to_html(&get_yao_oracle_md(&hexagram, yao).unwrap());

    HttpResponse::Ok().json(serde_json::json!({
        "html": html,
    }))
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/logo.jpg", web::get().to(logo))
            .route("/hexagram_gua", web::post().to(get_hexagram_gua))
            .route("/hexagram_yao", web::post().to(get_hexagram_yao))
            .route("/hexagram_gua_alt", web::post().to(get_hexagram_gua_alt))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
