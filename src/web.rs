use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::gram::{Trigram, Hexagram};
use crate::gaodaotext::{get_gua_oracle_json, get_yao_oracle};

#[derive(Serialize, Deserialize)]
struct HexagramRequest {
    up: u8,
    down: u8,
    yao: u8,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../assets/web/index.html"))
}

async fn get_hexagram(req: web::Json<HexagramRequest>) -> impl Responder {
    let up = Trigram::from_order(req.up);
    let down = Trigram::from_order(req.down);
    let yao = req.yao;

    let hexagram = Hexagram::from_up_down(up, down);

    //let oracle = get_gua_oracle(&hexagram).unwrap();
    //let yao_oracle = get_yao_oracle(&hexagram, req.yao).unwrap();
    //let changed = hexagram.get_change(req.yao);
    //let changed_oracle = get_gua_oracle(&changed).unwrap();
    
    let raw_json = get_gua_oracle_json(&hexagram).unwrap();

    /*
    let response = serde_json::json!({
        "hexagram": {
            "unicode": hexagram.unicode,
            "name": hexagram.long_name,
            "order": hexagram.order,
            "oracle": oracle
        },
        "yao": {
            "position": req.yao,
            "oracle": yao_oracle
        },
        "changed": {
            "unicode": changed.unicode, 
            "name": changed.long_name,
            "order": changed.order,
            "oracle": changed_oracle
        }
    });
    */

    HttpResponse::Ok().json(serde_json::from_str::<serde_json::Value>(&raw_json).unwrap())
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/hexagram", web::post().to(get_hexagram))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
