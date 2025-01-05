use clap::Parser;
use viuer::Config;
use image;

// Embed the image data directly into the binary
const ICON_DATA: &[u8] = include_bytes!("../assets/images/icon.png");

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the embedded image data
    let img = image::load_from_memory(ICON_DATA)?;
    println!("Image dimensions: {}x{}", img.width(), img.height());

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

    let args = Args::parse();

    println!("up {:?}!", args.up);
    println!("down {:?}!", args.down);
    println!("yao {:?}!", args.yao);

    let up = args.up.unwrap_or_else(|| {
        inquire::Select::new("Select up gua", vec!["1", "2", "3", "4", "5", "6", "7", "8"])
            .prompt()
            .expect("Failed to get user input")
            .parse::<u8>()
            .expect("Failed to parse input")
    });
    println!("up: {}", up);

    let down = args.down.unwrap_or_else(|| {
        inquire::Select::new("Select down gua", vec!["1", "2", "3", "4", "5", "6", "7", "8"])
            .prompt()
            .expect("Failed to get user input")
            .parse::<u8>()
            .expect("Failed to parse input")
    });
    println!("down: {}", down);

    let yao = args.yao.unwrap_or_else(|| {
        inquire::Select::new("Select yao number", vec!["1", "2", "3", "4", "5", "6"])
            .prompt()
            .expect("Failed to get user input")
            .parse::<u8>()
            .expect("Failed to parse input")
    });
    println!("yao: {}", yao);

    Ok(())
}
