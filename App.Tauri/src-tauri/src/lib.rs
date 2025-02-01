// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde_json::json;
use markdown;

mod gaodaotext;
mod gram;

#[tauri::command]
fn get_guaci(up: &str, down: &str) -> String {
    let gram_up = gram::Trigram::from_order(up.parse::<u8>().unwrap_or(1));
    let gram_down = gram::Trigram::from_order(down.parse::<u8>().unwrap_or(1));

    let hexagram = gram::Hexagram::from_up_down(gram_up, gram_down);

    let text = gaodaotext::get_gua_oracle_md(&hexagram).unwrap();
    // convert markdown to html
    let html = markdown::to_html(&text);

    json!({
        "html": html,
        "unicode": hexagram.unicode,
        "name": hexagram.long_name,
        "order": hexagram.order,
    }).to_string()
}

#[tauri::command]
fn get_guaci_alt(up: &str, down: &str, yao: &str) -> String {
    let gram_up = gram::Trigram::from_order(up.parse::<u8>().unwrap_or(1));
    let gram_down = gram::Trigram::from_order(down.parse::<u8>().unwrap_or(1));
    let yao = yao.parse::<u8>().unwrap_or(1);

    let hexagram = gram::Hexagram::from_up_down(gram_up, gram_down).get_change(yao);

    let text = gaodaotext::get_gua_oracle_md(&hexagram).unwrap();
    // convert markdown to html
    let html = markdown::to_html(&text);

    json!({
        "html": html,
        "unicode": hexagram.unicode,
        "name": hexagram.long_name,
        "order": hexagram.order,
    }).to_string()
}

#[tauri::command]
fn get_yaoci(up: &str, down: &str, yao: &str) -> String {
    let gram_up = gram::Trigram::from_order(up.parse::<u8>().unwrap_or(1));
    let gram_down = gram::Trigram::from_order(down.parse::<u8>().unwrap_or(1));

    let hexagram = gram::Hexagram::from_up_down(gram_up, gram_down);

    let yao = yao.parse::<u8>().unwrap_or(1);

    let text = gaodaotext::get_yao_oracle_md(&hexagram, yao).unwrap();
    // convert markdown to html
    let html = markdown::to_html(&text);

    json!({
        "html": html,
    }).to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_guaci, get_yaoci, get_guaci_alt])
        .setup(|app| {
            let webview_url = tauri::WebviewUrl::App("index.html".into());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
