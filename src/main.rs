use clap::Parser;
use viuer::Config;
use anyhow::Result;
use image;
use termimad::*;
use tera::{Tera, Context};

mod gram;
use gram::*;

mod gaodaotext;
use gaodaotext::*;

// Embed the image data directly into the binary
const ICON_DATA: &[u8] = include_bytes!("../assets/images/book-cover.jpg");
#[derive(Default, Debug)]
struct Keys {
    up: Trigram,
    down: Trigram,
    yao: u8,
}

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Up "gua" number: 1-8
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=8))]
    up: Option<u8>,

    /// Down "gua" number: 1-8
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=8))]
    down: Option<u8>,

    /// Number of critical "yao" number: 1-6
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=6))]
    yao: Option<u8>,
}

fn welcome_pic() -> Result<()> {
    // Load the embedded image data
    let img = image::load_from_memory(ICON_DATA)?;

    // Configure how to display the image
    let conf = Config {
        // Automatically fit to terminal width while maintaining aspect ratio
        width: Some(80),  // you can adjust this value
        height: None,     // will be calculated automatically
        // Set to true if your terminal has a dark background
        transparent: true,
        // Other options available:
        // absolute_offset: false,
        // x: 0,
        // y: 0,
        ..Default::default()
    };

    // Display the image
    viuer::print(&img, &conf)?;

    Ok(())
}


fn select_gua(prompt: &str) -> u8 {
    inquire::Select::new(prompt, vec!["1 ☰ Qian/Heaven (乾/天)",
                                      "2 ☱ Dui/Lake (兌/澤)",
                                      "3 ☲ Li/Fire (離/火)",
                                      "4 ☳ Zhen/Thunder (震/雷)",
                                      "5 ☴ Xun/Wind (巽/風)",
                                      "6 ☵ Kan/Water (坎/水)",
                                      "7 ☶ Gen/Mountain (艮/山)",
                                      "8 ☷ Kun/Earth (坤/地)"])
        .with_vim_mode(true)
        .with_page_size(8)
        .with_help_message("h/j/k/l | ←↑↓→ | <enter> | ctrl+c")
        .with_render_config(inquire::ui::RenderConfig::default().with_highlighted_option_prefix("☯️".into()))
        .prompt()
        .unwrap_or_else(|_e| { std::process::exit(1); })
        .split(" ")
        .next()
        .unwrap()
        .parse::<u8>()
        .expect("Failed to parse input")
}

fn select_yao() -> u8 {
    inquire::Select::new("Select YAO number", vec!["1 初", "2 二", "3 三", "4 四", "5 五", "6 上"])
        .with_vim_mode(true)
        .with_page_size(6)
        .with_help_message("h/j/k/l | ←↑↓→ | <enter> | ctrl+c")
        .with_render_config(inquire::ui::RenderConfig::default().with_highlighted_option_prefix("󰚀".into()))
        .prompt()
        .unwrap_or_else(|_e| { std::process::exit(1); })
        .split(" ")
        .next()
        .unwrap()
        .parse::<u8>()
        .expect("Failed to parse input")
}

fn main() -> Result<()> {
    //welcome_pic();

    let mut keys = Keys::default();

    // peek the cli args before parsing them
    let ori_args :Vec<_> = std::env::args().collect();

    if ori_args.len() == 4 && ori_args.iter().skip(1).all(|arg| arg.parse::<u8>().is_ok()) {
        // All arguments are valid numbers
        let numbers :Vec<_> = ori_args.iter().skip(1).map(|arg| arg.parse::<u8>().unwrap()).collect();
        if numbers[0] > 8 || numbers[1] > 8 || numbers[2] > 6 {
            println!("Invalid input");
            return Ok(());
        }

        keys = Keys { up: Trigram::from_order(numbers[0]),
                      down: Trigram::from_order(numbers[1]),
                      yao: numbers[2] };
    } else {
        let args = Args::parse();

        let up = args.up.unwrap_or_else(|| select_gua("Select up GUA"));
        let down = args.down.unwrap_or_else(|| select_gua("Select down GUA"));
        let yao = args.yao.unwrap_or_else(|| select_yao());

        keys = Keys { up: Trigram::from_order(up),
                      down: Trigram::from_order(down),
                      yao };
    }

    let hexagram = Hexagram::from_up_down(keys.up, keys.down);
    let ho = get_gua_oracle(hexagram.order)?;
    
    // Create a markdown string with your content
    let mut tmpl = Tera::default();
    tmpl.add_raw_template("OracleGua", ORACLE_GUA_TEMPLATE)?;
    tmpl.add_raw_template("OracleYao", ORACLE_YAO_TEMPLATE)?;

    let mut ctx = Context::new();
    ctx.insert("unicode", &hexagram.unicode.to_string());
    ctx.insert("long_name", &hexagram.long_name);
    ctx.insert("order", &hexagram.order.to_string());
    ctx.insert("summary", &ho.summary);
    ctx.insert("guaci", &ho.guaci);
    ctx.insert("guaci_explain", &ho.guaci_explain);
    ctx.insert("tuan", &ho.tuan);
    ctx.insert("tuan_explain", &ho.tuan_explain);
    ctx.insert("daxiang", &ho.daxiang);
    ctx.insert("daxiang_explain", &ho.daxiang_explain); 
    ctx.insert("guazhan", &ho.guazhan);

    let md_gua = tmpl.render("OracleGua", &ctx)?;


    let yao = &ho.yaos[keys.yao as usize - 1];
    ctx.insert("yaoci", &yao.yaoci);
    ctx.insert("xiaoxiang", &yao.xiaoxiang);
    ctx.insert("yaoci_explain", &yao.yaoci_explain);
    ctx.insert("yaozhan", &yao.yaozhan);
    ctx.insert("cases", &yao.cases);

    let md_yao = tmpl.render("OracleYao", &ctx)?;
    
    // Print the formatted markdown
    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(rgb(255, 187, 0));
    skin.italic.set_fg(rgb(215, 255, 135));
    skin.bullet = StyledChar::from_fg_char(rgb(255, 187, 0), '•');


    skin.print_text(&md_gua);
    skin.print_text(&md_yao);

    Ok(())
}

