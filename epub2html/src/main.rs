use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use epub::doc::EpubDoc;
use clap::Parser;

#[derive(Parser)]
#[command(name = "epub2html")]
#[command(about = "Convert EPUB files to HTML files")]
struct Args {
    /// Input EPUB file path
    #[arg(short, long)]
    input: String,

    /// Output directory for HTML files
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Create output directory if it doesn't exist
    fs::create_dir_all(&args.output)?;
    
    // Open the EPUB document
    let mut doc = EpubDoc::new(&args.input)?;
    
    // Extract and save all the resources
    for (id, (pb, _mime)) in &doc.resources.clone() {
        let chapter_path = PathBuf::from(&args.output).join(pb);
        println!("Resource: {:?} {:?} {:?}", chapter_path, _mime, id);
        if let Some((content, _mime)) = &doc.get_resource(&id) {
            fs::write(&chapter_path, content)?;
            println!("Saved res {} to {:?}", id, chapter_path);
        }
    }
    
    println!("Conversion completed successfully!");
    Ok(())
}
