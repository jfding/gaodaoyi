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

async fn static_index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../assets/web/index.html"))
}
async fn static_logo() -> impl Responder {
    HttpResponse::Ok().body(include_bytes!("../assets/images/book-cover.jpg").to_vec())
}
async fn static_styles() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../assets/web/styles.css"))
}
async fn static_scripts() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../assets/web/scripts.js"))
}

async fn get_hexagram_gua(req: web::Json<HexagramRequest>) -> impl Responder {
    let up = Trigram::from_order(req.up);
    let down = Trigram::from_order(req.down);
    let hexagram = Hexagram::from_up_down(&up, &down);

    let html = markdown::to_html(&get_gua_oracle_md(&hexagram).unwrap());

    let glyphs = get_gua_glyphs(&hexagram);
    let mut gl_html = String::new();
    for index in 0..glyphs.len() {
        let (name, _) = glyphs[index];
        if ! name.is_empty() {
            gl_html += &format!("<span>{}</span><img src='/glyphs/{}/{}/{}' alt='{}' />",
                name.to_owned()+":",
                &hexagram.up.order,
                &hexagram.down.order,
                index,
                name);
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "html": html,
        "unicode": hexagram.unicode,
        "name": hexagram.long_name,
        "order": hexagram.order,
        "glyphs": gl_html,
    }))
}

async fn get_hexagram_gua_alt(req: web::Json<HexagramRequest>) -> impl Responder {
    let up = Trigram::from_order(req.up);
    let down = Trigram::from_order(req.down);
    let yao = req.yao;
    let hexagram = Hexagram::from_up_down(&up, &down).get_change(yao);

    let html = markdown::to_html(&get_gua_oracle_md(&hexagram).unwrap());

    let glyphs = get_gua_glyphs(&hexagram);
    let mut gl_html = String::new();
    for index in 0..glyphs.len() {
        let (name, _) = glyphs[index];
        if ! name.is_empty() {
            gl_html += &format!("<span>{}</span><img src='/glyphs/{}/{}/{}' alt='{}' />",
                name.to_owned()+":",
                &hexagram.up.order,
                &hexagram.down.order,
                index,
                name);
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "html": html,
        "unicode": hexagram.unicode,
        "name": hexagram.long_name,
        "order": hexagram.order,
        "glyphs": gl_html,
    }))
}

async fn get_hexagram_yao(req: web::Json<HexagramRequest>) -> impl Responder {
    let up = Trigram::from_order(req.up);
    let down = Trigram::from_order(req.down);
    let yao = req.yao;

    let hexagram = Hexagram::from_up_down(&up, &down);

    let html = markdown::to_html(&get_yao_oracle_md(&hexagram, yao).unwrap());

    HttpResponse::Ok().json(serde_json::json!({
        "html": html,
    }))
}

async fn get_gua_glyph_image(path: web::Path<(u8, u8, u8)>) -> impl Responder {
    let up = Trigram::from_order(path.0);
    let down = Trigram::from_order(path.1);
    let index = path.2;
    let hexagram = Hexagram::from_up_down(&up, &down);

    let (_, glyph_stream) = get_gua_glyphs(& hexagram.clone())[index as usize];

    if glyph_stream.is_empty() {
        return HttpResponse::NotFound().body("Glyph not found");
    } else {
        return HttpResponse::Ok().append_header(("Content-Type", "image/jpeg")).body(glyph_stream);
    }
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(static_index))
            .route("/logo.jpg", web::get().to(static_logo))
            .route("/styles.css", web::get().to(static_styles))
            .route("/scripts.js", web::get().to(static_scripts))
            .route("/glyphs/{up}/{down}/{index}", web::get().to(get_gua_glyph_image))
            .route("/hexagram_gua", web::post().to(get_hexagram_gua))
            .route("/hexagram_yao", web::post().to(get_hexagram_yao))
            .route("/hexagram_gua_alt", web::post().to(get_hexagram_gua_alt))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
