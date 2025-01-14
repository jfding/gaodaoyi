use clap::Parser;
use viuer::Config;
use anyhow::Result;
use image;
use termimad::*;
use tera::{Tera, Context};
use clearscreen;

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

    /// Whether to show Changed Hexagram
    #[arg(short, long)]
    changed: bool,

    /// Whether to show welcome picture
    #[arg(short, long)]
    silent: bool,
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

    // clean up the terminal
    clearscreen::clear()?;

    // Display the image
    viuer::print(&img, &conf)?;

    Ok(())
}


fn select_gua(prompt: &str) -> u8 {
    inquire::Select::new(prompt, vec!["1 ☰ 乾/天 (Qian/Heaven)",
                                      "2 ☱ 兌/澤 (Dui/Lake)",
                                      "3 ☲ 離/火 (Li/Fire)",
                                      "4 ☳ 震/雷 (Zhen/Thunder)",
                                      "5 ☴ 巽/風 (Xun/Wind)",
                                      "6 ☵ 坎/水 (Kan/Water)",
                                      "7 ☶ 艮/山 (Gen/Mountain)",
                                      "8 ☷ 坤/地 (Kun/Earth)"])
        .with_vim_mode(true)
        .with_page_size(8)
        .with_help_message("h/j/k/l | ←↑↓→ | <enter> | ctrl+c")
        .with_render_config(inquire::ui::RenderConfig::default().with_highlighted_option_prefix("󰚀️".into()))
        .prompt()
        .unwrap_or_else(|_e| { std::process::exit(1); })
        .split(" ")
        .next()
        .unwrap()
        .parse::<u8>()
        .expect("Failed to parse input")
}

fn select_yao(prompt: &str) -> u8 {
    inquire::Select::new(prompt, vec!["1 初爻", "2 二爻", "3 三爻", "4 四爻", "5 五爻", "6 上爻"])
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

        if !args.silent {
            welcome_pic();
        }

        let up = args.up.unwrap_or_else(|| select_gua("請選擇上卦"));
        let down = args.down.unwrap_or_else(|| select_gua("請選擇下卦"));
        let yao = args.yao.unwrap_or_else(|| select_yao("請選擇變爻"));

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